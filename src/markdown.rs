use eyre::Context as _;

use crate::intralinks::{self, Link};

/// Given a `markdown` string:
///
/// - Resolves all links in it using `intralink_resolver`
/// - Modifies code blocks
pub fn resolve_markdown(
    markdown: &str,
    intralink_resolver: intralinks::IntralinkResolver<'_>,
) -> String {
    let events = pulldown_cmark::Parser::new_with_broken_link_callback(
        markdown,
        pulldown_cmark::Options::all(),
        Some(crate::ResolveIntraDocLinks {
            intralink_resolver: &intralink_resolver,
        }),
    );
    let events = pulldown_cmark::TextMergeStream::new(events);

    #[derive(Default)]
    struct State {
        is_rust_codeblock: bool,
    }

    let events = events.scan(State::default(), |state, event| match &event {
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(code_block_kind)) => {
            match code_block_kind {
                pulldown_cmark::CodeBlockKind::Indented => state.is_rust_codeblock = true,
                pulldown_cmark::CodeBlockKind::Fenced(cow_str) => {
                    state.is_rust_codeblock = is_rust_code_block(cow_str);
                }
            };

            if state.is_rust_codeblock {
                // Change language to rust
                Some(pulldown_cmark::Event::Start(
                    pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(
                        "rust".into(),
                    )),
                ))
            } else {
                // Keep code block as-is
                Some(event)
            }
        }
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let new_destination = intralink_resolver
                .resolve_link(&Link {
                    raw_link: dest_url.to_string(),
                })
                .unwrap_or(dest_url);

            Some(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                link_type: *link_type,
                dest_url: new_destination.to_string().into(),
                title: title.to_string().into(),
                id: id.to_string().into(),
            }))
        }
        pulldown_cmark::Event::Text(text) if state.is_rust_codeblock => {
            state.is_rust_codeblock = false;
            Some(pulldown_cmark::Event::Text(
                process_rust_code_block(text).into(),
            ))
        }
        _ => {
            state.is_rust_codeblock = false;
            Some(event)
        }
    });

    let mut output_markdown = String::new();
    let _ = pulldown_cmark_to_cmark::cmark(events, &mut output_markdown)
        .context("failed to write markdown");
    output_markdown
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
