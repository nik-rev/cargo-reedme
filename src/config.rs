//! Handles configuration in the `Cargo.toml` `[workspace.metadata]` and `[package.metadata]` sections

use core::fmt;
use std::{str::FromStr, sync::LazyLock};

use eyre::{ContextCompat, OptionExt, Result, bail};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub base_url: String,
    pub note: String,
    pub target: Target,
    pub increment_headings: bool,
    pub features: Option<Vec<String>>,
    pub all_features: Option<bool>,
    pub no_default_features: Option<bool>,
    pub rustc_args: Vec<String>,
    pub rustdoc_args: Vec<String>,
}

impl Config {
    /// Whether this config affects the Cargo metadata,
    /// in which case it will need to be re-computed
    pub fn affects_cargo_metadata(&self) -> bool {
        self.all_features.is_some() || self.features.is_some() || self.no_default_features.is_some()
    }
}

#[derive(Default, Clone, serde_with::SerializeDisplay, serde_with::DeserializeFromStr)]
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
            features: config.features,
            all_features: config.all_features,
            no_default_features: config.no_default_features,
            rustc_args: config
                .rustc_args
                .unwrap_or_else(|| DEFAULT_CONFIG.rustc_args.clone()),
            rustdoc_args: config
                .rustdoc_args
                .unwrap_or_else(|| DEFAULT_CONFIG.rustdoc_args.clone()),
        }
    }
}

/// This is the `[package.metadata.cargo-reedme]` and `[workspace.package.metadata.cargo-reedme]`
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
struct ConfigToml {
    base_url: Option<String>,
    note: Option<String>,
    target: Option<Target>,
    increment_headings: Option<bool>,
    features: Option<Vec<String>>,
    all_features: Option<bool>,
    no_default_features: Option<bool>,
    rustc_args: Option<Vec<String>>,
    rustdoc_args: Option<Vec<String>>,
}

impl ConfigToml {
    /// Extracts configuration from the `[workspace.metadata]` or `[package.metadata]` sections in `Cargo.toml`
    fn from_cargo_metadata(metadata: serde_json::Value) -> Self {
        #[derive(Serialize, Deserialize, Default)]
        #[serde(rename_all = "kebab-case")]
        struct PackageMetadata {
            cargo_reedme: Option<ConfigToml>,
        }

        serde_json::from_value::<Option<PackageMetadata>>(metadata.clone())
            .unwrap_or_default()
            .unwrap_or_default()
            .cargo_reedme
            .unwrap_or_default()
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

        if self.features.is_none() {
            self.features = workspace_config.features;
        }
        self.all_features = self.all_features.or(workspace_config.all_features);
        self.no_default_features = self
            .no_default_features
            .or(workspace_config.no_default_features);

        if self.rustc_args.is_none() {
            self.rustc_args = workspace_config.rustc_args;
        }
        if self.rustdoc_args.is_none() {
            self.rustdoc_args = workspace_config.rustdoc_args;
        }
    }
}
