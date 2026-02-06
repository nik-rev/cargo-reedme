use fs_err as fs;
use rayon::prelude::*;

// cargo test download_top_crates -- --ignored
#[ignore]
#[test]
fn download_top_crates() -> eyre::Result<()> {
    let (dependencies, _) = rust_playground_top_crates::generate_info(&Default::default());

    // Create a base directory for your 100 crates
    let base_path = "downloaded_crates";
    fs::create_dir_all(base_path)?;

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
    });

    Ok(())
}
