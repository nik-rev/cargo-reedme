use std::path::PathBuf;

use fs_err as fs;
use rayon::prelude::*;

// Downloads top crates into `top_crates` directory, and generates README data for them,
// so it is easy to see the real-world effects of `cargo-reedme`
//
// TO RUN: cargo test download_top_crates -- --ignored
#[ignore]
#[test]
fn download_top_crates() -> eyre::Result<()> {
    let (dependencies, _) = rust_playground_top_crates::generate_info(&Default::default());

    let downloaded_crates_path =
        PathBuf::from(format!("{}/downloaded_crates", env!("CARGO_MANIFEST_DIR")));

    fs::remove_dir_all(&downloaded_crates_path)?;
    fs::create_dir_all(&downloaded_crates_path)?;

    dependencies.into_par_iter().for_each(|(name, spec)| {
        let version = spec.version;
        let url = format!("https://crates.io/api/v1/crates/{name}/{version}/download");

        println!("Downloading {name} v{version}...");

        let Ok(mut response) = ureq::get(&url)
            .header("User-Agent", "cargo-reedme test suite")
            .call()
        else {
            return;
        };

        let tar_gz = flate2::read::GzDecoder::new(response.body_mut().as_reader());
        let mut archive = tar::Archive::new(tar_gz);

        let unpack_dir = format!("{}/tests/top_crates", env!("CARGO_MANIFEST_DIR"));
        archive.unpack(&unpack_dir).unwrap();

        println!("Unpacked to {}", unpack_dir);

        // set manifest to point to the package's Cargo.toml
        let mut manifest = clap_cargo::Manifest::default();
        manifest.manifest_path = Some(format!("{unpack_dir}/{name}-{version}/Cargo.toml").into());

        // use the correct feature set
        let mut features = clap_cargo::Features::default();
        features.no_default_features = !spec.default_features;
        features.features = spec.features.into_iter().map(|s| s.to_string()).collect();

        let output = cargo_reedme::resolve(&cargo_reedme::World {
            manifest,
            features,
            ..Default::default()
        })
        .unwrap();

        for readme in output.readmes {
            let base = readme.path.parent().unwrap();
            let filename = readme.path.file_name().unwrap();

            let generated = readme.file.contents;
            let original = readme.original_doc_comments;

            // this file shows the content that we actually generate
            fs::write(format!("{base}/cargo-reedme.{filename}"), &generated).unwrap();

            // diff from the raw content inside of doc comments
            fs::write(
                format!("{base}/cargo-reedme.{filename}.diff"),
                similar::TextDiff::from_lines(&original, &generated)
                    .unified_diff()
                    .to_string(),
            )
            .unwrap();
        }
    });

    Ok(())
}
