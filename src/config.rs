//! Handles configuration in the `Cargo.toml` `[workspace.metadata]` and `[package.metadata]` sections

use core::fmt;
use std::{str::FromStr, sync::LazyLock};

use eyre::{ContextCompat, Result, bail};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub base_url: String,
    pub note: String,
    pub target: Target,
    pub increment_headings: bool,
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
        is_target_empty: impl Fn(&'a cargo_metadata::Target) -> bool,
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
            Target::Lib => get_library_target()
                .ok()
                .wrap_err("expected a library target (lib.rs)"),
            Target::Heuristic => {
                if let Ok(target) = get_library_target()
                    && !is_target_empty(target)
                {
                    Ok(target)
                } else if let Some(target) = targets.iter().find(|target| target.is_bin()) {
                    Ok(target)
                } else {
                    bail!(
                        "no binary target (main.rs) or library target (lib.rs) found with documentation comments"
                    )
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
        if let Some(base_url) = workspace_config.base_url {
            self.base_url = Some(base_url);
        }
        if let Some(note) = workspace_config.note {
            self.note = Some(note);
        }
        if let Some(target) = workspace_config.target {
            self.target = Some(target);
        }
        if let Some(target) = workspace_config.increment_headings {
            self.increment_headings = Some(target);
        }
    }
}
