//! The [`World`] is the single input that the program receives,
//! essentially the program is almost entirely a pure function

use std::env;

use docstr::docstr;
use eyre::Context as _;
use eyre::ContextCompat as _;
use eyre::Result;
use fs_err as fs;
use itertools::Itertools;

use crate::config::Config;

/// Represents all necessary inputs to the program
#[allow(clippy::type_complexity)]
pub struct World {
    pub input_manifest: clap_cargo::Manifest,
    pub input_workspace: clap_cargo::Workspace,
    pub input_features: clap_cargo::Features,
    /// Get rustdoc crate information about the given package
    pub rustdoc_json_for_crate: Box<
        dyn Fn(
                &cargo_metadata::Package,
                &cargo_metadata::Metadata,
                &Config,
                &str,
            ) -> Result<rustdoc_types::Crate>
            + Sync,
    >,
    pub rustdoc_html_for_crate: fn(package_name: &str, toolchain: &str) -> Result<String>,
    /// Read the given file to a string
    pub read_file: fn(&camino::Utf8Path) -> std::io::Result<String>,
    /// Rust Toolchain
    pub toolchain: String,
    pub args: Vec<String>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            input_manifest: Default::default(),
            input_workspace: Default::default(),
            input_features: Default::default(),
            rustdoc_json_for_crate: Box::new(move |pkg, metadata, config, toolchain| {
                extract_rustdoc_json(pkg, metadata, toolchain, config)
            }),
            rustdoc_html_for_crate: |package_name, toolchain| {
                let output = std::process::Command::new(
                    env::var("CARGO").unwrap_or_else(|_| "cargo".into()),
                )
                .env("RUST_TOOLCHAIN", toolchain)
                .arg("rustdoc")
                .arg("--package")
                .arg(package_name)
                .output()
                .context("rustdoc error")?;

                let mut path = String::from_utf8(output.stderr)
                    .context("non-utf8 output")?
                    .strip_suffix("\n")
                    .context("invalid format")?
                    .chars()
                    .rev()
                    .take_while(|ch| !ch.is_whitespace())
                    .collect::<Vec<_>>();
                path.reverse();
                let path = path.into_iter().collect::<String>();

                let path = std::path::PathBuf::from(path);

                Ok(fs::read_to_string(path)?)
            },
            read_file: |path| fs::read_to_string(path),
            toolchain: std::env::var("RUST_TOOLCHAIN").unwrap_or("nightly".to_string()),
            args: std::env::args().skip(2).collect(),
        }
    }
}

/// Run `rustdoc` on the package and returns all available information
pub fn extract_rustdoc_json(
    pkg: &cargo_metadata::Package,
    metadata: &cargo_metadata::Metadata,
    toolchain: &str,
    config: &Config,
) -> Result<rustdoc_types::Crate> {
    let node = metadata
        .resolve
        .as_ref()
        .context("cargo failed resolution")?
        .nodes
        .iter()
        .find(|node| node.id == pkg.id)
        .context("node ID does not exist")?;

    let rustdoc_json_for_target = |target: &cargo_metadata::Target| {
        let builder = rustdoc_json::Builder::default()
            .toolchain(toolchain)
            .manifest_path(&pkg.manifest_path)
            .env(
                "RUSTFLAGS",
                format!(
                    "{} {}",
                    env::var("RUSTFLAGS").unwrap_or_default(),
                    config.rustc_args.join(" ")
                ),
            )
            .env(
                "RUSTDOCFLAGS",
                format!(
                    "{} {}",
                    env::var("RUSTDOCFLAGS").unwrap_or_default(),
                    config.rustdoc_args.join(" ")
                ),
            )
            .document_private_items(true)
            .no_default_features(true)
            .all_features(false)
            // NOTE: this already includes information about passed CLI arguments + config
            // (e.g. --no-default-features, --features, --all-features etc) so we disable those^^
            .features(
                node.features
                    .iter()
                    .map(|feature| format!("{}/{feature}", pkg.name)),
            )
            .package(&pkg.name)
            .package_target(convert_package_target(target));
        let rustdoc_json_path = builder.build().context("rustdoc error")?;

        let rustdoc_json =
            fs::read(rustdoc_json_path).context("failed to open rustdoc json file")?;
        let mut rustdoc_json = std::io::Cursor::new(rustdoc_json);

        serde_json::from_reader::<_, rustdoc_types::Crate>(&mut rustdoc_json).with_context(|| {
            docstr!(format!
                /// failed to deserialize rustdoc json
                ///
                /// this usually happens because rustdoc's JSON output is unstable and frequently changes
                ///
                /// the Rust version that `cargo reedme` was invoked with may be out of sync with
                /// the version of the `rustdoc_types` crate that `cargo reedme` uses, which is `{0}`
                ///
                /// You can usually fix this by invoking `cargo reedme` with a Rust version that has
                /// rustdoc version `{0}`. For example, try `cargo +nightly reedme` or another version: `cargo +nightly-YYYY-MM-DD reedme`
                rustdoc_types::FORMAT_VERSION
            )
        })
    };

    let target = config.target.select(&pkg.targets, |target| {
        let krate = rustdoc_json_for_target(target).unwrap();

        let root = krate
            .index
            .get(&krate.root)
            .expect("rustdoc's root item is a valid item");

        root.docs.as_ref().is_none_or(|docs| docs.trim().is_empty())
    })?;

    rustdoc_json_for_target(target)
}

pub fn extract_package_target(
    pkg: &cargo_metadata::Package,
) -> Result<rustdoc_json::PackageTarget> {
    let target = pkg.targets.first().context("no cargo target")?;
    let package_target = convert_package_target(target);
    Ok(package_target)
}

fn convert_package_target(target: &cargo_metadata::Target) -> rustdoc_json::PackageTarget {
    if target.is_kind(cargo_metadata::TargetKind::Bin) {
        rustdoc_json::PackageTarget::Bin(target.name.clone())
    } else {
        rustdoc_json::PackageTarget::Lib
    }
}
