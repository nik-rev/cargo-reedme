use std::path::PathBuf;

use camino::Utf8PathBuf;
use cargo_metadata::{Package, camino::Utf8Ancestors};
use clap::Parser;
use eyre::{Context, ContextCompat, Result};

#[derive(Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
struct Cli {
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
    let (pkgs, excluded_packages) = cli.workspace.partition_packages(&metadata);

    for pkg in pkgs {
        let pkg_name = &pkg.name;

        let readme_path = get_readme_path(pkg)
            .with_context(|| format!("failed to get `README.md` path for package `{pkg_name}`"))?;
    }

    println!("Hello, world!");

    Ok(())
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
