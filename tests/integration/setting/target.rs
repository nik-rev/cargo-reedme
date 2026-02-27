use super::*;

/// Prefers lib.rs over main.rs
#[test]
fn target_heuristic_lib_rs() -> Result<()> {
    test(Case {
        main_rs: Some(docstr!(
            /// //! bin
        )),
        lib_rs: Some(docstr!(
            /// //! lib
        )),
        generated: docstr!(
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
        main_rs: Some(docstr!(
            /// //! bin
        )),
        lib_rs: Some(docstr!(
            ///
        )),
        generated: docstr!(
            /// bin
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_heuristic_main_rs() -> Result<()> {
    test(Case {
        main_rs: Some(docstr!(
            /// //! bin
        )),
        generated: docstr!(
            /// bin
        ),
        ..
    })?;
    Ok(())
}

#[test]
fn target_heuristic_lib_rs_only() -> Result<()> {
    test(Case {
        lib_rs: Some(docstr!(
            /// //! lib
        )),
        generated: docstr!(
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
        generated: docstr!(
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
        generated: docstr!(
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
        generated: docstr!(
            /// first
        ),
        init: Some(init()),
        ..
    })?;
    test(Case {
        config: Some(docstr!(
            /// target = "bin:second"
        )),
        generated: docstr!(
            /// second
        ),
        init: Some(init()),
        ..
    })?;
    Ok(())
}
