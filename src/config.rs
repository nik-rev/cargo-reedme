//! Handles configuration in the `Cargo.toml` `[workspace.metadata]` and `[package.metadata]` sections

use core::fmt;
use std::{str::FromStr, sync::LazyLock};

use eyre::{ContextCompat, OptionExt, Result, bail};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use subdef::subdef;

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub base_url: String,
    pub note: String,
    pub target: Target,
    pub increment_headings: bool,
    pub features: Option<Vec<String>>,
    pub all_features: Option<bool>,
    pub no_default_features: Option<bool>,
    #[serde(default)]
    pub rustc_args: Vec<String>,
    #[serde(default)]
    pub rustdoc_args: Vec<String>,
}

impl Config {
    /// Whether this config affects the Cargo metadata,
    /// in which case it will need to be re-computed
    pub fn affects_cargo_metadata(&self) -> bool {
        self.all_features.is_some() || self.features.is_some() || self.no_default_features.is_some()
    }
}

#[derive(
    Default, Clone, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, PartialEq, Eq,
)]
pub enum Target {
    BinOnly,
    BinExact(String),
    Lib,
    #[default]
    Heuristic,
}

impl Target {
    pub fn select<'a>(
        &self,
        targets: &'a [cargo_metadata::Target],
        is_target_empty: impl Fn(&'a cargo_metadata::Target) -> Result<bool>,
    ) -> Result<&'a cargo_metadata::Target> {
        let get_library_target = || {
            targets
                .iter()
                .filter(|target| {
                    target.is_lib()
                        || target.is_rlib()
                        || target.is_cdylib()
                        || target.is_proc_macro()
                        || target.is_staticlib()
                })
                .exactly_one()
                .ok()
        };

        match self {
            Target::BinOnly => targets
                .iter()
                .filter(|target| target.is_bin())
                .exactly_one()
                .ok()
                .wrap_err("expected exactly 1 binary target (main.rs)"),
            Target::BinExact(name) => targets
                .iter()
                .filter(|target| target.is_bin())
                .find(|target| target.name == *name)
                .wrap_err_with(|| format!("binary target with the name `{name}` not found")),
            Target::Lib => get_library_target().wrap_err("expected a library target (lib.rs)"),
            Target::Heuristic => {
                let lib_target = get_library_target();

                if let Some(target) = lib_target
                    && !is_target_empty(target)?
                {
                    // lib.rs but it's non-empty
                    Ok(target)
                } else if let Some(target) = targets.iter().find(|target| target.is_bin()) {
                    // main.rs
                    Ok(target)
                } else {
                    // lib.rs but it's empty
                    lib_target.ok_or_eyre("no library (lib.rs) or binary (main.rs) target found")
                }
            }
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::BinOnly => f.write_str("bin"),
            Target::BinExact(name) => f.write_fmt(format_args!("bin:{name}")),
            Target::Lib => f.write_str("lib"),
            Target::Heuristic => f.write_str("heuristic"),
        }
    }
}

impl FromStr for Target {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lib" => Ok(Self::Lib),
            "bin" => Ok(Self::BinOnly),
            "heuristic" => Ok(Self::Heuristic),
            s => {
                if let Some(("bin", bin_name)) = s.split_once(':') {
                    return Ok(Self::BinExact(bin_name.to_string()));
                };

                bail!(
                    r#"valid values for `target` are: "lib", "bin", "bin:bin-name", or "heuristic""#
                )
            }
        }
    }
}

pub const DEFAULT_CONFIG_STR: &str = include_str!("default_config.toml");

static DEFAULT_CONFIG: LazyLock<Config> =
    LazyLock::new(|| toml::from_str(DEFAULT_CONFIG_STR).unwrap());

impl Config {
    pub fn new(workspace_metadata: serde_json::Value, package_metadata: serde_json::Value) -> Self {
        let workspace_config = ConfigToml::from_cargo_metadata(workspace_metadata);
        // Read configuration as specified in [package.metadata.cargo-reedme]
        let mut config = ConfigToml::from_cargo_metadata(package_metadata);
        // Inherit values from [workspace.metadata.cargo-reedme]
        config.inherit_workspace_metadata(workspace_config);

        Self {
            base_url: config
                .base_url
                .unwrap_or_else(|| DEFAULT_CONFIG.base_url.clone()),
            note: config.note.unwrap_or_else(|| DEFAULT_CONFIG.note.clone()),
            target: config
                .target
                .unwrap_or_else(|| DEFAULT_CONFIG.target.clone()),
            increment_headings: config
                .increment_headings
                .unwrap_or_else(|| DEFAULT_CONFIG.increment_headings),
            features: config.cargo.features,
            all_features: config.cargo.all_features,
            no_default_features: config.cargo.no_default_features,
            rustc_args: config
                .cargo
                .rustc_args
                .unwrap_or_else(|| DEFAULT_CONFIG.rustc_args.clone()),
            rustdoc_args: config
                .cargo
                .rustdoc_args
                .unwrap_or_else(|| DEFAULT_CONFIG.rustdoc_args.clone()),
        }
    }
}

/// This is the `[package.metadata.cargo-reedme]` and `[workspace.package.metadata.cargo-reedme]`
#[subdef(
    derive(Serialize, Deserialize, Default, Clone),
    serde(rename_all = "kebab-case")
)]
// TODO: this fails to compile, requires changes to `subdef`
//
// #[serde(deny_unknown_fields)]
struct ConfigToml {
    base_url: Option<String>,
    note: Option<String>,
    target: Option<Target>,
    increment_headings: Option<bool>,
    // we want to re-use this for `[metadata.docs.rs]`
    #[serde(flatten)]
    cargo: [_; {
        struct ConfigTomlRustdocShared {
            features: Option<Vec<String>>,
            all_features: Option<bool>,
            no_default_features: Option<bool>,
            rustc_args: Option<Vec<String>>,
            rustdoc_args: Option<Vec<String>>,
        }
    }],
}

impl ConfigToml {
    /// Extracts configuration from the `[workspace.metadata]` or `[package.metadata]` sections in `Cargo.toml`
    fn from_cargo_metadata(metadata: serde_json::Value) -> Self {
        #[subdef(
            derive(Serialize, Deserialize, Default),
            serde(rename_all = "kebab-case")
        )]
        struct PackageMetadata {
            cargo_reedme: Option<ConfigToml>,
            docs: [Option<_>; {
                struct DocsRs {
                    rs: Option<ConfigTomlRustdocShared>,
                }
            }],
        }

        let this = serde_json::from_value::<Option<PackageMetadata>>(metadata.clone())
            .unwrap_or_default()
            .unwrap_or_default();

        let mut config = this.cargo_reedme.unwrap_or_default();

        let docs_rs_config = this.docs.unwrap_or_default().rs.unwrap_or_default();

        // forward config from docs.rs config if it makes sense to do so

        macro_rules! forward {
            ($field:ident) => {
                if config.cargo.$field.is_none()
                    && let Some($field) = docs_rs_config.$field
                {
                    config.cargo.$field = Some($field);
                }
            };
        }

        forward!(features);
        forward!(all_features);
        forward!(no_default_features);
        forward!(rustc_args);
        forward!(rustdoc_args);

        config
    }

    /// Merges contents of `[package.metadata]` with `[workspace.metadata]`,
    /// package metadata takes priority
    fn inherit_workspace_metadata(&mut self, workspace_config: Self) {
        if self.base_url.is_none() {
            self.base_url = workspace_config.base_url;
        }
        if self.note.is_none() {
            self.note = workspace_config.note;
        }
        if self.target.is_none() {
            self.target = workspace_config.target;
        }
        if self.increment_headings.is_none() {
            self.increment_headings = workspace_config.increment_headings;
        }

        if self.cargo.features.is_none() {
            self.cargo.features = workspace_config.cargo.features;
        }
        self.cargo.all_features = self
            .cargo
            .all_features
            .or(workspace_config.cargo.all_features);
        self.cargo.no_default_features = self
            .cargo
            .no_default_features
            .or(workspace_config.cargo.no_default_features);

        if self.cargo.rustc_args.is_none() {
            self.cargo.rustc_args = workspace_config.cargo.rustc_args;
        }
        if self.cargo.rustdoc_args.is_none() {
            self.cargo.rustdoc_args = workspace_config.cargo.rustdoc_args;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    use docstr::docstr;
    use serde_json::Value;

    struct Case {
        docs_rs: &'static str,
        package: &'static str,
        workspace: Option<&'static str>,
        expected: &'static str,
    }

    fn test(case: Case) {
        let make_meta = |reedme: &str, docs: &str| -> Value {
            let toml = docstr!(format!
                /// [cargo-reedme]
                /// {reedme}
                ///
                /// [docs.rs]
                /// {docs}
            );
            let v: toml::Value =
                toml::from_str(&toml).unwrap_or(toml::Value::Table(Default::default()));
            serde_json::to_value(v).unwrap()
        };

        let workspace_meta = case
            .workspace
            .map(|w| make_meta(w, ""))
            .unwrap_or_else(|| serde_json::json!({}));

        let package_meta = make_meta(case.package, case.docs_rs);

        let expected_config: Config = {
            let mut config: toml::Table = toml::from_str(DEFAULT_CONFIG_STR).unwrap();
            let expected_config_override: toml::Table = toml::from_str(case.expected).unwrap();

            for (k, v) in expected_config_override {
                config.insert(k, v);
            }

            toml::from_str(&toml::to_string(&config).unwrap()).unwrap()
        };

        let actual_config = Config::new(workspace_meta, package_meta);

        assert!(expected_config == actual_config);
    }

    #[test]
    fn workspace_inheritance() {
        test(Case {
            docs_rs: "",
            package: "note = 'Package Note'",
            workspace: Some("base-url = 'https://workspace.io'"),
            expected: docstr!(
                /// base-url = "https://workspace.io"
                /// note = "Package Note"
            ),
        });
    }

    #[test]
    fn docs_rs_fallback() {
        test(Case {
            docs_rs: "rustdoc-args = ['--cfg', 'docsrs']",
            package: "base-url = 'https://fixed.com'",
            workspace: None,
            expected: docstr!(
                /// base-url = "https://fixed.com"
                /// rustdoc-args = ["--cfg", "docsrs"]
            ),
        });
    }

    #[test]
    fn package_priority_over_docs_rs() {
        test(Case {
            docs_rs: "all-features = true",
            package: "all-features = false",
            workspace: None,
            expected: docstr!(
                /// all-features = false
            ),
        });
    }

    #[test]
    fn docs_rs_priority_over_workspace() {
        test(Case {
            docs_rs: "rustc-args = ['--target-cpu=native']",
            package: "",
            workspace: Some("rustc-args = ['--generic']"),
            expected: docstr!(
                /// rustc-args = ["--target-cpu=native"]
            ),
        });
    }

    #[test]
    fn target_enum_deserialization() {
        test(Case {
            docs_rs: "",
            package: "target = 'bin:server-app'",
            workspace: None,
            expected: docstr!(
                /// target = "bin:server-app"
            ),
        });
    }
}
