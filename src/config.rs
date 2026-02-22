//! Handles configuration in the `Cargo.toml` `[workspace.metadata]` and `[package.metadata]` sections

use serde::{Deserialize, Serialize};

/// This is the `[package.metadata.cargo-reedme]` and `[workspace.package.metadata.cargo-reedme]`
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub docs_rs: IntralinksDocsRsConfig,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IntralinksDocsRsConfig {
    pub base_url: Option<String>,
}

impl Config {
    /// Extracts configuration from the `[workspace.metadata]` or `[package.metadata]` sections in `Cargo.toml`
    pub fn from_cargo_metadata(metadata: serde_json::Value) -> Self {
        #[derive(Serialize, Deserialize, Default)]
        #[serde(rename_all = "kebab-case")]
        struct PackageMetadata {
            cargo_reedme: Option<Config>,
        }

        serde_json::from_value::<Option<PackageMetadata>>(metadata.clone())
            .unwrap_or_default()
            .unwrap_or_default()
            .cargo_reedme
            .unwrap_or_default()
    }

    /// Merges contents of `[package.metadata]` with `[workspace.metadata]`,
    /// package metadata takes priority
    pub fn inherit_workspace_metadata(&mut self, workspace_config: Self) {
        if let Some(base_url) = workspace_config.docs_rs.base_url {
            self.docs_rs.base_url = Some(base_url);
        }
    }
}
