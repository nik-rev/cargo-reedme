//! Handles logic for inserting generated README files into existing README files

use std::fmt;

use docstr::docstr;
use eyre::{ContextCompat as _, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ReadmeContents {
    /// The README file did not exist before, so we created it
    ///
    /// Contains full README contents from documentation comments
    NewlyCreated(String),
    /// README file already existed, and we inserted our generated README in the middle of it
    InsertedIntoExisting(ReadmeParts),
}

impl fmt::Display for ReadmeContents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadmeContents::NewlyCreated(s) => {
                f.write_fmt(format_args!("{}\n\n", ReadmeParts::MARKER_INSERT_START))?;
                f.write_fmt(format_args!("{s}\n\n"))?;
                f.write_fmt(format_args!("{}\n", ReadmeParts::MARKER_INSERT_END))?;
                Ok(())
            }
            ReadmeContents::InsertedIntoExisting(s) => s.fmt(f),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReadmeParts {
    before: String,
    inserted: String,
    after: String,
}

impl ReadmeParts {
    const MARKER_INSERT: &'static str = "<!-- cargo-reedme -->";
    const MARKER_INSERT_START: &'static str = "<!-- cargo-reedme: start -->";
    const MARKER_INSERT_END: &'static str = "<!-- cargo-reedme: end -->";

    pub fn new(original_readme: &str, to_insert: &str) -> Result<Self> {
        let (before, after) = original_readme
            .split_once(Self::MARKER_INSERT)
            .or_else(|| {
                original_readme
                    .split_once(Self::MARKER_INSERT_START)
                    .and_then(|(before, remaining)| {
                        remaining
                            .split_once(Self::MARKER_INSERT_END)
                            .map(|(_previously_inserted_by_us, after)| (before, after))
                    })
            })
            .with_context(|| {
                format!(
                    concat!(
                        "please add `{}` somewhere in your README, that's where",
                        " the generated portion from rustdoc comments will be inserted!"
                    ),
                    Self::MARKER_INSERT
                )
            })?;

        Ok(Self {
            before: before.to_string(),
            inserted: to_insert.to_string(),
            after: after.to_string(),
        })
    }
}

impl fmt::Display for ReadmeParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(docstr!(format_args!
            /// {}{}
            ///
            /// {}
            ///
            /// {}{}
            self.before, Self::MARKER_INSERT_START, self.inserted, Self::MARKER_INSERT_END, self.after
        ))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    use super::*;

    fn t(original: &str, insert: &str) -> Result<String> {
        ReadmeParts::new(original, insert).map(|s| s.to_string())
    }

    #[test]
    fn missing_marker() {
        _ = t(
            docstr!(
                /// Header
                ///
                /// Footer
            ),
            "...",
        )
        .unwrap_err();
    }

    /// Creating README file for the first time will insert markers,
    /// so subsequent invocations won't error
    #[test]
    fn create() {
        assert_str_eq!(
            ReadmeContents::NewlyCreated(
                docstr!(
                    /// first
                )
                .to_string(),
            )
            .to_string(),
            docstr!(
                /// <!-- cargo-reedme: start -->
                ///
                /// first
                ///
                /// <!-- cargo-reedme: end -->
                ///
            )
        );
    }

    #[test]
    fn insert() {
        let output = t(
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
        let output = t(
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
