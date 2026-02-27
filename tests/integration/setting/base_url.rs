use super::*;

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
        generated: &docstr!(format!
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
        generated: &docstr!(format!
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
        generated: &docstr!(format!
            /// [serde_core::Serialize](https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html)
        ),
        ..
    })?;
    Ok(())
}
