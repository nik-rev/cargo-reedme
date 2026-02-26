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

#[track_caller]
fn test_link(link: &str, item: &str, result: &str) -> Result<()> {
    test_link_full(
        link,
        item,
        &format!("https://docs.rs/test_case/0.0.0/test_case/{result}"),
    )
}

#[track_caller]
fn test_link_full(link: &str, item: &str, result: &str) -> Result<()> {
    test(Case {
        lib_rs: Some(&docstr!(format!
            /// //! {link}
            ///
            /// {item}
        )),
        readme: &docstr!(format!
            /// {link}({result})
        ),
        ..
    })
}

#[test]
fn kind_field() -> Result<()> {
    test_link(
        "[X::y]",
        "struct X { y: i32 }",
        "struct.X.html#structfield.y",
    )?;
    test_link("[X::y]", "union X { y: () }", "union.X.html#structfield.y")?;
    Ok(())
}

#[test]
fn kind_method() -> Result<()> {
    test_link(
        "[X::y]",
        "struct X; impl X { fn y(&self) {} }",
        "struct.X.html#method.y",
    )?;
    test_link(
        "[X::y]",
        "enum X { A } impl X { fn y(&self) {} }",
        "enum.X.html#method.y",
    )?;
    Ok(())
}

#[test]
fn trait_items() -> Result<()> {
    // The trait method definition
    test_link(
        "[X::y]",
        "trait X { fn y(&self); }",
        "trait.X.html#tymethod.y",
    )?;
    // An associated type within a trait
    test_link(
        "[X::Y]",
        "trait X { type Y; }",
        "trait.X.html#associatedtype.Y",
    )?;
    // An associated constant within a trait
    test_link(
        "[X::Y]",
        "trait X { const Y: i32; }",
        "trait.X.html#associatedconstant.Y",
    )?;
    Ok(())
}

#[test]
fn associated_fn() -> Result<()> {
    test_link(
        "[X::y]",
        "struct X; impl X { fn y() {} }",
        "struct.X.html#method.y",
    )?;
    Ok(())
}

#[test]
fn link_enum_variant() -> Result<()> {
    test_link("[E::A]", "enum E { A }", "enum.E.html#variant.A")?;
    Ok(())
}

#[test]
fn link_struct_field() -> Result<()> {
    test_link(
        "[S::f]",
        "struct S { f: i32 }",
        "struct.S.html#structfield.f",
    )?;
    Ok(())
}

#[test]
fn link_union_field() -> Result<()> {
    test_link("[U::f]", "union U { f: i32 }", "union.U.html#structfield.f")?;
    Ok(())
}

#[test]
fn link_method() -> Result<()> {
    test_link(
        "[S::m]",
        "struct S; impl S { fn m(&self) {} }",
        "struct.S.html#method.m",
    )?;
    Ok(())
}

#[test]
fn link_assoc_fn() -> Result<()> {
    test_link(
        "[S::f]",
        "struct S; impl S { fn f() {} }",
        "struct.S.html#method.f",
    )?;
    Ok(())
}

#[test]
fn link_assoc_const() -> Result<()> {
    test_link(
        "[S::C]",
        "struct S; impl S { const C: i32 = 1; }",
        "struct.S.html#associatedconstant.C",
    )?;
    Ok(())
}

#[test]
fn link_trait_method() -> Result<()> {
    test_link(
        "[T::m]",
        "trait T { fn m(&self); }",
        "trait.T.html#tymethod.m",
    )?;
    Ok(())
}

#[test]
fn link_trait_assoc_type() -> Result<()> {
    test_link(
        "[T::A]",
        "trait T { type A; }",
        "trait.T.html#associatedtype.A",
    )?;
    Ok(())
}

#[test]
fn resolve_ambiguity_fn() -> Result<()> {
    let code = "fn x() {} struct x {}";
    test_link("[fn@x]", code, "fn.x.html")?;
    Ok(())
}

#[test]
fn resolve_ambiguity_struct() -> Result<()> {
    let code = "fn x() {} struct x {}";
    test_link("[struct@x]", code, "struct.x.html")?;
    Ok(())
}

#[test]
fn resolve_ambiguity_macro() -> Result<()> {
    let code = "fn x() {} #[macro_export] macro_rules! x { () => {} }";
    test_link("[x!]", code, "macro.x.html")?;
    test_link("[macro@x]", code, "macro.x.html")?;
    Ok(())
}

#[test]
fn resolution_by_suffix() -> Result<()> {
    test_link("[x()]", "fn x() {}", "fn.x.html")?;
    test_link(
        "[x!]",
        "#[macro_export] macro_rules! x { () => {} }",
        "macro.x.html",
    )?;
    Ok(())
}

#[test]
fn link_module() -> Result<()> {
    test_link("[m]", "mod m {}", "m/")?;
    Ok(())
}

#[test]
fn link_nested_item() -> Result<()> {
    test_link("[m::x]", "mod m { pub fn x() {} }", "m/fn.x.html")?;
    Ok(())
}

#[test]
fn primitive_link() -> Result<()> {
    test_link_full(
        "[u32]",
        "",
        "https://doc.rust-lang.org/stable/std/primitive.u32.html",
    )?;
    test_link_full(
        "[slice]",
        "",
        "https://doc.rust-lang.org/stable/std/primitive.slice.html",
    )?;
    Ok(())
}

#[test]
fn relative_paths() -> Result<()> {
    test_link("[crate::x]", "fn x() {}", "fn.x.html")?;
    test_link("[self::x]", "fn x() {}", "fn.x.html")?;
    Ok(())
}

#[test]
fn tuple_struct_field() -> Result<()> {
    test_link("[S::0]", "struct S(i32);", "struct.S.html#structfield.0")?;
    Ok(())
}

#[test]
fn nested_assoc_item() -> Result<()> {
    let code = "mod m { pub struct S; impl S { pub fn f() {} } }";
    test_link("[m::S::f]", code, "m/struct.S.html#method.f")?;
    Ok(())
}

#[test]
fn generic_path_ignoring() -> Result<()> {
    test_link(
        "[Vec<T>::push]",
        "struct Vec<T> { a: [T] } impl<T> Vec<T> { fn push(&mut self, x: T) {} }",
        "struct.Vec.html#method.push",
    )?;
    Ok(())
}

#[test]
fn base_url() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// base-url = "https://example.com"
        )),
        lib_rs: Some(docstr!(
            /// //! [x]
            ///
            /// fn x() {}
        )),
        readme: &docstr!(format!
            /// [x](https://example.com/test_case/0.0.0/test_case/fn.x.html)
        ),
        ..
    })?;
    Ok(())
}

/// `base-url` Doesn't affect stuff from STD
#[test]
fn base_url_on_std() -> Result<()> {
    test(Case {
        config: Some(docstr!(
            /// base-url = "https://example.com"
        )),
        lib_rs: Some(docstr!(
            /// //! [Option]
        )),
        readme: &docstr!(format!
            /// [Option](https://doc.rust-lang.org/stable/core/option/enum.Option.html)
        ),
        ..
    })?;
    Ok(())
}

/// `base-url` Doesn't affect stuff from external dependencies
/// because they have their own HTML root URL
#[test]
fn base_url_on_external_crate() -> Result<()> {
    test(Case {
        dependencies: Some(docstr!(
            /// serde_core = "=1.0.228"
        )),
        config: Some(docstr!(
            /// base-url = "https://example.com"
        )),
        lib_rs: Some(docstr!(
            /// //! [serde_core::Serialize]
        )),
        readme: &docstr!(format!
            /// [serde_core::Serialize](https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html)
        ),
        ..
    })?;
    Ok(())
}
