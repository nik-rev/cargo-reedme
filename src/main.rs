//! This is the main binary that calls cargo-reedme API. Major logic is implemented in `lib.rs`,
//! this just provides a command-line interface

use std::io::Write as _;

use cargo_reedme::World;
use docstr::docstr;
use eyre::Context as _;
use eyre::Result;
use eyre::bail;
use fs_err as fs;
use rayon::prelude::*;

use clap::Parser;

mod diff_file;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub enum Command {
    #[command(name = "reedme")]
    #[command(about, author, version)]
    Reedme(Cli),
}

#[derive(clap::Args)]
pub struct Cli {
    /// Check that running will not modify any files. Use this in CI
    ///
    /// This command will succeed if no modifications will be made to any README files
    /// when they are generated. It otherwise fails, printing a diff between the current files and
    /// what `cargo-reedme` would have written
    #[arg(short, long)]
    pub check: bool,

    /// Output JSON to stdout, instead of writing README contents
    #[arg(short, long)]
    pub json: bool,

    /// Use colored output
    #[arg(hide = true, long, default_value = "auto")]
    pub color: diff_file::Color,

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
    let Command::Reedme(cli) = Command::parse();

    init_logging(cli.verbosity);

    let world = cargo_reedme::World {
        input_manifest: cli.manifest,
        input_workspace: cli.workspace,
        input_features: cli.features,
        rustdoc_json_for_crate: Box::new(move |pkg, metadata| {
            let toolchain = match std::env::var("RUSTUP_TOOLCHAIN") {
                Ok(toolchain) if !toolchain.contains("nightly") => {
                    println!(
                        "`cargo reedme` only works with a nightly Rust toolchain: using `nightly` instead of `{toolchain}`"
                    );
                    String::from("nightly")
                }
                Ok(toolchain) => toolchain,
                Err(_) => String::from("nightly"),
            };

            cargo_reedme::world::extract_rustdoc_json(pkg, metadata, &toolchain)
        }),
        read_file: |path| fs::read_to_string(path),
        ..Default::default()
    };

    // Writes README.md files for each Cargo package
    let mut output = cargo_reedme::resolve(&world)?;

    // take because we need ownership of `output` to pretty-print it if `--json`
    // flag is passed, but we actually don't read that field because it is marked `#[serde(skip)]`
    report_errors(std::mem::take(&mut output.errors));

    /// What action the program should take
    enum Action {
        /// Will output all JSON data to stdout
        OutputJson,
        /// Will write the README contents we emit to the file system
        WriteFiles,
        /// Will check that the README contents we emit is the same as
        /// contents of the actual README files
        RunCheck,
    }

    let action = match [cli.json, cli.check] {
        [true, true] => bail!("flags `--json` and `--check` are mutually exclusive"),
        [true, false] => Action::OutputJson,
        [false, true] => Action::RunCheck,
        [false, false] => Action::WriteFiles,
    };

    let errors = match action {
        Action::OutputJson => {
            output.readmes.sort_unstable_by(|a, b| a.path.cmp(&b.path));

            let json =
                colored_json::to_colored_json_auto(&output).context("failed to write json")?;

            std::io::stdout()
                .write_all(json.as_bytes())
                .context("failed to write JSON")?;

            Vec::new()
        }
        Action::WriteFiles => output
            .readmes
            .par_iter()
            .map(|readme| -> Result<()> {
                if let Err(err) = fs::write(&readme.path, read_readme_file(&world, readme)?)
                    .context("failed to write `README.md` file")
                {
                    println!("{err}");
                }

                Ok(())
            })
            .filter_map(|res| res.err())
            .collect(),
        Action::RunCheck => {
            let mut has_diff_any = false;

            let errors = output
                .readmes
                // not par_iter because we want the diffs to be in a determined order
                .iter()
                .map(|readme| -> Result<()> {
                    let new_readme = read_readme_file(&world, readme)?;

                    let current_readme = fs::read_to_string(&readme.path)
                        .context("failed to read `README.md` file")?;

                    let current_readme = current_readme.trim_end();

                    let new_readme = cargo_reedme::insert_into_readme::normalize_new_readme(
                        current_readme,
                        &new_readme,
                    );

                    let diff = diff_file::make_diff(current_readme, &new_readme, 3);

                    let display_path = if let Ok(cwd) = std::env::current_dir()
                        && let Ok(path) = readme.path.strip_prefix(cwd)
                    {
                        path.to_path_buf()
                    } else {
                        readme.path.clone()
                    };

                    let has_diff = !diff.is_empty();

                    if has_diff {
                        diff_file::print_diff(
                            diff,
                            |line_num| format!("\n\ndiff in {display_path}:{line_num}:\n"),
                            cli.color,
                        );
                        has_diff_any = has_diff;
                    }

                    Ok(())
                })
                .filter_map(|res| res.err())
                .collect();

            if has_diff_any {
                bail!(
                    "README files need to be updated (with https://github.com/nik-rev/cargo-reedme)"
                )
            } else {
                println!("all READMEs are up-to-date!")
            }

            errors
        }
    };

    report_errors(errors);

    Ok(())
}

/// Reports errors, exits if there are any
fn report_errors(mut errors: Vec<eyre::Report>) {
    // Errors are sorted by their display, so we show the same errors at the same time
    errors.sort_by_key(|err| err.to_string());

    // Let's report each individual error rather than just the first one
    for err in &errors {
        eprintln!("{err:?}");
    }

    if !errors.is_empty() {
        // We encountered errors, reported, exit with a non-zero exit code
        std::process::exit(1);
    }
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

fn read_readme_file(world: &World, readme: &cargo_reedme::GeneratedReadme) -> Result<String> {
    readme.file.to_readme(world, &readme.config).ok_or_else(|| {
        docstr!(eyre::format_err!
            /// can't figure out where to insert generated content in: {}
            ///
            /// please add `<!-- cargo-reedme -->` somewhere in your README, as that's where
            /// the generated portion from rustdoc comments will be inserted!
            readme.path,
        )
    })
}
