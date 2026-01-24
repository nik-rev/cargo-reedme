use core::fmt;
use std::borrow::Cow;

use itertools::Itertools;
use pulldown_cmark::LinkType;

use crate::{intralinks::Links, replace_content::ReplaceContent};

/// Given a `markdown` string:
///
/// - Resolves all `links` in it
/// - Strips rustdoc-specific tags from code fences, such as "edition2024,compile_fail"
/// - Labels code blocks without a language as "Rust"
pub fn resolve_markdown(markdown: &str, links: Links<'_>) -> String {
    let replacements = pulldown_cmark::Parser::new_with_broken_link_callback(
        markdown,
        pulldown_cmark::Options::all(),
        Some(|broken_link: pulldown_cmark::BrokenLink<'_>| {
            let url = links.get(&*broken_link.reference)?;
            let url = match crate::intralinks::link_fragment(&broken_link.reference) {
                None => url.to_string().into(),
                Some(fragment) => format!("{url}#{fragment}").into(),
            };
            Some((url, "".into()))
        }),
    )
    .into_offset_iter()
    .filter_map(|(event, span)| match event {
        // Code blocks are transformed to use Rust language, and
        // hidden lines are removed
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(code_block_kind)) => {
            // Only consider code blocks that contain Rust from here on out
            match code_block_kind {
                // Indented code blocks are ignored
                pulldown_cmark::CodeBlockKind::Indented => return None,
                pulldown_cmark::CodeBlockKind::Fenced(tags) => {
                    if !is_rust_code_block(&tags) {
                        return None;
                    }
                }
            }

            // How many backticks this code block has (3+)
            //
            // Usually 3, sometimes 4 if this code block has nested code blocks
            let backtick_count = markdown[span.clone()]
                .chars()
                .take_while(|ch| *ch == '`')
                .count();
            // The code fence itself: ```
            let code_fence = "`".repeat(backtick_count);
            let code_block_lines = markdown[span.clone()].lines().collect_vec();

            // Remove the first line (```compile_error) and last line (```) of the code blocks,
            // both of which are the code fences
            //
            // ```compile_error
            // code
            // ```
            //
            // We are left with just:
            //
            // code
            let code_block_content_lines =
                &code_block_lines[1..code_block_lines.len().saturating_sub(1)];

            // Remove all commented lines - lines that start with a `#`
            //
            // ```
            // # a
            // b
            // ```
            //
            // Becomes:
            //
            // ```rust
            // b
            // ```
            let code_block_content = code_block_content_lines
                .iter()
                .filter_map(|line| {
                    // Lines starting with `##` are not comments, that is a way to intentionally start a
                    // line with `#`.  See https://github.com/rust-lang/rust/pull/41785.
                    if line.starts_with("##") {
                        Some(Cow::Owned(format!("#{}", &line[1..])))
                    }
                    // Line is commented and will be removed
                    else if line.trim_start().starts_with("# ") || line.trim() == "#" {
                        None
                    } else {
                        Some((*line).into())
                    }
                })
                .join("\n");

            // The full modified code block
            let code_block = format!("{code_fence}rust\n{code_block_content}\n{code_fence}");

            Some(ReplaceContent {
                range: span,
                content: code_block,
            })
        }
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            match link_type {
                // Only rewrite inline links of the form [text](destination)
                LinkType::Inline => {}
                // This was a broken link, but we fixed it
                LinkType::ShortcutUnknown => {
                    debug_assert!(title.is_empty());

                    return Some(ReplaceContent {
                        range: span.clone(),
                        content: format!("[{link_content}]({dest_url})", link_content = id),
                    });
                }
                _ => return None,
            }

            // Present only for other link types.
            debug_assert!(id.is_empty());

            // [text](destination)
            // ^^^^^^^^^^^^^^^^^^^
            let markdown_link = &markdown[span.clone()];

            // [text](destination)
            //  ^^^^
            let link_content = &markdown_link[1..markdown_link.len()
                - ')'.len_utf8()
                - dest_url.len()
                - '('.len_utf8()
                - ']'.len_utf8()];

            let new_destination = links.get(&*dest_url)?;

            Some(ReplaceContent {
                range: span.clone(),
                content: format!(
                    "[{link_content}]({new_destination}{})",
                    fmt::from_fn(|f| {
                        if title.is_empty() {
                            Ok(())
                        } else {
                            f.write_fmt(format_args!(r#" "{title}""#))
                        }
                    })
                ),
            })
        }
        _ => None,
    });
    ReplaceContent::replace_all(markdown.to_string(), replacements)
}

/// If this markdown fence language can be considered to be a "rust" language
///
/// All attributes: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes
pub fn is_rust_code_block(tags: &str) -> bool {
    tags.split(',').all(|tag| {
        tag.is_empty()
            || matches!(
                tag,
                "should_panic"
                    | "no_run"
                    | "ignore"
                    | "allow_fail"
                    | "rust"
                    | "rs"
                    | "test_harness"
                    | "standalone_crate"
                    | "compile_fail"
            )
            || tag.starts_with("ignore-")
            || tag.starts_with("edition")
    })
}

#[cfg(test)]
mod tests {
    use docstr::docstr;
    use pretty_assertions::assert_str_eq;

    use super::*;

    #[rust_analyzer::macro_style(braces)]
    macro_rules! links {
        ($($a:literal => $b:literal),* $(,)?) => {{
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($a.into(), $b.into());
            )*
            hm
        }};
    }

    #[track_caller]
    fn t(input: &str, links: Links<'_>, expected: &str) {
        let out = resolve_markdown(input, links);
        assert_str_eq!(out, expected);
    }

    #[test]
    fn valid_broken_link() {
        t(
            docstr! {
                /// just a [link]
            },
            links! {
                "link" => "to somewhere"
            },
            docstr! {
                /// just a [link](to somewhere)
            },
        );
    }

    #[test]
    fn invalid_broken_link() {
        t(
            docstr! {
                /// just a [link]
            },
            links! {
                "that does not exist" => "..."
            },
            docstr! {
                /// just a [link]
            },
        );
    }

    #[test]
    fn valid_inline_link() {
        t(
            docstr! {
                /// just a [link](where? "title")
            },
            links! {
                "where?" => "to somewhere"
            },
            docstr! {
                /// just a [link](to somewhere "title")
            },
        );
    }

    #[test]
    fn invalid_inline_link() {
        t(
            docstr! {
                /// just a [link](where? "title")
            },
            links! {
                "no exist" => "..."
            },
            docstr! {
                /// just a [link](where? "title")
            },
        );
    }

    #[test]
    fn label() {
        t(
            docstr! {
                /// just a [link]
                ///
                /// [link]: somewhere
            },
            links! {
                "somewhere" => "but where?"
            },
            docstr! {
                /// just a [link]
                ///
                /// [link]: but where?
            },
        );
    }

    #[test]
    fn is_rust_code_block() {
        let pass = [
            "ignore",
            "should_panic",
            "no_run",
            "compile_fail",
            "edition2018",
            "rust",
            "rs",
            "standalone_crate",
            "ignore-x86_64",
            "ignore-x86_64,ignore-windows",
            "ignore,ignore-x86_64",
        ]
        .into_iter()
        .all(super::is_rust_code_block);

        assert!(pass);
    }

    #[test]
    fn is_rust_code_block_fail() {
        let fail = ["py", "js", "ts", "toml"]
            .into_iter()
            .all(|s| !super::is_rust_code_block(s));

        assert!(fail);
    }
}
