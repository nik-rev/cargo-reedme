#![allow(unstable_name_collisions)]

use std::borrow::Cow;

use itertools::Itertools;
use pulldown_cmark::{CowStr, LinkType, Options};
use rangemap::RangeSet;

use crate::{intralinks::Links, replace_content::ReplaceContent};

mod locate;

/// Given a `markdown` string:
///
/// - Resolves all `links` in it
/// - Strips rustdoc-specific tags from code fences, such as "edition2024,compile_fail"
/// - Makes text have "smart punctuation", since rustdoc does the same
/// - Labels code blocks without a language as "Rust"
pub fn resolve_markdown(markdown: &str, links: Links<'_>, increment_headings: bool) -> String {
    let mut reference_definitions = Vec::new();
    // When searching for reference definitions, anything we find that
    // touches this set will be excluded because it is inside of a code block
    let mut code_block_ranges = RangeSet::new();

    // Pass 1/3
    //
    // Resolves regular links (such as inline links), broken links, code blocks
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
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level, .. })
            if increment_headings && level != pulldown_cmark::HeadingLevel::H6 =>
        {
            Some(ReplaceContent {
                range: span.clone(),
                content: format!("#{}", &markdown[span.clone()]).into(),
            })
        }
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

            // The finishing code block may be indented:
            //
            // - this is a list
            //
            //   ```
            //   this code block is in a list
            //   ```
            // ^^
            //
            // It is this indentation (marked by ^^) that this variable stores,
            // we will need it when we re-insert the code block
            let last_line_indentation = code_block_lines[code_block_lines.len().saturating_sub(1)]
                .chars()
                .take_while(|ch| ch.is_whitespace())
                .collect::<String>();

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
                content: format!(
                    "{code_fence}rust\n{code_block_content}\n{last_line_indentation}{code_fence}"
                )
                .into(),
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
        // for those reference definitions. Their existence is simply erased.
        //
        // So what we do is mostly a hack. We remember every reference ID and its URL,
        // then we do a 2nd search over the entire input to find all reference links.
        //
        // Reference links that have an entry in the map will be replaced
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
            link_type: LinkType::Reference | LinkType::Shortcut,
            dest_url,
            title: _,
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
            ..
        }) => {
            // [text](destination "title")
            // ^^^^^^^^^^^^^^^^^^^^^^^^^^^
            let markdown_link = &markdown[span.clone()];

            let Some(destination_range) = locate::inline_link_destination(markdown_link) else {
                // Link destination parsing failed for one reason or another
                return None;
            };

            let new_destination = links.get(&*dest_url)?;

            let destination_range =
                span.start + destination_range.start..span.start + destination_range.end;

            Some(ReplaceContent {
                range: destination_range,
                content: new_destination.to_string().into(),
            })
        }
        _ => None,
    });

    let markdown = ReplaceContent::replace_all(markdown.to_string(), replacements);

    // Pass 2/3
    //
    // Replace reference link definitions with the correct location
    //
    // [clone method]: Clone::clone
    //                 ^^^^^^^^^^^^
    //
    // becomes:
    //
    // [clone method]: https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone
    //                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let replacements = reference_definitions.into_iter().filter_map(|(dest, id)| {
        let new_url = links.get(&*dest)?;
        let match_ = locate::reference_link_definition(&markdown, dest, id, &code_block_ranges)?;
        Some(ReplaceContent {
            range: match_,
            content: CowStr::Borrowed(new_url),
        })
    });

    let markdown = ReplaceContent::replace_all(markdown.to_string(), replacements);

    // Pass 3/3
    //
    // Replace original text with processed text via ENABLE_SMART_PUNCTUATION
    //
    // Replaces -- with —, --- with —, ... with …, "quote" with “quote”, and 'quote' with ‘quote’.
    //
    // This is done as a separate pass to not mess up regular link replacements
    let replacements = pulldown_cmark::Parser::new_ext(&markdown, markdown_options())
        .into_offset_iter()
        .filter_map(|(event, span)| {
            if let pulldown_cmark::Event::Text(text) = event {
                Some(ReplaceContent {
                    range: span,
                    content: text,
                })
            } else {
                None
            }
        });

    ReplaceContent::replace_all(markdown.to_string(), replacements)
}

/// If this markdown fence language can be considered to be a "rust" language
///
/// All attributes: <https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes>
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
    use std::collections::HashMap;

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
        let out = resolve_markdown(input, links, true);
        assert_str_eq!(out, expected);
    }

    #[track_caller]
    fn unchanged(input: &str, links: Links<'_>) {
        t(input, links, input)
    }

    #[test]
    fn intra_doc_broken_link() {
        t(
            docstr! {
                /// just a [link]
            },
            links! {
                "link" => "to_somewhere"
            },
            docstr! {
                /// just a [link](to_somewhere)
            },
        );
    }

    #[test]
    fn regular_broken_link() {
        t(
            docstr! {
                /// just a [link]
            },
            HashMap::new(),
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
                /// just a [url](because)
                /// just a [link](to somewhere)
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
                "where?" => "to_somewhere"
            },
            docstr! {
                /// just a [link](to_somewhere "title")
            },
        );

        // 3 spaces at the front
        t(
            docstr! {
                /// just a [link](where?   "title")
            },
            links! {
                "where?" => "to_somewhere"
            },
            docstr! {
                /// just a [link](to_somewhere   "title")
            },
        );

        // 2 spaces at the front and at the back
        t(
            docstr! {
                /// just a [link](where?  "title"  )
            },
            links! {
                "where?" => "to_somewhere"
            },
            docstr! {
                /// just a [link](to_somewhere  "title"  )
            },
        );
    }

    #[test]
    fn regular_inline_link() {
        t(
            docstr! {
                /// just a [link](where?)
            },
            HashMap::new(),
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
        unchanged(
            docstr! {
                /// just a [link]
                ///
                /// [link]: somewhere
            },
            links! {
                "..." => "..."
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

    #[test]
    fn indented_code_block() {
        t(
            docstr! {
                /// - this is a list item
                ///
                ///   ```ignore
                ///   I am inside of a code block
                ///   ```
            },
            HashMap::new(),
            docstr! {
                /// - this is a list item
                ///
                ///   ```rust
                ///   I am inside of a code block
                ///   ```
            },
        );
    }

    #[test]
    fn increment_headings() {
        t(
            docstr! {
                /// # a
                ///
                ///  ## b
                ///
                /// ### c
                ///
                /// #### d
                ///
                /// ##### e
                ///
                /// ###### f
            },
            HashMap::new(),
            docstr! {
                /// ## a
                ///
                ///  ### b
                ///
                /// #### c
                ///
                /// ##### d
                ///
                /// ###### e
                ///
                /// ###### f
            },
        );
    }
}
