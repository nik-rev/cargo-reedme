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
        }
    }
}

/// Run Rustdoc on the package, generate the JSON into a file
///
/// Returns path to the file
pub fn extract_rustdoc_json(
    pkg: &cargo_metadata::Package,
    metadata: &cargo_metadata::Metadata,
    toolchain: &str,
) -> Result<rustdoc_types::Crate> {
    let node = metadata
        .resolve
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|node| node.id == pkg.id)
        .unwrap();

    let builder = rustdoc_json::Builder::default()
        .toolchain(toolchain)
        .manifest_path(&pkg.manifest_path)
        .document_private_items(true)
        .no_default_features(true)
        .all_features(false)
        .features(dbg!(node.features.as_slice()))
        .quiet(true)
        .color(rustdoc_json::Color::Never)
        .package_target(extract_package_target(pkg).context("failed to extract package target")?);

    let mut stderr = Vec::new();
    let rustdoc_json_path = builder
        .build_with_captured_output(std::io::sink(), &mut stderr)
        .with_context(|| {
            format!(
                "rustdoc stderr: {}",
                String::from_utf8(stderr)
                    .expect("rustdoc outputs valid utf-8")
                    .trim()
            )
        })?;

    let rustdoc_json = fs::read(rustdoc_json_path).context("failed to open rustdoc json file")?;
    let mut rustdoc_json = std::io::Cursor::new(rustdoc_json);

    serde_json::from_reader(&mut rustdoc_json).context("failed to deserialize rustdoc json")
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
