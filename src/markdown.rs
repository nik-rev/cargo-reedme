use core::fmt;
use std::{borrow::Cow, ops::Range};

use eyre::Context as _;
use itertools::Itertools;
use pulldown_cmark::LinkType;

use crate::{
    intralinks::{self, Link},
    replace_content::ReplaceContent,
};

/// Given a `markdown` string:
///
/// - Resolves all links in it using `intralink_resolver`
/// - Strips rustdoc-specific tags from code fences, such as "edition2024,compile_fail"
/// - Labels code blocks without a language as "Rust"
pub fn resolve_markdown(
    markdown: &str,
    intralink_resolver: intralinks::IntralinkResolver<'_>,
) -> String {
    let replacements = pulldown_cmark::Parser::new_with_broken_link_callback(
        markdown,
        pulldown_cmark::Options::all(),
        Some(crate::ResolveIntraDocLinks {
            intralink_resolver: &intralink_resolver,
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
            let code_block_lines = markdown.lines().collect_vec();

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
            // Only rewrite inline links of the form [text](destination)
            if !matches!(link_type, LinkType::Inline) {
                return None;
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

            let new_destination = intralink_resolver
                .resolve_link(&Link {
                    raw_link: dest_url.to_string(),
                })
                .unwrap_or(&dest_url);

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

pub fn process_rust_code_block(code_block: &str) -> String {
    let mut new_doc_str = String::new();
    let mut first = true;

    for (i, line) in code_block.split('\n').enumerate() {
        // If we have an indent code block and we start with a comment we need to
        // drop any indent whitespace that started this indent block, since
        // pulldown-cmark doesn't consider it part of the code block.
        if i == 0 && is_line_commented(line) {
            while !new_doc_str.ends_with('\n') && !new_doc_str.is_empty() {
                new_doc_str.pop();
            }
        }

        if !is_line_commented(line) {
            if !first {
                new_doc_str.push('\n');
            }

            // Lines starting with `##` are not comments, that is a way to intentionally start a
            // line with `#`.  See https://github.com/rust-lang/rust/pull/41785.
            match line.trim_start().starts_with("##") {
                true => new_doc_str.push_str(&line.replacen('#', "", 1)),
                false => new_doc_str.push_str(line),
            }

            first = false;
        }
    }
    new_doc_str
}

fn is_line_commented(line: &str) -> bool {
    line.trim_start().starts_with("# ") || line.trim() == "#"
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
