//! The [`World`] is the single input that the program receives,
//! essentially the program is almost entirely a pure function

use docstr::docstr;
use eyre::Context as _;
use eyre::ContextCompat as _;
use eyre::Result;
use fs_err as fs;

/// Represents all necessary inputs to the program
#[allow(clippy::type_complexity)]
pub struct World {
    pub input_manifest: clap_cargo::Manifest,
    pub input_workspace: clap_cargo::Workspace,
    pub input_features: clap_cargo::Features,
    /// Get rustdoc crate information about the given package
    pub rustdoc_json_for_crate: Box<
        dyn Fn(&cargo_metadata::Package, &cargo_metadata::Metadata) -> Result<rustdoc_types::Crate>
            + Sync,
    >,
    /// Read the given file to a string
    pub read_file: fn(&camino::Utf8Path) -> std::io::Result<String>,
    pub args: Vec<String>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            input_manifest: Default::default(),
            input_workspace: Default::default(),
            input_features: Default::default(),
            rustdoc_json_for_crate: Box::new(move |pkg, metadata| {
                extract_rustdoc_json(pkg, metadata, "nightly")
            }),
            read_file: |path| fs::read_to_string(path),
            args: std::env::args().skip(2).collect(),
        }
    }
}

/// Run `rustdoc` on the package and returns all available information
pub fn extract_rustdoc_json(
    pkg: &cargo_metadata::Package,
    metadata: &cargo_metadata::Metadata,
    toolchain: &str,
) -> Result<rustdoc_types::Crate> {
    let node = metadata
        .resolve
        .as_ref()
        .context("cargo failed resolution")?
        .nodes
        .iter()
        .find(|node| node.id == pkg.id)
        .context("node ID does not exist")?;

    let builder = rustdoc_json::Builder::default()
        .toolchain(toolchain)
        .manifest_path(&pkg.manifest_path)
        .document_private_items(true)
        .no_default_features(true)
        .all_features(false)
        // NOTE: this already includes information about --no-default-features,
        // --features, --all-features etc so we disable those^^
        .features(node.features.as_slice())
        .package_target(extract_package_target(pkg).context("failed to extract package target")?);

    let rustdoc_json_path = builder.build().context("rustdoc error")?;

    let rustdoc_json = fs::read(rustdoc_json_path).context("failed to open rustdoc json file")?;
    let mut rustdoc_json = std::io::Cursor::new(rustdoc_json);

    serde_json::from_reader(&mut rustdoc_json).with_context(|| {
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
}

pub fn extract_package_target(
    pkg: &cargo_metadata::Package,
) -> Result<rustdoc_json::PackageTarget> {
    let target = pkg.targets.first().context("no cargo target")?;
    let package_target = if target.is_kind(cargo_metadata::TargetKind::Bin) {
        rustdoc_json::PackageTarget::Bin(target.name.clone())
    } else {
        rustdoc_json::PackageTarget::Lib
    };
    Ok(package_target)
}
