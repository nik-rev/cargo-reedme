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

#[allow(clippy::type_complexity)]
struct Case<'a> {
    config: Option<&'a str> = None,
    lib_rs: Option<&'a str> = None,
    main_rs: Option<&'a str> = None,
    /// Arbitrary initialization logic
    init: Option<Box<dyn Fn(&TempDir) -> Result<()>>> = None,
    readme: &'a str,
}

fn test(
    Case {
        config,
        lib_rs,
        main_rs,
        init,
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
        /// edition = "2024"
        /// name = "test_case"
        ///
        /// {config}
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

/// Prefers lib.rs over main.rs
#[test]
fn target_heuristic_lib_rs() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// target = "heuristic"
        )),
        main_rs: Some(docstr!(
            /// //! bin
        )),
        lib_rs: Some(docstr!(
            /// //! lib
        )),
        readme: docstr!(
            /// lib
        ),
        ..
    })?;
    Ok(())
}

/// If lib.rs is empty, use comments from main.rs
#[test]
fn target_heuristic_lib_rs_empty() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// target = "heuristic"
        )),
        main_rs: Some(docstr!(
            /// //! bin
        )),
        lib_rs: Some(docstr!(
            ///
        )),
        readme: docstr!(
            /// bin
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_heuristic_main_rs() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// target = "heuristic"
        )),
        main_rs: Some(docstr!(
            /// //! bin
        )),
        readme: docstr!(
            /// bin
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_heuristic_lib_rs_only() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// target = "heuristic"
        )),
        lib_rs: Some(docstr!(
            /// //! lib
        )),
        readme: docstr!(
            /// lib
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_lib() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// target = "lib"
        )),
        lib_rs: Some(docstr!(
            /// //! lib
        )),
        main_rs: Some(docstr!(
            /// //! bin
        )),
        readme: docstr!(
            /// lib
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_bin() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// target = "bin"
        )),
        lib_rs: Some(docstr!(
            /// //! lib
        )),
        main_rs: Some(docstr!(
            /// //! bin
        )),
        readme: docstr!(
            /// bin
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_bin_exact() -> Result<()> {
    let init = || {
        Box::new(|dir: &TempDir| {
            dir.child("src/bin/first.rs").write_str(docstr!(
                /// //! first
                ///
                /// fn main() {}
            ))?;
            dir.child("src/bin/second.rs").write_str(docstr!(
                /// //! second
                ///
                /// fn main() {}
            ))?;
            Ok(())
        })
    };

    test(Case {
        config: Some(docstr!(
            /// target = "bin:first"
        )),
        readme: docstr!(
            /// first
        ),
        init: Some(init()),
        ..
    })?;
    test(Case {
        config: Some(docstr!(
            /// target = "bin:second"
        )),
        readme: docstr!(
            /// second
        ),
        init: Some(init()),
        ..
    })?;
    Ok(())
}
