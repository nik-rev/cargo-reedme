//! yes, this is **very cool** crate
//!
//! macro: [`get_readme_path`]
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
//! ```

mod intralinks;

use std::io::Cursor;

use camino::Utf8PathBuf;
use cargo_metadata::Package;
use clap::Parser;
use eyre::{Context, ContextCompat, Result};
use fs_err as fs;
use pulldown_cmark::{BrokenLink, BrokenLinkCallback};
use rustdoc_json::PackageTarget;
use rustdoc_types::Crate;
use serde::{Deserialize, Serialize};

use crate::intralinks::{create_intralink_resolver, is_rust_code_block, items_info};

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
    let intralink_resolver = create_intralink_resolver(pkg, &config, &rustdoc_json);

    /// Broken link callback that does nothing.
    #[derive(Debug)]
    pub struct ResolveIntraDocLinks<'a> {
        intralink_resolver: intralinks::IntralinkResolver<'a>,
    }

    impl<'input> BrokenLinkCallback<'input> for ResolveIntraDocLinks<'_> {
        fn handle_broken_link(
            &mut self,
            link: BrokenLink<'input>,
        ) -> Option<(
            pulldown_cmark::CowStr<'input>,
            pulldown_cmark::CowStr<'input>,
        )> {
            let link = intralinks::Link {
                raw_link: link.reference.to_string(),
            };
            if let Some(url) = self.intralink_resolver.resolve_link(&link) {
                let url = match link.link_fragment() {
                    None => url.to_owned(),
                    Some(fragment) => format!("{url}#{fragment}"),
                };
                Some((url.into(), "".into()))
            } else {
                None
            }
        }
    }

    let markdown = root.docs.as_ref().unwrap();
    let cmark = pulldown_cmark::Parser::new_with_broken_link_callback(
        markdown,
        pulldown_cmark::Options::all(),
        Some(ResolveIntraDocLinks { intralink_resolver }),
    );
    let cmark = pulldown_cmark::TextMergeStream::new(cmark);

    #[derive(Default)]
    struct State {
        is_rust_codeblock: bool,
    }

    let cmark = cmark.scan(State::default(), |state, event| match &event {
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(code_block_kind)) => {
            match code_block_kind {
                pulldown_cmark::CodeBlockKind::Indented => state.is_rust_codeblock = true,
                pulldown_cmark::CodeBlockKind::Fenced(cow_str) => {
                    state.is_rust_codeblock = is_rust_code_block(cow_str);
                }
            };

            if state.is_rust_codeblock {
                // Change language to rust
                Some(pulldown_cmark::Event::Start(
                    pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(
                        "rust".into(),
                    )),
                ))
            } else {
                // Keep code block as-is
                Some(event)
            }
        }
        // pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
        //     link_type,
        //     dest_url,
        //     title,
        //     id,
        // }) => root.links.get(k),
        pulldown_cmark::Event::Text(text) if state.is_rust_codeblock => {
            state.is_rust_codeblock = false;
            Some(pulldown_cmark::Event::Text("transformed".into()))
        }
        _ => {
            state.is_rust_codeblock = false;
            Some(event)
        }
    });

    let mut output_markdown = String::new();
    let _ = pulldown_cmark_to_cmark::cmark(cmark, &mut output_markdown)
        .context("failed to write markdown");

    println!("{output_markdown}");

    // let readme_path = get_readme_path(pkg).context("failed to get `README.md` path")?;

    Ok(())
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
    docs_rs: intralinks::IntralinksDocsRsConfig,
}
