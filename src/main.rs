use std::io::Write as _;

use eyre::Context as _;
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
    let cli = Cli::parse();

    init_logging(cli.verbosity);

    let world = cargo_reedme::World {
        input_manifest: cli.manifest,
        input_workspace: cli.workspace,
        input_features: cli.features,
        rustdoc_json_for_crate: Box::new(move |pkg, metadata| {
            let toolchain = match std::env::var("RUSTUP_TOOLCHAIN") {
                Ok(toolchain) if !toolchain.contains("nightly") => {
                    println!(
                        "`cargo-reedme` only works with a nightly Rust toolchain: using `nightly` instead of `{toolchain}`"
                    );
                    String::from("nightly")
                }
                Ok(toolchain) => toolchain,
                Err(_) => String::from("nightly"),
            };

            cargo_reedme::world::extract_rustdoc_json(pkg, metadata, &toolchain)
        }),
        read_file: |path| fs::read_to_string(path),
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
        output.readmes.sort_unstable_by(|a, b| a.path.cmp(&b.path));

        let json = colored_json::to_colored_json_auto(&output).context("failed to write json")?;

        std::io::stdout()
            .write_all(json.as_bytes())
            .context("failed to write JSON")?;
    } else {
        // Regular output
        output.readmes.par_iter().for_each(|readme| {
            let Some(content) = readme.file.to_readme(std::env::args().skip(1)) else {
                docstr::docstr!(eprintln!
                    /// can't figure out where to insert generated content in: {}
                    ///
                    /// please add `<!-- cargo-reedme -->` somewhere in your README, as that's where
                    /// the generated portion from rustdoc comments will be inserted!
                    readme.path,
                );
                return;
            };

            if let Err(err) =
                fs::write(&readme.path, content).context("failed to write `README.md` file")
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
