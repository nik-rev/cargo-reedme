use std::io::{Cursor, Write};

use camino::Utf8PathBuf;
use cargo_metadata::Package;
use clap::Parser;
use eyre::{Context, ContextCompat, Result};
use fs_err as fs;
use itertools::Either;
use rayon::prelude::*;
use rustdoc_json::PackageTarget;
use rustdoc_types::Crate;
use serde::{Deserialize, Serialize};

use crate::{config::Config, insert_into_readme::ReadmeContents};

mod config;
mod insert_into_readme;
mod intralinks;
mod markdown;
mod replace_content;

#[derive(Serialize, Deserialize)]
struct JsonOutput {
    version: String,
    generated_readmes: Vec<GeneratedReadme>,
}

#[derive(Serialize, Deserialize)]
struct GeneratedReadme {
    readme_path: Utf8PathBuf,
    package: String,
    readme_contents: ReadmeContents,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    init_logging(cli.verbosity);

    // Writes README.md files for each Cargo package
    let mut data = map_each_package(&cli, |pkg, workspace_metadata| {
        let generated_readme = generate_readme_for_package(&cli, pkg, workspace_metadata)
            .with_context(|| format!("failed to generate README for package `{}`", pkg.name))?;

        let readme_path =
            get_readme_path_for_package(pkg).context("failed to get `README.md` path")?;

        let original_readme = match fs::read_to_string(&readme_path) {
            Ok(contents) => Some(contents),
            // if this file doesn't exist, we will create it
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => return Err(err.into()),
        };

        // NOTE: not .map() due to ownership issues
        let new_readme = match original_readme {
            Some(original_readme) => ReadmeContents::InsertedIntoExisting(
                insert_into_readme::ReadmeParts::new(&original_readme, &generated_readme)?,
            ),
            None => ReadmeContents::NewlyCreated(generated_readme),
        };

        if !cli.json {
            fs::write(&readme_path, new_readme.to_string())
                .context("failed to write `README.md` file")?;
        }

        Ok(GeneratedReadme {
            readme_path,
            readme_contents: new_readme,
            package: pkg.name.to_string(),
        })
    })?;

    if cli.json {
        data.sort_unstable_by(|a, b| a.readme_path.cmp(&b.readme_path));

        let json = colored_json::to_colored_json_auto(&JsonOutput {
            version: env!("CARGO_PKG_VERSION").to_string(),
            generated_readmes: data,
        })
        .context("failed to write json")?;

        std::io::stdout()
            .write_all(json.as_bytes())
            .context("failed to write JSON")?;
    }

    Ok(())
}

/// Cargo plugin that generates `README.md` files from documentation comments in `lib.rs` or `main.rs`
#[derive(Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
struct Cli {
    /// Write JSON to stdout
    #[arg(long)]
    json: bool,
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
fn map_each_package<T: Send>(
    cli: &Cli,
    f: impl Fn(&Package, &Config) -> Result<T> + Sync,
) -> Result<Vec<T>> {
    let mut metadata_cmd = cli.manifest.metadata();

    // Takes into account selected features via --feature
    cli.features.forward_metadata(&mut metadata_cmd);

    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;

    // This takes into account selected packages such as via --package
    let (pkgs, _excluded_packages) = cli.workspace.partition_packages(&metadata);

    // Config from [workspace.metadata.cargo-reedme]
    let config = Config::from_cargo_metadata(metadata.workspace_metadata.clone());

    let (oks, mut errs): (Vec<_>, Vec<_>) =
        pkgs.into_par_iter()
            .partition_map(|pkg| match f(pkg, &config) {
                Ok(ok) => Either::Left(ok),
                Err(err) => Either::Right(err),
            });

    // Errors are sorted by their display, so we show the same errors at the same time
    errs.sort_by_key(|x| x.to_string());

    // Let's report each individual error rather than just the first one
    for err in errs {
        eprintln!("{err}");
    }

    Ok(oks)
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
