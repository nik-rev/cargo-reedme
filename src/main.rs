use std::io::Write as _;

use eyre::Context as _;
use eyre::ContextCompat as _;
use eyre::Result;
use fs_err as fs;
use rayon::prelude::*;

use clap::Parser;

/// Cargo plugin that generates `README.md` files from documentation comments in `lib.rs` or `main.rs`
#[derive(Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Cli {
    /// Write JSON to stdout
    #[arg(long)]
    pub json: bool,
    /// Specify a custom toolchain to use
    #[arg(long, default_value = "nightly")]
    pub toolchain: String,

    // cargo-specific flags for package resolution
    //
    #[command(flatten)]
    pub manifest: clap_cargo::Manifest,
    #[command(flatten)]
    pub workspace: clap_cargo::Workspace,
    #[command(flatten)]
    pub features: clap_cargo::Features,
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    init_logging(cli.verbosity);

    let world = cargo_reedme::World {
        manifest: cli.manifest,
        workspace: cli.workspace,
        features: cli.features,
        rustdoc_json_for_crate: Box::new(move |pkg| extract_rustdoc_json(pkg, &cli.toolchain)),
        read_file: |a| fs::read_to_string(a),
    };

    // Writes README.md files for each Cargo package
    let mut output = cargo_reedme::resolve(&world)?;

    // Errors are sorted by their display, so we show the same errors at the same time
    output.errors.sort_by_key(|x| x.to_string());

    // Let's report each individual error rather than just the first one
    for err in &output.errors {
        eprintln!("{err:?}");
    }

    if !output.errors.is_empty() {
        std::process::exit(1);
    }

    // Execute the actual function of the program

    if cli.json {
        output
            .generated_readmes
            .sort_unstable_by(|a, b| a.readme_path.cmp(&b.readme_path));

        let json = colored_json::to_colored_json_auto(&output).context("failed to write json")?;

        std::io::stdout()
            .write_all(json.as_bytes())
            .context("failed to write JSON")?;
    } else {
        // Regular output
        output.generated_readmes.par_iter().for_each(|readme| {
            if let Err(err) = fs::write(&readme.readme_path, readme.readme_contents.to_string())
                .context("failed to write `README.md` file")
            {
                println!("{err}");
            }
        });
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

/// Run Rustdoc on the package, generate the JSON into a file
///
/// Returns path to the file
fn extract_rustdoc_json(
    pkg: &cargo_metadata::Package,
    toolchain: &str,
) -> Result<rustdoc_types::Crate> {
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
    let mut rustdoc_json = std::io::Cursor::new(rustdoc_json);

    serde_json::from_reader(&mut rustdoc_json).context("failed to deserialize rustdoc json")
}

fn extract_package_target(pkg: &cargo_metadata::Package) -> Result<rustdoc_json::PackageTarget> {
    let target = pkg.targets.first().context("no cargo target")?;
    let package_target = if target.is_kind(cargo_metadata::TargetKind::Bin) {
        rustdoc_json::PackageTarget::Bin(target.name.clone())
    } else {
        rustdoc_json::PackageTarget::Lib
    };
    Ok(package_target)
}
