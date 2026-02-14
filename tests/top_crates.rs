use std::{path::PathBuf, process::Command};

use fs_err as fs;
use rayon::prelude::*;

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

        // Command::new(env!("CARGO_BIN_EXE_cargo-reedme")).arg(arg);
    });

    Ok(())
}
