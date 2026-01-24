//! yes, this is **very cool** crate
//!
//! macro:
//!
//! - [`main`]
//! - [get_readme_path](main)
//!
//! ```
//! hello world
//! ```
//!
//!     hello world
//!
//! ```py
//! hello world
//! ```
//!
//! ```ignore
//! python
//! # hidden line
//! ```

mod intralinks;
mod markdown;
mod replace_content;

use std::io::Cursor;

use cargo_metadata::Package;
use clap::Parser;
use eyre::{Context, ContextCompat, Result};
use fs_err as fs;
use pulldown_cmark::{BrokenLink, BrokenLinkCallback};
use rustdoc_json::PackageTarget;
use rustdoc_types::Crate;
use serde::{Deserialize, Serialize};

use crate::intralinks::Links;

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(format!("cargo_reedme={}", cli.verbosity).parse().unwrap()),
        )
        .without_time()
        .with_target(false)
        .init();

    let mut metadata_cmd = cli.manifest.metadata();
    cli.features.forward_metadata(&mut metadata_cmd);
    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;
    let (pkgs, _excluded_packages) = cli.workspace.partition_packages(&metadata);

    for pkg in pkgs {
        resolve_package(&cli, pkg)
            .with_context(|| format!("failed to create README for package `{}`", pkg.name))?;
    }

    Ok(())
}

fn resolve_package(cli: &Cli, pkg: &Package) -> Result<()> {
    let config = serde_json::from_value::<Option<PackageMetadata>>(pkg.metadata.clone())
        .unwrap_or_default()
        .unwrap_or_default()
        .cargo_reedme
        .unwrap_or_default();

    let rustdoc_json =
        extract_rustdoc_json(pkg, &cli.toolchain).context("failed to run rustdoc")?;

    let root = rustdoc_json.index.get(&rustdoc_json.root).unwrap();
    let intralink_resolver = intralinks::create_links(pkg, &config, &rustdoc_json);

    let markdown = root.docs.as_ref().unwrap();
    let output_markdown = markdown::resolve_markdown(markdown, intralink_resolver);

    println!("{output_markdown}");

    // let readme_path = get_readme_path(pkg).context("failed to get `README.md` path")?;

    Ok(())
}

/// Broken link callback that does nothing.
#[derive(Debug)]
pub struct ResolveLinks<'a> {
    links: &'a Links<'a>,
}

impl<'input> BrokenLinkCallback<'input> for ResolveLinks<'_> {
    fn handle_broken_link(
        &mut self,
        link: BrokenLink<'input>,
    ) -> Option<(
        pulldown_cmark::CowStr<'input>,
        pulldown_cmark::CowStr<'input>,
    )> {
        let url = self.links.get(&*link.reference)?;
        let url = match crate::intralinks::link_fragment(&link.reference) {
            None => url.to_string().into(),
            Some(fragment) => format!("{url}#{fragment}").into(),
        };
        Some((url, "".into()))
    }
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
#[cfg(false)]
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

#[derive(Serialize, Deserialize, Default)]
struct PackageMetadata {
    cargo_reedme: Option<Config>,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    docs_rs: IntralinksDocsRsConfig,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IntralinksDocsRsConfig {
    base_url: Option<String>,
    docs_rs_version: Option<String>,
}
