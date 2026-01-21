use std::{
    env,
    io::{BufReader, Cursor},
    path::PathBuf,
};

use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Package, camino::Utf8Ancestors};
use clap::Parser;
use eyre::{Context, ContextCompat, Result, eyre};
use fs_err as fs;
use rustdoc_types::Crate;

#[derive(Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
struct Cli {
    /// Specify a custom toolchain to use
    #[arg(long, default_value = "nightly")]
    toolchain: String,
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
    #[command(flatten)]
    workspace: clap_cargo::Workspace,
    #[command(flatten)]
    features: clap_cargo::Features,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let mut metadata_cmd = cli.manifest.metadata();
    cli.features.forward_metadata(&mut metadata_cmd);
    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;
    let (pkgs, _excluded_packages) = cli.workspace.partition_packages(&metadata);

    for pkg in pkgs {
        resolve_package(&cli, pkg)
            .with_context(|| format!("failed to resolve package `{}`", pkg.name))?;
    }

    // println!("Hello, world!");

    Ok(())
}

fn resolve_package(cli: &Cli, pkg: &Package) -> Result<()> {
    let rustdoc_json =
        extract_rustdoc_json(pkg, &cli.toolchain).context("failed to run rustdoc")?;

    let readme_path = get_readme_path(pkg).context("failed to get `README.md` path")?;

    Ok(())
}

/// Run Rustdoc on the package, generate the JSON into a file
///
/// Returns path to the file
fn extract_rustdoc_json(pkg: &Package, toolchain: &str) -> Result<Crate> {
    let builder = rustdoc_json::Builder::default()
        .toolchain(toolchain)
        .manifest_path(&pkg.manifest_path)
        .document_private_items(true)
        .no_default_features(true)
        .all_features(false)
        .features(pkg.features.keys())
        .quiet(true)
        .color(rustdoc_json::Color::Never)
        .package_target(rustdoc_json::PackageTarget::Lib);

    let mut stderr = Vec::new();
    let rustdoc_json_path = builder
        .build_with_captured_output(std::io::sink(), &mut stderr)
        .context("failed to run rustdoc")?;

    let rustdoc_json = fs::read(rustdoc_json_path).context("failed to open rustdoc json file")?;
    let mut rustdoc_json = Cursor::new(rustdoc_json);

    serde_json::from_reader(&mut rustdoc_json).context("failed to deserialize rustdoc json")
}

/// For the given Cargo package, gets the path to the package's README.md file
fn get_readme_path(pkg: &Package) -> Result<Utf8PathBuf> {
    let readme_path = match pkg.readme() {
        Some(path) => path,
        None => {
            let parent = pkg
                .manifest_path
                .parent()
                .context("manifest has no parent directory")?;

            parent.join("README.md")
        }
    };

    Ok(readme_path)
}
