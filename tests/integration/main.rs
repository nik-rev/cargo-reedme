#![feature(default_field_values)]
#![feature(result_option_map_or_default)]
#![feature(exit_status_error)]

use std::process::Command;

use assert_fs::{
    TempDir,
    prelude::{FileWriteStr, PathChild},
};
use assert2::assert;
use docstr::docstr;
use eyre::Result;

mod link_resolution;
mod setting;

/// A single test case
#[allow(clippy::type_complexity)]
struct Case<'a> {
    dependencies: Option<&'a str> = None,
    /// cargo-reedme specific config
    config: Option<&'a str> = None,
    /// Contents of `lib.rs` file
    lib_rs: Option<&'a str> = None,
    /// Contents of `main.rs` file
    main_rs: Option<&'a str> = None,
    /// Arbitrary initialization logic
    init: Option<Box<dyn Fn(&TempDir) -> Result<()>>> = None,
    /// Expected README file contents
    readme: &'a str,
}

#[track_caller]
fn test(
    Case {
        config,
        lib_rs,
        main_rs,
        init,
        dependencies,
        readme: expected_readme,
    }: Case,
) -> Result<()> {
    let dir = TempDir::new()?.into_persistent();

    let config = config.map_or_default(|config| {
        docstr!(format!
            ///
            /// [package.metadata.cargo-reedme]
            /// {config}
        )
    });

    let dependencies = dependencies.unwrap_or_default();

    dir.child("Cargo.toml").write_str(&docstr!(format!
        /// [package]
        /// edition = "2024"
        /// name = "test_case"
        ///
        /// {config}
        ///
        /// [dependencies]
        /// {dependencies}
    ))?;

    if let Some(file) = lib_rs {
        dir.child("src/lib.rs").write_str(file)?;
    }

    if let Some(file) = main_rs {
        dir.child("src/main.rs").write_str(&docstr!(format!
            /// {file}
            ///
            /// fn main() {{}}
        ))?;
    }

    if let Some(init) = init {
        init(&dir)?;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_cargo-reedme"))
        .arg("reedme")
        .arg("--json")
        .env("RUST_TOOLCHAIN", "nightly")
        .current_dir(dir)
        .output()?;

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8(output.stderr)?
    );

    let output: cargo_reedme::Output = serde_json::from_slice(&output.stdout)?;

    let actual_readme = &output.readmes.first().unwrap().file.contents;

    assert!(actual_readme == expected_readme);

    Ok(())
}
