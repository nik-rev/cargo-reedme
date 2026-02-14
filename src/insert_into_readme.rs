use docstr::docstr;
use eyre::{ContextCompat as _, Result};

const MARKER_INSERT: &str = "<!-- cargo-reedme -->";
const MARKER_INSERT_START: &str = "<!-- cargo-reedme: start -->";
const MARKER_INSERT_END: &str = "<!-- cargo-reedme: end -->";

pub fn insert_into_readme(original_readme: &str, to_insert: &str) -> Result<String> {
    let (before, after) = original_readme
        .split_once(MARKER_INSERT)
        .or_else(|| {
            original_readme
                .split_once(MARKER_INSERT_START)
                .and_then(|(before, remaining)| {
                    remaining
                        .split_once(MARKER_INSERT_END)
                        .map(|(_previously_inserted_by_us, after)| (before, after))
                })
        })
        .with_context(|| {
            format!(
                concat!(
                    "please add `{}` somewhere in your README, that's where",
                    " the generated portion from rust doc comments will be inserted!"
                ),
                MARKER_INSERT
            )
        })?;

    let new_readme = docstr!(format!
        /// {before}{MARKER_INSERT_START}
        ///
        /// {to_insert}
        ///
        /// {MARKER_INSERT_END}{after}
    );

    Ok(new_readme)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    use super::*;

    #[test]
    fn missing_marker() {
        _ = super::insert_into_readme(
            docstr!(
                /// Header
                ///
                /// Footer
            ),
            "...",
        )
        .unwrap_err();
    }

    #[test]
    fn insert() {
        let output = super::insert_into_readme(
            docstr!(
                /// Header
                ///
                /// <!-- cargo-reedme -->
                ///
                /// Footer
            ),
            "hello world",
        )
        .unwrap();

        assert_str_eq!(
            output,
            docstr!(
                /// Header
                ///
                /// <!-- cargo-reedme: start -->
                ///
                /// hello world
                ///
                /// <!-- cargo-reedme: end -->
                ///
                /// Footer
            )
        );
    }

    #[test]
    fn update() {
        let output = super::insert_into_readme(
            docstr!(
                /// Header
                ///
                /// <!-- cargo-reedme: start -->
                ///
                /// hello world
                ///
                /// <!-- cargo-reedme: end -->
                ///
                /// Footer
            ),
            "goodbye moon",
        )
        .unwrap();

        assert_str_eq!(
            output,
            docstr!(
                /// Header
                ///
                /// <!-- cargo-reedme: start -->
                ///
                /// goodbye moon
                ///
                /// <!-- cargo-reedme: end -->
                ///
                /// Footer
            )
        );
    }
}
