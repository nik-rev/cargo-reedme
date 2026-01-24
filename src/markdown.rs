use std::borrow::Cow;

use docstr::docstr;
use itertools::Itertools;
use pulldown_cmark::{CowStr, LinkType, Options};
use rangemap::RangeSet;

use crate::{intralinks::Links, replace_content::ReplaceContent};

/// Given a `markdown` string:
///
/// - Resolves all `links` in it
/// - Strips rustdoc-specific tags from code fences, such as "edition2024,compile_fail"
/// - Labels code blocks without a language as "Rust"
pub fn resolve_markdown(markdown: &str, links: Links<'_>) -> String {
    let mut reference_definitions = Vec::new();
    // When searching for reference definitions, anything we find that
    // touches this set will be excluded because it is inside of a code block
    let mut code_block_ranges = RangeSet::new();

    let replacements = pulldown_cmark::Parser::new_with_broken_link_callback(
        markdown,
        markdown_options(),
        Some(|broken_link: pulldown_cmark::BrokenLink<'_>| {
            let url = links.get(&*broken_link.reference)?;
            let url = match crate::intralinks::link_fragment(&broken_link.reference) {
                None => CowStr::Borrowed(url),
                Some(fragment) => format!("{url}#{fragment}").into(),
            };
            Some((url, "".into()))
        }),
    )
    .into_offset_iter()
    .filter_map(|(event, span)| match event {
        // Replace original text with processed text via ENABLE_SMART_PUNCTUATION
        //
        // Replaces -- with —, --- with —, ... with …, "quote" with “quote”, and 'quote' with ‘quote’.
        pulldown_cmark::Event::Text(text) => Some(ReplaceContent {
            range: span,
            content: text,
        }),
        // Code blocks are transformed to use Rust language, and
        // hidden lines are removed
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(code_block_kind)) => {
            code_block_ranges.insert(span.clone());

            // Only consider code blocks that contain Rust from here on out
            match code_block_kind {
                // Indented code blocks are ignored, for now
                pulldown_cmark::CodeBlockKind::Indented => return None,
                // Fenced code blocks: ```rust
                pulldown_cmark::CodeBlockKind::Fenced(tags) => {
                    if !is_rust_code_block(&tags) {
                        return None;
                    }
                }
            }

            // How many backticks this code block has
            //
            // At least - and usually 3, sometimes 4+ if this code block has nested code blocks
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

            Some(ReplaceContent {
                range: span,
                content: format!("{code_fence}rust\n{code_block_content}\n{code_fence}").into(),
            })
        }
        // This was a broken link, but we fixed it with our broken link callback
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) if matches!(
            link_type,
            LinkType::ShortcutUnknown | LinkType::CollapsedUnknown | LinkType::ReferenceUnknown
        ) =>
        {
            debug_assert!(title.is_empty(), "we never insert a title");

            let end = if matches!(link_type, LinkType::CollapsedUnknown) {
                // add +2 to also replace the [] at the end. without this,
                // we will generate "[foo](bar)[]" for "[foo][]" if [foo] links to "bar"
                span.clone().end + 2
            } else {
                span.clone().end
            };

            Some(ReplaceContent {
                range: span.clone().start..end,
                content: format!("[{link_content}]({dest_url})", link_content = id).into(),
            })
        }
        // Reference links:
        //
        // [link][somewhere]
        //
        // [somewhere]: foo
        //
        //
        // Shortcut links:
        //
        // [link]
        //
        // [link]: foo
        //
        //
        // Process:
        //
        // Since we want to output markdown that is as similar to the original input
        // as possible, all we want to do is replace those "foo" links with the correct
        // links obtained from the "links" map.
        //
        // That is tricky, because `pulldown_cmark` does not generate any events
        // for those reference definitions. Their existance is simply erased.
        //
        // So what we do is mostly a hack. We remember every reference ID and its URL,
        // then we do a 2nd search over the entire input to find all reference links.
        //
        // Reference links that have an entry in the map will be replaced
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
            link_type: LinkType::Reference | LinkType::Shortcut,
            dest_url,
            title,
            id,
        }) if !id.is_empty() => {
            reference_definitions.push((dest_url, id));
            None
        }
        // Re-write inline links: [text](destination "title")
        //
        // This is tricky because we need to find the "destination"
        // and replace it with the correct link, so we edit the user's
        // input as little as possible.
        //
        // To do this we need to manually parse from the end of the link
        // until the destination. The title is the one that's hardest, it can
        // contain quotes and parentheses.
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
            link_type: LinkType::Inline,
            dest_url,
            title,
            id,
        }) => {
            // We find span of the destination:
            //
            // [text](destination "title")
            //        ^^^^^^^^^^^
            dbg!(title, id, dest_url);

            None

            // // Present only for other link types.
            // debug_assert!(id.is_empty());

            // // [text](destination "title")
            // // ^^^^^^^^^^^^^^^^^^^^^^^^^^^
            // let markdown_link = &markdown[span.clone()];

            // // Skip past link section

            // let mut nesting_level = 1;
            // let link_chars = markdown_link.char_indices();
            // // skip the first "["
            // let mut link_chars = link_chars.skip(1);
            // // whether the current character is escaped
            // let mut escaped = false;
            // let mut in_quotes = false;

            // let destination_start = loop {
            //     let Some((i, ch)) = link_chars.next() else {
            //         // invalid markdown!
            //         continue;
            //     };

            //     match ch {
            //         '[' if !escaped => {
            //             nesting_level += 1;
            //         }
            //         ']' if !escaped => {
            //             nesting_level -= 1;

            //             // reached end
            //             if nesting_level == 0 {
            //                 break i + ']'.len_utf8() + '('.len_utf8();
            //             }
            //         }
            //         '"' if !escaped => {}
            //         '\\' if !escaped => {
            //             // will be set to "false" on the next iteration
            //             escaped = true;
            //             continue;
            //         }
            //         _ => {}
            //     }

            //     escaped = false;
            // };

            // // [text](destination "title")
            // //        ^^^^^^^^^^^^^^^^^^^^
            // let destination_start_index = markdown_link
            //     .find("](")
            //     .expect("invalid markdown link; parser would fail")
            //     + ']'.len_utf8()
            //     + '('.len_utf8();

            // // [text](destination  "title"  )
            // //                              ^
            // let link_end_index = markdown_link
            //     .rfind(')')
            //     .expect("invalid markdown link; parser would fail");

            // let mut destination_end_index = link_end_index;

            // let mut chars = markdown_link[destination_start_index..link_end_index]
            //     .char_indices()
            //     .rev()
            //     .peekable();

            // while let Some((i, ch)) = chars.next() {
            //     match ch {
            //         ' ' => {}
            //         // This is a ", NOT an escaped \"
            //         //
            //         // Important since \" is actually part of the title itself.
            //         '"' if chars.peek().expect("invalid markdown").1 != '\\' => {}
            //         _ => break,
            //     }
            // }

            // // Adjust it to be relative to the entire markdown input

            // let destination_start_index = destination_start_index + span.start;
            // let destination_end_index = destination_end_index + span.start;

            // let new_destination = links.get(&*dest_url)?;

            // Some(ReplaceContent {
            //     range: destination_start_index..destination_end_index,
            //     content: new_destination.to_string(),
            // })
        }
        _ => None,
    });

    let markdown = ReplaceContent::replace_all(markdown.to_string(), replacements);

    let replacements = reference_definitions.into_iter().filter_map(|(dest, id)| {
        let new_url = links.get(&*dest)?;

        let dest = regex::escape(&dest);
        let id = regex::escape(&id);

        // Regex for reference definitions.
        //
        // Reference definitions look like this:
        //
        // [id]: dest "optional title"
        //
        // They must be at the start of the line, they may be preceded
        // by whitespace, they may be inside of list items and blockquotes
        let regex = docstr! { format!
            /// ^                         # Start of line
            ///
            /// \[\s>*\-+0-9.]*?          # Non-greedily match common container prefixes:
            ///                           # spaces, blockquotes (>), or list bullets (*, -, +, 1.)
            ///
            /// \[(?i:{id})\]:            # The label and colon
            ///
            /// \s*                       # Optional whitespace before the url
            ///
            /// (?:<({dest})>|({dest}))   # Non-capturing group to match either <URL> or URL,
            ///                           # capturing just the URL itself in group 1 or 2.
        };
        let regex = regex::RegexBuilder::new(&regex)
            .multi_line(true)
            .ignore_whitespace(true)
            .build()
            .unwrap();

        // Only care about the first one, because markdown only uses the first
        // reference link definition if multiple are present
        let capture = regex.captures_iter(&markdown).next()?;

        // Group 1 is <url>, Group 2 is the raw url
        let match_ = capture.get(1).or_else(|| capture.get(2))?;

        // This definition is part of a code block.
        if code_block_ranges.overlaps(&match_.range()) {
            return None;
        }

        Some(ReplaceContent {
            range: match_.range().clone(),
            content: CowStr::Borrowed(new_url),
        })
    });

    ReplaceContent::replace_all(markdown.to_string(), replacements)
}

/// If this markdown fence language can be considered to be a "rust" language
///
/// All attributes: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes
fn is_rust_code_block(tags: &str) -> bool {
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

/// Keep same options as what rustdoc enables: <https://github.com/rust-lang/rust/blob/021fc25b7a48f6051bee1e1f06c7a277e4de1cc9/src/librustdoc/html/markdown.rs#L68-L74>
fn markdown_options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION
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

    // #[test]
    // fn link_destination() {
    //     let a = docstr! {
    //         /// [link](/uri "title")"
    //         ///        ^^^^
    //         ///
    //         /// [link](/uri "title")
    //         ///        ^^^^
    //         ///
    //         /// [link](/uri)
    //         ///        ^^^^
    //         ///
    //         /// [](./target.md)
    //         ///    ^^^^^^^^^^^
    //         ///
    //         /// [link]()
    //         ///       ^
    //         ///
    //         /// [link](<>)
    //         ///        ^
    //         ///
    //         /// []()
    //         ///   ^
    //         ///
    //         /// [link](/my uri)
    //         ///        ^^^^^^^
    //         ///
    //         /// [link](</my uri>)
    //         ///        ^^^^^^^^^
    //         ///
    //         /// [a](<b)c>)
    //         ///     ^^^^^
    //         ///
    //         /// [link](\(foo\))
    //         ///        ^^^^^^^
    //         ///
    //         /// [link](foo(and(bar)))
    //         ///        ^^^^^^^^^^^^^^
    //         ///
    //         /// [link](foo\(and\(bar\))
    //         ///        ^^^^^^^^^^^^^^^^
    //         ///
    //         /// [link](<foo(and(bar)>)
    //         ///        ^^^^^^^^^^^^^^
    //         ///
    //         /// r"[link](foo\)\:)";
    //         ///
    //         /// // Titles may be in single quotes, double quotes, or parentheses:
    //         ///
    //         /// r#"[link](/url "title")"#;
    //         /// "[link](/url 'title')";
    //         /// "[link](/url (title))";
    //     };

    //     // Parentheses and other symbols can also be escaped, as usual in Markdown:
    //     r"[link](foo\)\:)";
    //     // Titles may be in single quotes, double quotes, or parentheses:
    //     r#"[link](/url "title")"#;
    //     "[link](/url 'title')";
    //     "[link](/url (title))";
    //     // Backslash escapes and entity and numeric character references may be used in titles:
    //     r#"[link](/url "title \"&quot;")"#;
    //     // quotes can be mixed
    //     r#"[link](/url 'title "and" title')"#;
    //     // Spaces, tabs, and up to one line ending is allowed around the destination and title:
    //     "[link](\t/uri\n\"title\"\t)";
    //     // The link text may contain balanced brackets, but not unbalanced ones, unless they are escaped:
    //     "[link [foo [bar]]](/uri)";
    //     r"[link \[bar](/uri)";
    //     // The link text may contain inline content:
    //     "[link *foo **bar** `#`*](/uri)";
    //     "[![moon](moon.jpg)](/uri)";

    //     ///
    //     /// [link](foo\)\:)
    //     ///        ^^^^^^^^
    //     ///
    //     /// [link](/url "title")
    //     ///        ^^^^
    //     ///
    //     /// [link](/url 'title')
    //     ///        ^^^^
    //     ///
    //     /// [link](/url (title))
    //     ///        ^^^^
    //     ///
    //     /// [link](/url "title \"&quot;")
    //     ///        ^^^^
    //     ///
    //     /// [link](/url 'title "and" title')
    //     ///        ^^^^
    //     ///
    //     /// [link](	/uri
    //     ///        "title"	)
    //     ///        ^^^^
    //     ///
    //     /// [link [foo [bar]]](/uri)
    //     ///                      ^^^^
    //     ///
    //     /// [link \[bar](/uri)
    //     ///              ^^^^
    //     ///
    //     /// [link *foo **bar** `#`*](/uri)
    //     ///                           ^^^^
    //     ///
    //     /// [![moon](moon.jpg)](/uri)
    //     ///                     ^^^^
    // }

    #[track_caller]
    fn t(input: &str, links: Links<'_>, expected: &str) {
        let out = resolve_markdown(input, links);
        assert_str_eq!(out, expected);
    }

    #[test]
    fn intra_doc_broken_link() {
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
    fn regular_broken_link() {
        t(
            docstr! {
                /// just a [link]
            },
            links! {
                "..." => "..."
            },
            docstr! {
                /// just a [link]
            },
        );
    }

    #[test]
    fn intra_doc_inline_link() {
        t(
            docstr! {
                /// just a [link](where?)
            },
            links! {
                "where?" => "to somewhere"
            },
            docstr! {
                /// just a [link](to somewhere)
            },
        );

        // Now there are 3X as many links to resolve
        t(
            docstr! {
                /// just a [link](where?)
                /// just a [link](where?)
                /// just a [link](where?)
            },
            links! {
                "where?" => "to somewhere"
            },
            docstr! {
                /// just a [link](to somewhere)
                /// just a [link](to somewhere)
                /// just a [link](to somewhere)
            },
        );

        // Now there are 2 different links, and 1 of them is duplicated
        t(
            docstr! {
                /// just a [link](where?)
                /// just a [url](what?)
                /// just a [link](where?)
            },
            links! {
                "where?" => "to somewhere",
                "what?" => "because"
            },
            docstr! {
                /// just a [link](to somewhere)
                /// just a [url](what?)
                /// just a [link](because)
            },
        );
    }

    /// Test link titles with different kinds of whitespace on left and right
    #[test]
    fn intra_doc_inline_link_title() {
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

        // 3 spaces at the front
        t(
            docstr! {
                /// just a [link](where?   "title")
            },
            links! {
                "where?" => "to somewhere"
            },
            docstr! {
                /// just a [link](to somewhere   "title")
            },
        );

        // 2 spaces at the front and at the back
        t(
            docstr! {
                /// just a [link](where?  "title"  )
            },
            links! {
                "where?" => "to somewhere"
            },
            docstr! {
                /// just a [link](to somewhere  "title"  )
            },
        );
    }

    #[test]
    fn regular_inline_link() {
        t(
            docstr! {
                /// just a [link](where?)
            },
            links! {
                "..." => "..."
            },
            docstr! {
                /// just a [link](where?)
            },
        );
    }

    #[test]
    fn intra_doc_reference_link() {
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
    fn regular_doc_reference_link() {
        t(
            docstr! {
                /// just a [link]
                ///
                /// [link]: somewhere
            },
            links! {
                "..." => "..."
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
