#![feature(default_field_values)]
#![feature(result_option_map_or_default)]
#![feature(exit_status_error)]
use std::process::Command;

use assert_cli::Assert;
use assert_fs::{
    TempDir,
    assert::PathAssert,
    prelude::{FileTouch, FileWriteStr, PathChild},
};
use assert2::check;
use docstr::docstr;
use eyre::Result;

struct Case<'a> {
    config: Option<&'a str> = None,
    lib_rs: Option<&'a str> = None,
    main_rs: Option<&'a str> = None,
    readme: &'a str,
}

fn single_package(
    Case {
        config,
        lib_rs,
        main_rs,
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

    dir.child("Cargo.toml").write_str(&docstr!(format!
        /// [package]
        /// name = "test_case"
        /// {config}
    ))?;

    if let Some(file) = lib_rs {
        dir.child("src/lib.rs").write_str(file)?;
    }

    if let Some(file) = main_rs {
        dir.child("src/main.rs").write_str(file)?;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_cargo-reedme"))
        .arg("reedme")
        .arg("--json")
        .env("RUST_TOOLCHAIN", "nightly")
        .current_dir(dir)
        .output()?
        .exit_ok()?;

    let output: cargo_reedme::Output = serde_json::from_slice(&output.stdout)?;

    let actual_readme = &output.readmes.first().unwrap().file.contents;

    assert!(actual_readme == expected_readme);

    Ok(())
}

#[test]
fn basic() -> Result<()> {
    single_package(Case {
        lib_rs: Some(docstr!(
            /// //! a
        )),
        readme: docstr!(
            /// a
        ),
        ..
    })?;
    Ok(())
}
