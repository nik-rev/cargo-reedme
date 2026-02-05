use std::ops::Range;

use docstr::docstr;
use pulldown_cmark::CowStr;
use rangemap::RangeSet;

/// Locates reference link definition with the given `link_id` and `link_destination`,
/// in `markdown`, returns span of the link URL
///
/// # Example
///
/// With an id of `"rust_compiler"` and a destination of `"https://github.com/rust-lang/rust"`,
/// and the following markdown input -- span of the highlighted section is returned:
///
/// ```markdown
/// The Rust compiler can be [found][rust compiler] on GitHub
///
/// [rust compiler]: <https://github.com/rust-lang/rust> "link title"
///                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub fn reference_link_definition(
    markdown: &str,
    link_destination: CowStr<'_>,
    link_id: CowStr<'_>,
    code_blocks: &RangeSet<usize>,
) -> Option<Range<usize>> {
    // Must escape since we'll interpolate them into the regex.
    // If they contain special characters, our regex will be messed up
    let link_destination = regex::escape(&link_destination);
    let link_id = regex::escape(&link_id);

    let regex = docstr! { format!
        /// # Start of line
        ///
        /// ^
        ///
        /// # Non-greedily match common container prefixes:
        /// # spaces, blockquotes (>), or list bullets (*, -, +, 1.)
        ///
        /// [\s>*\-+0-9.]*?
        ///
        /// # The label and colon, case doesn't matter
        ///
        /// \[(?i:{link_id})\]:
        ///
        /// # Optional whitespace before the url
        ///
        /// \s*
        ///
        /// # Non-capturing group to match either <URL> or URL,
        /// # capturing just the URL itself in group 1 or 2.
        ///
        /// (?:<({link_destination})>|({link_destination}))
    };

    // Compile the regex
    let regex = regex::RegexBuilder::new(&regex)
        .multi_line(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();

    // Only care about the first capture because in markdown, the
    // first reference link will be used if multiple reference links
    // with the same ID are present. Hence the `find`
    regex.captures_iter(markdown).find_map(|capture| {
        // Either the `<link>`, or a regular `link`
        let match_ = capture.get(1).or_else(|| capture.get(2))?;

        if code_blocks.overlaps(&match_.range()) {
            return None;
        }

        Some(match_.range().clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Status {
        Found,
        Missing,
    }

    #[track_caller]
    fn t(
        link_location: Status,
        id: impl Into<String>,
        url: impl Into<String>,
        md: impl Into<String>,
    ) {
        let md = md.into();
        let url = url.into();

        let ranges = pulldown_cmark::Parser::new(&md)
            .into_offset_iter()
            .filter_map(|(event, span)| {
                matches!(
                    event,
                    pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(_))
                )
                .then_some(span)
            })
            .collect();

        let url_span =
            reference_link_definition(&md, url.clone().into(), id.into().into(), &ranges);

        match link_location {
            Status::Found => {
                let url_span = url_span.expect("link not found");
                assert_eq!(md[url_span], url);
            }
            Status::Missing => assert!(url_span.is_none()),
        };
    }

    /// Bare URL
    #[test]
    fn url() {
        let id = "rust";
        let url = "https://rust-lang.org";

        t(
            Status::Found,
            "rust",
            "https://rust-lang.org",
            docstr!(format!
                /// Check [{id}].
                ///
                /// [{id}]: {url}
            ),
        );
    }

    /// URL angle brackets are ignored
    #[test]
    fn bracketed_url() {
        let id = "rust";
        let url = "https://rust-lang.org";

        t(
            Status::Found,
            id,
            url,
            docstr!(format!
                /// [{id}]: <{url}>
            ),
        );
    }

    /// Contents of code block are ignored when searching for links
    #[test]
    fn skip_code_block() {
        let id = "rust";
        let url = "https://rust-lang.org";

        t(
            Status::Missing,
            id,
            url,
            docstr!(format!
                /// ```
                /// [{id}]: {url}
                /// ```
            ),
        );
    }

    /// Comparisons between link IDs is case-insensitive
    #[test]
    fn case_insensitivity() {
        let id = "RUST_COMPILER";
        let url = "https://github.com/rust-lang/rust";
        t(
            Status::Found,
            "rust_compiler",
            url,
            docstr!(format!
                /// [{id}]: {url}
            ),
        );
    }

    /// Links that have a title
    #[test]
    fn title() {
        let id = "rust";
        let url = "https://rust-lang.org";
        t(
            Status::Found,
            id,
            url,
            docstr!(format!
                /// [{id}]: {url} (A link title)
            ),
        );
        t(
            Status::Found,
            id,
            url,
            docstr!(format!
                /// [{id}]: {url} 'A link title'
            ),
        );
        t(
            Status::Found,
            id,
            url,
            docstr!(format!
                /// [{id}]: {url} "A link title with \"escapes\""
            ),
        );
    }

    /// Nested in various Markdown elements
    #[test]
    fn nested() {
        let id = "rust";
        let url = "https://rust-lang.org";

        // Blockquote, list
        t(
            Status::Found,
            id,
            url,
            docstr!(format!
                /// > [{id}]: {url}
            ),
        );

        // Space, blockquote, list
        t(
            Status::Found,
            id,
            url,
            docstr!(format!
                ///  > 1. [{id}]: {url}
            ),
        );
    }
}
