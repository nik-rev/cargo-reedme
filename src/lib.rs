use camino::Utf8PathBuf;
use cargo_metadata::Package;

use itertools::Either;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    insert_into_readme::{ReadmeContentsMeta, ReadmeFile},
};
use eyre::{Context as _, ContextCompat as _, Result};

pub mod world;

pub use world::World;

mod config;
mod insert_into_readme;
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
    /// Cargo package, to which this README belongs to
    pub package: String,
    /// Full new contents of the README file
    pub file: ReadmeFile,
    /// Content extracted from documentation comments, with
    /// zero processing applied
    pub original_doc_comments: String,
}

pub fn resolve(world: &World) -> Result<Output> {
    try_map_each_package(world, |pkg, config, workspace_metadata| {
        let (generated_readme, original_doc_comments) =
            generate_readme_for_package(world, pkg, config, workspace_metadata)
                .with_context(|| format!("failed to generate README for package `{}`", pkg.name))?;

        let readme_path = get_readme_path_for_package(pkg).with_context(|| {
            format!("failed to get `README.md` path for package `{}`", pkg.name)
        })?;

        let original_readme = match (world.read_file)(&readme_path) {
            Ok(contents) => Some(contents),
            // if this file doesn't exist, we will create it
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => return Err(err.into()),
        };

        // NOTE: not .map() due to ownership issues
        let new_readme = match original_readme {
            Some(original_readme) => {
                let meta = match insert_into_readme::UsersReadmeParts::new(&original_readme) {
                    Some(ok) => ReadmeContentsMeta::InsertedIntoUsersReadme(ok),
                    None => ReadmeContentsMeta::ErrorMarkerMissing,
                };

                ReadmeFile {
                    contents: generated_readme,
                    meta,
                }
            }
            None => ReadmeFile {
                contents: generated_readme.to_string(),
                meta: ReadmeContentsMeta::NewlyCreated,
            },
        };

        Ok(GeneratedReadme {
            path: readme_path,
            file: new_readme,
            original_doc_comments,
            package: pkg.name.to_string(),
        })
    })
    .map(|(readmes, errors)| Output {
        version: VERSION,
        readmes,
        errors,
    })
}

const VERSION: semver::Version = semver::Version::new(
    konst::unwrap_ctx!(konst::primitive::parse_u64(env!("CARGO_PKG_VERSION_MAJOR"))),
    konst::unwrap_ctx!(konst::primitive::parse_u64(env!("CARGO_PKG_VERSION_MINOR"))),
    konst::unwrap_ctx!(konst::primitive::parse_u64(env!("CARGO_PKG_VERSION_PATCH"))),
);

/// Calls the given function for each Cargo package in the workspace
fn try_map_each_package<T: Send>(
    world: &World,
    f: impl Fn(&cargo_metadata::Package, &Config, &cargo_metadata::Metadata) -> Result<T> + Sync,
) -> Result<(Vec<T>, Vec<eyre::Report>)> {
    let mut metadata_cmd = world.input_manifest.metadata();

    // Takes into account selected features via --feature
    world.input_features.forward_metadata(&mut metadata_cmd);

    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;

    // This takes into account selected packages such as via --package
    let (pkgs, _excluded_packages) = world.input_workspace.partition_packages(&metadata);

    // Config from [workspace.metadata.cargo-reedme]
    let config = Config::from_cargo_metadata(metadata.workspace_metadata.clone());

    let (oks, errs): (Vec<_>, Vec<_>) =
        pkgs.into_par_iter()
            .partition_map(|pkg| match f(pkg, &config, &metadata) {
                Ok(ok) => Either::Left(ok),
                Err(err) => Either::Right(err),
            });

    Ok((oks, errs))
}

/// For a given Cargo package, returns contents of generated README.md
fn generate_readme_for_package(
    world: &World,
    pkg: &Package,
    workspace_config: &Config,
    workspace_metadata: &cargo_metadata::Metadata,
) -> Result<(String, String)> {
    // Read configuration as specified in [package.metadata.cargo-reedme]
    let mut config = Config::from_cargo_metadata(pkg.metadata.clone());
    // Inherit values from [workspace.metadata.cargo-reedme]
    config.inherit_workspace_metadata(workspace_config.clone());

    let krate =
        (world.rustdoc_json_for_crate)(pkg, workspace_metadata).context("failed to run rustdoc")?;

    let root = krate
        .index
        .get(&krate.root)
        .expect("rustdoc's root item is a valid item");

    // Get the link map, which for [main function](main) creates: { "main": "https://example.com" }
    let links = intralinks::create_links(pkg, &config, &krate);

    let docs = root.docs.as_deref().unwrap_or_default();

    // All links in the markdown are rewritten to consider the link map, e.g. [main function](https://example.com)
    Ok((markdown::resolve_markdown(docs, links), docs.to_string()))
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
