use std::io::Cursor;

use camino::Utf8PathBuf;
use cargo_metadata::Package;
use clap::Parser;
use eyre::{Context, ContextCompat, Result};
use fs_err as fs;
use rustdoc_json::PackageTarget;
use rustdoc_types::Crate;

use crate::config::Config;

mod config;
mod intralinks;
mod markdown;
mod replace_content;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    init_logging(cli.verbosity);

    // Writes README.md files for each Cargo package
    for_each_package(&cli, |pkg, workspace_metadata| {
        let readme = generate_readme_for_package(&cli, pkg, workspace_metadata)
            .with_context(|| format!("failed to generate README for package `{}`", pkg.name))?;

        let readme_path =
            get_readme_path_for_package(pkg).context("failed to get `README.md` path")?;

        fs::write(readme_path, readme).context("failed to write `README.md` file")?;

        Ok(())
    })?;

    Ok(())
}

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
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

/// Calls the given function for each Cargo package in the workspace
fn for_each_package(cli: &Cli, f: impl Fn(&Package, &Config) -> Result<()>) -> Result<()> {
    let mut metadata_cmd = cli.manifest.metadata();

    // Takes into account selected features via --feature
    cli.features.forward_metadata(&mut metadata_cmd);

    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;

    // This takes into account selected packages such as via --package
    let (pkgs, _excluded_packages) = cli.workspace.partition_packages(&metadata);

    // Let's report each individual error rather than just the first one
    let mut errs = Vec::new();

    // Config from [workspace.metadata.cargo-reedme]
    let config = Config::from_cargo_metadata(metadata.workspace_metadata.clone());

    for pkg in pkgs {
        match f(pkg, &config) {
            Ok(()) => (),
            Err(err) => errs.push(err),
        };
    }

    for err in errs {
        eprintln!("{err}");
    }

    Ok(())
}

fn init_logging(verbosity: clap_verbosity_flag::Verbosity) {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(format!("cargo_reedme={verbosity}").parse().unwrap()),
        )
        .without_time()
        .with_target(false)
        .init();
}

/// For a given Cargo package, returns contents of generated README.md
fn generate_readme_for_package(
    cli: &Cli,
    pkg: &Package,
    workspace_config: &Config,
) -> Result<String> {
    // Read configuration as specified in [package.metadata.cargo-reedme]
    let mut config = Config::from_cargo_metadata(pkg.metadata.clone());
    // Inherit values from [workspace.metadata.cargo-reedme]
    config.inherit_workspace_metadata(workspace_config.clone());

    let krate = extract_rustdoc_json(pkg, &cli.toolchain).context("failed to run rustdoc")?;

    let root = krate
        .index
        .get(&krate.root)
        .expect("rustdoc's root item is a valid item");

    // Get the link map, which for [main function](main) creates: { "main": "https://example.com" }
    let links = intralinks::create_links(pkg, &config, &krate);

    // All links in the markdown are rewritten to consider the link map, e.g. [main function](https://example.com)
    Ok(markdown::resolve_markdown(
        root.docs.as_deref().unwrap_or_default(),
        links,
    ))
}

fn extract_package_target(pkg: &Package) -> Result<PackageTarget> {
    let target = pkg.targets.first().context("no cargo target")?;
    let package_target = if target.is_kind(cargo_metadata::TargetKind::Bin) {
        PackageTarget::Bin(target.name.clone())
    } else {
        PackageTarget::Lib
    };
    Ok(package_target)
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
    let mut rustdoc_json = Cursor::new(rustdoc_json);

    serde_json::from_reader(&mut rustdoc_json).context("failed to deserialize rustdoc json")
}

/// For the given Cargo package, gets the path to the package's README.md file
fn get_readme_path_for_package(pkg: &Package) -> Result<Utf8PathBuf> {
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
