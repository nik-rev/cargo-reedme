use super::*;

/// Increments all headings by 1 level
#[test]
fn increment_headings() -> Result<()> {
    test(Case {
        lib_rs: Some(INPUT),
        readme: INCREMENTED,
        init: Some(Box::new(|dir| {
            dir.child("README.md").write_str(docstr!(
                /// # h1
                ///
                /// <!-- cargo-reedme -->
            ))?;
            Ok(())
        })),
        ..
    })?;
    Ok(())
}

/// Does not increment headings if it is explicitly disabled
#[test]
fn no_increment_if_option_is_turned_off() -> Result<()> {
    test(Case {
        lib_rs: Some(INPUT),
        config: Some(docstr!(
            /// increment-headings = false
        )),
        readme: NOT_INCREMENTED,
        init: Some(Box::new(|dir| {
            dir.child("README.md").write_str(docstr!(
                /// # h1
                ///
                /// <!-- cargo-reedme -->
            ))?;
            Ok(())
        })),
        ..
    })?;
    Ok(())
}

/// When the level 1 heading is located AFTER the inserted region,
/// headings are not incremented
#[test]
fn no_increment_when_h1_after() -> Result<()> {
    test(Case {
        lib_rs: Some(INPUT),
        readme: NOT_INCREMENTED,
        init: Some(Box::new(|dir| {
            dir.child("README.md").write_str(docstr!(
                /// <!-- cargo-reedme -->
                ///
                /// # h1
            ))?;
            Ok(())
        })),
        ..
    })?;
    Ok(())
}

/// Does not increment headings when there is no level 1 heading in original README
#[test]
fn no_increment_when_no_h1_in_original() -> Result<()> {
    test(Case {
        lib_rs: Some(INPUT),
        readme: NOT_INCREMENTED,
        init: Some(Box::new(|dir| {
            dir.child("README.md").write_str(docstr!(
                /// ## h2
                ///
                /// <!-- cargo-reedme -->
            ))?;
            Ok(())
        })),
        ..
    })?;
    Ok(())
}

const INPUT: &str = docstr!(
    /// //! # a
    /// //!
    /// //! ## b
    /// //!
    /// //! # c
    /// //!
    /// //! ## d
    /// //!
    /// //! ### e
);

const INCREMENTED: &str = docstr!(
    /// ## a
    ///
    /// ### b
    ///
    /// ## c
    ///
    /// ### d
    ///
    /// #### e
);

const NOT_INCREMENTED: &str = docstr!(
    /// # a
    ///
    /// ## b
    ///
    /// # c
    ///
    /// ## d
    ///
    /// ### e
);
