//! lmao
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::Package;

use itertools::Either;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{config::Config, insert_into_readme::ReadmeContents};
use eyre::{Context as _, ContextCompat as _, Result};

mod config;
mod insert_into_readme;
mod intralinks;
mod markdown;
mod replace_content;

/// Represents all necessary inputs to the program
#[allow(clippy::type_complexity)]
pub struct World {
    pub manifest: clap_cargo::Manifest,
    pub workspace: clap_cargo::Workspace,
    pub features: clap_cargo::Features,
    /// Get rustdoc crate information about the given package
    pub rustdoc_json_for_crate: Box<dyn Fn(&Package) -> Result<rustdoc_types::Crate> + Sync>,
    /// Read the given file to a string
    pub read_file: fn(&Utf8Path) -> std::io::Result<String>,
}

/// Output of the program, with all computed README paths
#[derive(Serialize, Deserialize)]
pub struct Output {
    /// Version of the cargo-reedme when it generated the output
    pub version: semver::Version,
    /// List of generated README files
    pub generated_readmes: Vec<GeneratedReadme>,
    /// Errors that were encountered while processing the READMEs
    #[serde(skip)]
    pub errors: Vec<eyre::Report>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratedReadme {
    pub path: Utf8PathBuf,
    pub package: String,
    pub contents: ReadmeContents,
}

pub fn resolve(world: &World) -> Result<Output> {
    try_map_each_package(world, |pkg, workspace_metadata| {
        let generated_readme = generate_readme_for_package(world, pkg, workspace_metadata)
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
            Some(original_readme) => ReadmeContents::InsertedIntoExisting(
                insert_into_readme::ReadmeParts::new(&original_readme, &generated_readme)
                    .with_context(|| {
                        format!("failed to find edit location in README: {readme_path}")
                    })?,
            ),
            None => ReadmeContents::NewlyCreated(generated_readme),
        };

        Ok(GeneratedReadme {
            path: readme_path,
            contents: new_readme,
            package: pkg.name.to_string(),
        })
    })
    .map(|(readmes, errors)| Output {
        version: VERSION,
        generated_readmes: readmes,
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
    f: impl Fn(&cargo_metadata::Package, &Config) -> Result<T> + Sync,
) -> Result<(Vec<T>, Vec<eyre::Report>)> {
    let mut metadata_cmd = world.manifest.metadata();

    // Takes into account selected features via --feature
    world.features.forward_metadata(&mut metadata_cmd);

    let metadata = metadata_cmd
        .exec()
        .context("failed to obtain Cargo metadata")?;

    // This takes into account selected packages such as via --package
    let (pkgs, _excluded_packages) = world.workspace.partition_packages(&metadata);

    // Config from [workspace.metadata.cargo-reedme]
    let config = Config::from_cargo_metadata(metadata.workspace_metadata.clone());

    let (oks, errs): (Vec<_>, Vec<_>) =
        pkgs.into_par_iter()
            .partition_map(|pkg| match f(pkg, &config) {
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
) -> Result<String> {
    // Read configuration as specified in [package.metadata.cargo-reedme]
    let mut config = Config::from_cargo_metadata(pkg.metadata.clone());
    // Inherit values from [workspace.metadata.cargo-reedme]
    config.inherit_workspace_metadata(workspace_config.clone());

    let krate = (world.rustdoc_json_for_crate)(pkg).context("failed to run rustdoc")?;

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
