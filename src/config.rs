//! Handles configuration in the `Cargo.toml` `[workspace.metadata]` and `[package.metadata]` sections

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub base_url: String,
    pub note: String,
}

const DEFAULT_CONFIG_STR: &str = include_str!("../default_config.toml");

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
        }
    }
}

/// This is the `[package.metadata.cargo-reedme]` and `[workspace.package.metadata.cargo-reedme]`
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
struct ConfigToml {
    base_url: Option<String>,
    note: Option<String>,
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
    }
}
