#![doc = concat!("[![crates.io](https://img.shields.io/crates/v/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=rust)](https://crates.io/crates/", env!("CARGO_PKG_NAME"), ")")]
#![doc = concat!("[![docs.rs](https://img.shields.io/docsrs/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=docs.rs)](https://docs.rs/", env!("CARGO_PKG_NAME"), ")")]
#![doc = "![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)"]
#![doc = concat!("![msrv](https://img.shields.io/badge/msrv-", env!("CARGO_PKG_RUST_VERSION"), "-blue?style=flat-square&logo=rust)")]
//! [![github](https://img.shields.io/github/stars/nik-rev/cargo-reedme)](https://github.com/nik-rev/cargo-reedme)
//!
//! Generate `README.md` from documentation comments in `lib.rs` or `main.rs`
//!
//! - [Example](#example)
//! - [Installation](#installation)
//! - [Features](#features)
//!
//! # Example
//!
//! The following documentation in `lib.rs`:
//!
//! ````rust
//! //! This prints all prime numbers, using [`println!`]:
//! //!
//! //! ```
//! //! # fn main() {
//! //! for i in 2.. {
//! //!     if is_prime(i) {
//! //!         println!("{i}");
//! //!     }
//! //! }
//! //! # }
//! //! ```
//! ````
//!
//! Generates the following `README.md` when running `cargo +nightly reedme`:
//!
//! ````markdown
//! This prints all prime numbers, using [`println!`](https://doc.rust-lang.org/stable/std/macro.println.html):
//!
//! ```rust
//! for i in 2.. {
//!     if is_prime(i) {
//!         println!("{i}");
//!     }
//! }
//! ```
//! ````
//!
//! # Installation
//!
//! ```sh
//! cargo install cargo-reedme
//! ```
//!
//! # Features
//!
//! - [Generate `README.md` from documentation comments in `lib.rs`](#generate-readmemd-from-documentation-comments-in-librs)
//! - [Intra-doc link resolution](#intra-doc-link-resolution)
//! - [Code blocks transformation](#code-blocks-transformation)
//! - [Macro expansion](#macro-expansion)
//! - [Check mode](#check-mode)
//! - [Workspace support](#workspace-support)
//! - [Cargo features](#cargo-features)
//! - [Informational note](#informational-note)
//! - [Config](#config)
//! - [Incremented headings](#incremented-headings)
//! - [Use programmatically from scripts](#use-programmatically-from-scripts)
//!
//! ## Generate `README.md` from documentation comments in `lib.rs`
//!
//! Running `cargo +nightly reedme` will take your doc comments and generate a README from them. This `src/lib.rs`:
//!
//! ```rust
//! //! Hello, world!
//! ```
//!
//! Generates the following `README.md`:
//!
//! ```markdown
//! <!-- cargo-reedme: start -->
//!
//! Hello, world!
//!
//! <!-- cargo-reedme: end -->
//! ```
//!
//! If the `README.md` file already exists, it must have a `<!-- cargo-reedme -->` somewhere inside of it. If the `README.md` contains the following:
//!
//! ```markdown
//! # my_crate
//!
//! <!-- cargo-reedme -->
//! ```
//!
//! Running `cargo +nightly reedme` will replace that `<!-- cargo-reedme -->` with documentation from `lib.rs`:
//!
//! ```markdown
//! # my_crate
//!
//! <!-- cargo-reedme: start -->
//!
//! Hello, world!
//!
//! <!-- cargo-reedme: end -->
//! ```
//!
//! Further invocations of `cargo +nightly reedme` update the inserted region
//!
//! ## Intra-doc link resolution
//!
//! Intra-doc links will be transformed into absolute URLs:
//!
//! ```rust,ignore
//! //! This data structure is [`serde_json::Value`](Value).
//! struct Value;
//! ```
//!
//! The above generate the following `README.md`:
//!
//! ```markdown
//! This data structure is [`serde_json::Value`](https://docs.rs/serde_json/1.0.149/serde_json/enum.Value.html).
//! ```
//!
//! ## Code blocks transformation
//!
//! Code blocks will have `rust` language added, and hidden lines (lines starting with `#`) will be removed:
//!
//! ````rust,ignore
//! //! An example program:
//! //!
//! //! ```
//! //! # fn main() {
//! //! // "hello world" in Rust
//! //! println!("Hello, world!");
//! //! # }
//! //! ```
//! ````
//!
//! The above generates the following `README.md`:
//!
//! ````markdown
//! An example program:
//!
//! ```rust
//! // "hello world" in Rust
//! println!("Hello, world!");
//! ```
//! ````
//!
//! ## Macro expansion
//!
//! Macros in doc comments get properly expanded:
//!
//! ````rust,ignore
//! //! ```toml
//! //! [dependencies]
//! #![doc = concat!("derive_aliases = '", env!("CARGO_PKG_VERSION"), "'")]
//! //! ```
//! ````
//!
//! The above generates the following `README.md`:
//!
//! ````markdown
//! ```toml
//! [dependencies]
//! derive_aliases = '0.4'
//! ```
//! ````
//!
//! Notice that the `concat!` and inner `env!` macro was expanded appropriately.
//!
//! ## Check mode
//!
//! Run `cargo +nightly reedme` as part of your CI pipeline!
//!
//! The `--check` flag is used to make sure PRs keep the `README.md` up to date with `lib.rs` doc comments.
//!
//! An example workflow that runs `cargo +nightly reedme --check` on every commit and PR:
//!
//! ```yaml
//! # .github/workflows/cargo-reedme.yaml
//! name: cargo-reedme
//! on:
//!   pull_request:
//!   push:
//!     branches:
//!       - main
//!
//! jobs:
//!   cargo-reedme:
//!     runs-on: ubuntu-latest
//!     steps:
//!       - uses: actions/checkout@v6
//!
//!       - uses: actions-rust-lang/setup-rust-toolchain@v1
//!
//!       - run: cargo install --locked cargo-reedme
//!
//!       - run: cargo +nightly reedme --check
//! ```
//!
//! On failure, the program exits with a non-zero exit code and prints a colorful diff between the **expected** and **actual** `README.md` files
//!
//! ## Workspace support
//!
//! Generate `README.md`s for all crates in your workspace with a single command!
//!
//! Supports `--workspace`, `--exclude`, and `--package`
//!
//! ## Cargo features
//!
//! Supports `--all-features`, `--features`, and `--no-default-features`
//!
//! ## Informational note
//!
//! When `cargo +nightly reedme` generates your `README.md` file, it will insert a comment that explains how this section was generated:
//!
//! ```markdown
//! # my_crate
//!
//! <!-- cargo-reedme: start -->
//!
//! <!-- cargo-reedme: info-start
//!
//!     Do not edit this region by hand
//!     ===============================
//!
//!     This region was generated from Rust documentation comments by `cargo-reedme` using this command:
//!
//!         cargo reedme
//!
//!     for more info: https://github.com/nik-rev/cargo-reedme
//!
//! cargo-reedme: info-end -->
//!
//! Your documentation
//!
//! <!-- cargo-reedme: end -->
//! ```
//!
//! This note can be customized via the `note` field in `metadata` table in `Cargo.toml`
//!
//! ## Config
//!
//! You can configure the behavior of `cargo reedme` via the crate-level `[package.metadata]` table in `Cargo.toml`:
//!
//! ```toml
//! # project/crates/foo_bar/Cargo.toml
//!
//! [package.metadata.cargo-reedme]
//! # ... your settings go here ...
//! ```
//!
//! ...or the workspace-level `[workspace.metadata]` table
//!
//! ```toml
//! # project/Cargo.toml
//!
//! [workspace.metadata.cargo-reedme]
//! # ... your settings go here ...
//! ```
//!
//! And if none of the above are defined, `cargo reedme` will use fields of the same names as defined in `metadata.docs.rs` (see docs.rs [metadata section](https://docs.rs/about/metadata))
//!
//! ### Default config
//!
//! The default config is this:
//!
//! ```toml
#![doc = include_str!("default_config.toml")]
//! ```
//!
//! Fun fact: This README itself is generated by `cargo +nightly reedme`. The above TOML is added from the [`default_config.toml`](https://github.com/nik-rev/cargo-reedme/blob/main/src/default_config.toml) file like this:
//!
//! ```ignore
//! #![doc = include_str!("default_config.toml")]
//! ```
//!
//! ## Incremented headings
//!
//! It's good practice to only have a single level 1 heading. rustdoc will decrement all of your settings in the output HTML files,
//! so your level 1 headings become level 2 headings and so on.
//!
//! `cargo reedme` does the same. If your README file already has a level 1 heading, every heading will be incremented:
//!
//! ````rust,ignore
//! //! # Usage
//! //!
//! //! ...
//! //!
//! //! # Examples
//! //!
//! //! ...
//! ````
//!
//! The following `README.md` file:
//!
//! ````markdown
//! # docstr
//!
//! <!-- cargo-reedme -->
//! ````
//!
//! Will be updated to this, when running `cargo +nightly reedme`:
//!
//! ````markdown
//! # docstr
//!
//! ## Usage
//!
//! ...
//!
//! ## Examples
//!
//! ...
//! ````
//!
//! ## Use programmatically from scripts
//!
//! The `--json` flag can be used to have `cargo +nightly reedme` do all computation but not write any files, so you can
//! do with that data as you please.
//!
//! You can also use `cargo reedme` as a crate. 95% of the `cargo reedme`'s logic lives in a single, pure function [`cargo_reedme::resolve`](resolve)
//! which does no IO. It has the following signature:
//!
//! ```ignore
//! pub fn resolve(world: &World) -> Result<Output>
//! ```
//!
//! You can take a look at `main.rs` to see how this function is called
//!
//! # Credits
//!
//! This project was inspired by:
//!
//! - [`cargo-readme`](https://github.com/webern/cargo-readme)
//! - [`cargo-rdme`](https://github.com/orium/cargo-rdme)

use camino::Utf8PathBuf;
use cargo_metadata::Package;

use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    insert_into_readme::{ReadmeContentsMeta, ReadmeFile},
};
use eyre::{Context as _, ContextCompat as _, Result};

pub mod world;

pub use world::World;

pub mod config;
pub mod insert_into_readme;
mod intralinks;
mod markdown;
mod replace_content;

/// Output of the program, with all computed README paths
#[derive(Serialize, Deserialize)]
pub struct Output {
    /// Version of the cargo-reedme when it generated the output
    pub version: semver::Version,
    /// List of generated README files
    pub readmes: Vec<GeneratedReadme>,
    /// Errors that were encountered while processing the READMEs
    #[serde(skip)]
    pub errors: Vec<eyre::Report>,
}

/// Data about individual README files
#[derive(Serialize, Deserialize)]
pub struct GeneratedReadme {
    /// Path to the README file
    pub path: Utf8PathBuf,
    /// Name of cargo package, to which this README belongs to
    pub package: String,
    /// Full new contents of the README file
    pub file: ReadmeFile,
    /// Content extracted from documentation comments, with
    /// zero processing applied
    pub original_doc_comments: String,
    /// Configuration that was used to generate this README file
    pub config: Config,
}

pub fn resolve(world: &World) -> Result<Output> {
    let mut metadata_cmd = world.input_manifest.metadata();

    // Takes into account selected features via CLI via --feature flags for example
    world.input_features.forward_metadata(&mut metadata_cmd);

    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;

    // This takes into account selected packages such as via --package
    let (pkgs, _excluded_packages) = world.input_workspace.partition_packages(&metadata);

    let mut readmes = Vec::new();
    let mut errors = Vec::new();

    for pkg in pkgs {
        let result = (|| -> Result<Option<GeneratedReadme>> {
            let config = Config::new(metadata.workspace_metadata.clone(), pkg.metadata.clone());

            // We run `cargo metadata` AGAIN for each package because contents of `[metadata.cargo-reedme]`
            // will actually affect the Cargo metadata, since users can choose different features.
            //
            // Only compute metadata again if it makes sense to - no need to do extra work
            let updated_metadata = if config.affects_cargo_metadata() {
                Some(compute_cargo_metadata_again(world, &config)?)
            } else {
                None
            };

            let metadata = updated_metadata.as_ref().unwrap_or(&metadata);

            let readme_path = get_readme_path_for_package(pkg).with_context(|| {
                format!("failed to get `README.md` path for package `{}`", pkg.name)
            })?;

            let original_readme = match (world.read_file)(&readme_path) {
                Ok(contents) => Some(contents),
                // if this file doesn't exist, we will create it
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
                Err(err) => return Err(err.into()),
            };

            /// Returns whether the passed markdown has any h1 headings
            fn has_h1_heading(content: &str) -> bool {
                let mut parser = pulldown_cmark::Parser::new(content);
                parser.any(|event| {
                    matches!(
                        event,
                        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading {
                            level: pulldown_cmark::HeadingLevel::H1,
                            ..
                        })
                    )
                })
            }

            let (increment_headings, meta) = original_readme.as_ref().map_or(
                (false, ReadmeContentsMeta::NewlyCreated),
                |original_readme| match insert_into_readme::UsersReadmeParts::new(original_readme) {
                    Some(ok) => (
                        has_h1_heading(&ok.before),
                        ReadmeContentsMeta::InsertedIntoUsersReadme(ok),
                    ),
                    None => (false, ReadmeContentsMeta::ErrorMarkerMissing),
                },
            );

            let increment_headings = increment_headings && config.increment_headings;

            let (contents, original_doc_comments) =
                generate_readme_for_package(world, pkg, &config, metadata, increment_headings)
                    .with_context(|| {
                        format!("failed to generate README for package `{}`", pkg.name)
                    })?;

            // skip READMEs that have empty content
            if contents.is_empty() {
                return Ok(None);
            }

            Ok(Some(GeneratedReadme {
                path: readme_path,
                file: ReadmeFile { contents, meta },
                original_doc_comments,
                package: pkg.name.to_string(),
                config,
            }))
        })();

        match result {
            Ok(Some(ok)) => readmes.push(ok),
            Ok(None) => {}
            Err(err) => errors.push(err),
        }
    }

    Ok(Output {
        version: VERSION,
        readmes,
        errors,
    })
}

/// Computes cargo metadata. This time, taking settings like `default-features` into account
fn compute_cargo_metadata_again(
    world: &World,
    config: &Config,
) -> Result<cargo_metadata::Metadata> {
    // Takes into account selected features via passed CLI arguments --feature
    //
    // Note that CLI overrides any configuration arguments (in `[metadata.cargo-reedme]`)
    //
    // For example, specifying `all-features = false` in configuration
    // but `--all-features` in CLI will override the configuration values
    let mut features = clap_cargo::Features::default();

    // extract all values from config

    if let Some(all_features) = config.all_features {
        features.all_features = all_features;
    }

    if let Some(no_default_features) = config.no_default_features {
        features.no_default_features = no_default_features;
    }

    if let Some(config_features) = &config.features {
        features.features = config_features.clone();
    }

    // CLI options take precedence over config options

    if world.input_features.all_features {
        features.all_features = true;
    }

    if world.input_features.no_default_features {
        features.no_default_features = true;
    }

    if !world.input_features.features.is_empty() {
        features.features = world.input_features.features.clone();
    }

    let mut metadata_cmd = world.input_manifest.metadata();

    features.forward_metadata(&mut metadata_cmd);

    metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")
}

const VERSION: semver::Version = semver::Version::new(
    konst::unwrap_ctx!(konst::primitive::parse_u64(env!("CARGO_PKG_VERSION_MAJOR"))),
    konst::unwrap_ctx!(konst::primitive::parse_u64(env!("CARGO_PKG_VERSION_MINOR"))),
    konst::unwrap_ctx!(konst::primitive::parse_u64(env!("CARGO_PKG_VERSION_PATCH"))),
);

/// For a given Cargo package, returns contents of generated README.md
fn generate_readme_for_package(
    world: &World,
    pkg: &Package,
    config: &Config,
    workspace_metadata: &cargo_metadata::Metadata,
    increment_headings: bool,
) -> Result<(String, String)> {
    let krate = (world.rustdoc_json_for_crate)(pkg, workspace_metadata, config, &world.toolchain)
        .context("failed to run rustdoc")?;

    let root = krate
        .index
        .get(&krate.root)
        .expect("rustdoc's root item is a valid item");

    // Get the link map, which for [main function](main) creates: { "main": "https://example.com" }
    let links = intralinks::create_links(world, pkg, config, &krate);

    let docs = root.docs.as_deref().unwrap_or_default();

    // All links in the markdown are rewritten to consider the link map, e.g. [main function](https://example.com)
    Ok((
        markdown::resolve_markdown(docs, links, increment_headings),
        docs.to_string(),
    ))
}

/// For the given Cargo package, gets the path to the package's README.md file
fn get_readme_path_for_package(pkg: &cargo_metadata::Package) -> Result<Utf8PathBuf> {
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
