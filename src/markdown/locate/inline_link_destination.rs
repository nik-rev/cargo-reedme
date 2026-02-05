use docstr::docstr;
use itertools::Itertools;
use std::ops::Range;

/// Returns span of link destination in the `input`, which must be a markdown [inline  link]
///
/// [inline link]: https://spec.commonmark.org/0.31.2/#inline-link
///
/// # Example
///
/// ```markdown
/// This is an [inline link](<https://github.com> "Link to GitHub")
///                           ^^^^^^^^^^^^^^^^^^
/// ```
///
/// # Panics
///
/// If the `input` is not a valid Markdown inline link
pub fn inline_link_destination(input: &str) -> Option<Range<usize>> {
    let mut chars = input.char_indices().rev().peekable();

    let closing_paren = chars.next().expect("invalid inline markdown link");

    // [inline link](<https://github.com> "Link to GitHub")
    //                                                    ^
    assert_eq!(
        closing_paren.1, ')',
        "invalid inline markdown link: {input}"
    );

    let closing_paren_index = closing_paren.0;

    // Where the title ends
    // [inline link](<https://github.com> "Link to GitHub"  )
    //                                                   ^
    let mut title_end = input.len();

    // This variable represents the index of the last character
    // of the destination, which may be after some whitespace, e.g.
    //
    // [link](/uri   "title")
    //               ^
    // [link](/uri)
    //            ^
    //
    // From here, we can get position of the link destination
    let destination_end = 'parse_title: loop {
        let Some((i, ch)) = chars.peek() else { break 0 };

        match *ch {
            // a sequence of zero or more characters between straight double-quote characters ("), including a " character only if it is backslash-escaped
            '"' => {
                title_end = *i + 1;
                chars.next();

                while let Some((i, ch)) = chars.next() {
                    if ch == '"'
                        && let Some((_, '\\')) = chars.peek()
                    {
                        // Escaped double-quote character: \"
                    } else if ch == '"' {
                        // End of title
                        break 'parse_title i;
                    } else {
                        // Regular character
                    }
                }
            }
            // a sequence of zero or more characters between straight single-quote characters ('), including a ' character only if it is backslash-escaped
            '\'' => {
                title_end = *i + 1;
                chars.next();

                while let Some((i, ch)) = chars.next() {
                    if ch == '\''
                        && let Some((_, '\\')) = chars.peek()
                    {
                        // Escaped single-quote character: \'
                    } else if ch == '\'' {
                        // End of title
                        break 'parse_title i;
                    } else {
                        // Regular character
                    }
                }
            }
            // a sequence of zero or more characters between matching parentheses ((...)), including a ( or ) character only if it is backslash-escaped
            ')' => {
                title_end = *i + 1;
                chars.next();

                if let Some(i) = locate_matching_opening_parentheses(&mut chars) {
                    break i;
                };
            }
            ch if ch.is_whitespace() => {
                chars.next();
                // whitespace after title is ignored
                //
                // [link](/uri   "title"   )
                //                      ^^^
            }
            // not a link title
            _ => break closing_paren_index,
        }
    };

    let mut destination_end = destination_end;

    // Eat all whitespace before the link title
    //
    // [link](<destination>     "title")
    //                     ^ we'll be left here
    while let Some((i, _)) = chars.next_if(|(_, ch)| ch.is_whitespace()) {
        destination_end = i;
    }

    if matches!(chars.peek(), Some((_, '>'))) {
        // link destination is enclosed in angle brackets

        // a sequence of zero or more characters between an opening < and a closing > that
        // contains no line endings or unescaped < or > characters
        //
        // example:
        //
        // [link](<destination>)
        //                    ^
        let closing = chars.next().unwrap().0;

        let opening = loop {
            match chars.next() {
                // Reached start of destination
                //
                // [link](<destination>)
                //        ^
                Some((i, '<')) if !matches!(chars.peek(), Some((_, '\\'))) => break i,
                Some(_) => {}
                None => panic!("invalid markdown inline link input"),
            }
        };

        // opening..=closing would include the opening '<' and closing '>'
        Some(opening + 1..closing)
    } else {
        let next_char_index = chars.peek().map(|(i, _)| *i);

        // link destination is NOT enclosed in angle brackets
        //
        // parentheses may be present in the destination, but escaped and balanced:
        //
        // we are here (the cursor, whitespace before "title" was skipped earlier):
        //
        // [link]( destination  "title")
        //                    ^
        //
        // [link]( destination  "title")
        //       ^ this is the required opening parentheses
        //
        // we want this range:
        //
        // [link]( destination  "title")
        //         ^^^^^^^^^^^
        //
        // [link]( des(t(in)a)t\)ion  "title")
        //
        // So we find position of this opening parentheses:
        //
        // [link]( des(t(in)a)t\)ion  "title")
        //       ^
        let opening_parentheses_position = locate_matching_opening_parentheses(&mut chars)
            .unwrap_or_else(|| panic!("input is invalid markdown link: {input}"));

        // If the next character after the destination is also the opening parentheses,
        // then what we thought was the title is actually the destination
        //
        // For example:
        //
        // [link]("dest")
        //        ^^^^^^
        //
        // That is not a title, that is the destination
        if next_char_index == Some(opening_parentheses_position) {
            // +1 to exclude the opening parentheses
            let title_start = opening_parentheses_position + 1;

            if &input[title_start..title_end] == ")" {
                // in this case, there is no title at all, AND no link destination
                // [link]()
                //        |
                //
                // We return an empty range, so replacing it with the link title will just insert the
                // link title at the correct place
                return Some(title_start..title_start);
            } else {
                return Some(title_start..title_end);
            }
        }

        // the destination is between `opening_parentheses_position` and `destination_end`, we must
        // trim whitespace between them first
        //
        // [link]( des(t(in)a)t\)ion  "title")
        //       ^ opening_parentheses_position
        //                            ^ destination_end
        //         ^^^^^^^^^^^^^^^^^ we want this

        // location of where the destination starts
        //
        // [link]( des(t(in)a)t\)ion  "title")
        //         ^
        let destination_start = input
            .char_indices()
            .skip_while(|(i, _)| *i != opening_parentheses_position)
            // skip the opening parentheses
            .skip(1)
            .find(|(_, ch)| !ch.is_whitespace())
            .map_or(opening_parentheses_position, |(i, _)| i);

        Some(destination_start..destination_end)
    }
}

/// Returns position of the matching opening parentheses
///
/// a ( () \( )
///
///           ^ if we start here
///
/// a ( () \( )
///
///   ^ position of this parentheses will be returned
///
/// a ( () \( )
///
///      ^ if we start here
///
/// a ( () \( )
///
///     ^ position of this parentheses will be returned
fn locate_matching_opening_parentheses(
    chars: &mut std::iter::Peekable<std::iter::Rev<std::str::CharIndices<'_>>>,
) -> Option<usize> {
    let mut nesting_level = 1;

    while let Some((i, ch)) = chars.next() {
        match ch {
            '(' if chars.peek().is_some_and(|(_, ch)| *ch == '\\') => {
                // Escaped opening parentheses: \(
            }
            '(' => {
                if nesting_level == 1 {
                    return Some(i);
                } else if nesting_level == 0 {
                    // example of invalid input: [foo](bar ()))
                    //                                      ^ unpalanced parentheses
                    panic!("invalid inline markdown link title, contains unbalanced parentheses",)
                } else {
                    nesting_level -= 1;
                }
            }
            ')' if chars.peek().is_some_and(|(_, ch)| *ch == '\\') => {
                // Escaped closing parentheses: \)
            }
            ')' => {
                nesting_level += 1;
            }
            _ => {
                // any other characters
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use anstream::eprintln;
    use pretty_assertions::assert_str_eq;
    use simply_colored::*;

    use super::*;

    /// Tests that the extracted destination of a link has a correct location
    #[track_caller]
    fn t(s: &'static str) {
        eprintln!("{YELLOW}{s}{RESET}");
        enum Assert {
            /// Uses | to assert that the range ends and begins at a given character,
            /// the only reasonable way to represent an empty range.
            ///
            /// Still important because that's where we will actually insert the link text,
            /// e.g. replacing destination with "foo" in [link]() will make it into [link](foo)
            Position(usize),
            /// Represents destination of a link
            Range(Range<usize>),
        }

        let (assert_line_idx, assert) = s
            .lines()
            .enumerate()
            .find_map(|(line_index, line)| {
                let chars = line.char_indices().filter(|(_, ch)| !ch.is_whitespace());

                if let Ok((char_index, '|')) = chars.clone().exactly_one() {
                    Some((line_index, Assert::Position(char_index)))
                } else if chars.clone().all(|(_, ch)| ch == '^') {
                    Some((
                        line_index,
                        Assert::Range(
                            chars.clone().next().unwrap().0
                                ..chars.clone().next_back().unwrap().0 + 1,
                        ),
                    ))
                } else {
                    None
                }
            })
            .expect("expected a line that contains ^^^^^^^ or a single |");

        let s = s
            .lines()
            .enumerate()
            .filter_map(|(i, line)| (i != assert_line_idx).then_some(line))
            .join("\n");

        let obtained_range = super::inline_link_destination(&s)
            .expect("we always pass a valid markdown inline link");

        match assert {
            Assert::Position(position) => {
                assert_eq!(position..position, obtained_range);
            }
            Assert::Range(expected_destination_range) => {
                assert_str_eq!(s[obtained_range], s[expected_destination_range]);
            }
        }
    }

    #[test]
    fn with_title() {
        t(docstr! {
           /// [link](/uri "title")
           ///        ^^^^
        });
    }

    /// The title, the link text and even the destination may be omitted:
    #[test]
    fn omitted_parts() {
        t(docstr! {
           /// [link](/uri)
           ///        ^^^^
        });
        t(docstr! {
           /// [](./target.md)
           ///    ^^^^^^^^^^^
        });
        t(docstr! {
           /// [link]()
           ///        |
        });
        t(docstr! {
           /// [link](<>)
           ///         |
        });
        t(docstr! {
           /// []()
           ///    |
        });
    }

    #[test]
    fn unescaped_closing_paren_inside_destination() {
        t(docstr! {
           /// [a](<b)c>)
           ///      ^^^
        });
    }

    /// Parentheses inside the link destination may be escaped:
    #[test]
    fn escaped_parens_inside_destination() {
        t(docstr! {
           /// [link](\(foo\))
           ///        ^^^^^^^
        });
    }

    /// Any number of parentheses are allowed without escaping, as long as they are balanced:
    #[test]
    fn balanced_parentheses() {
        t(docstr! {
           /// [link](foo(and(bar)))
           ///        ^^^^^^^^^^^^^
        });
    }

    /// However, if you have unbalanced parentheses, you need to escape or use the <...> form:
    #[test]
    fn unbalanced_parentheses() {
        t(docstr! {
           /// [link](foo\(and\(bar\))
           ///        ^^^^^^^^^^^^^^^
        });
    }

    /// Parentheses and other symbols can also be escaped, as usual in Markdown:
    #[test]
    fn escaped() {
        t(docstr! {
           /// [link](foo\)\:)
           ///        ^^^^^^^
        });
    }

    /// A link can contain fragment identifiers and queries:
    #[test]
    fn fragment_identifier_and_queries() {
        t(docstr! {
           /// [link](#fragment)
           ///        ^^^^^^^^^
        });
        t(docstr! {
           /// [link](https://example.com#fragment)
           ///        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        });
        t(docstr! {
           /// [link](https://example.com?foo=3#frag)
           ///        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        });
    }

    /// Note that a backslash before a non-escapable character is just a backslash:
    #[test]
    fn regular_backslash() {
        t(docstr! {
           /// [link](foo\bar)
           ///        ^^^^^^^
        });
    }

    #[test]
    fn url_esaped() {
        t(docstr! {
           /// [link](foo%20b&auml;)
           ///        ^^^^^^^^^^^^^
        });
    }

    #[test]
    fn looks_like_title_but_is_dest() {
        t(docstr! {
           /// [link]("dest")
           ///        ^^^^^^
        });
    }

    /// Titles may be in single quotes, double quotes, or parentheses:
    #[test]
    fn kinds_of_titles() {
        t(docstr! {
           /// [link](/url "title")
           ///        ^^^^
        });
        t(docstr! {
           /// [link](/url 'title')
           ///        ^^^^
        });
        t(docstr! {
           /// [link](/url (title))
           ///        ^^^^
        });
    }

    /// Backslash escapes and entity and numeric character references may be used in titles
    #[test]
    fn escapes() {
        t(docstr! {
           /// [link](/url "title \"&quot;")
           ///        ^^^^
        });
    }

    /// Titles must be separated from the link using spaces, tabs, and up to one line ending.
    #[test]
    fn title_basic() {
        t(docstr! {
           /// [link](/url "title")
           ///        ^^^^
        });
    }

    #[test]
    fn mixed_title_quotes() {
        t(docstr! {
           /// [link](/url 'title "and" title')
           ///        ^^^^
        });
    }

    /// Spaces, tabs, and up to one line ending is allowed around the destination and title:
    #[test]
    fn mixed_whitespace() {
        t(docstr! {
           /// [link](   /uri
           ///   "title"  )
           ///           ^^^^
        });
    }

    /// The link text may contain balanced brackets, but not unbalanced ones, unless they are escaped:
    #[test]
    fn link_brackets() {
        t(docstr! {
           /// [link [foo [bar]]](/uri)
           ///                    ^^^^
        });
        t(docstr! {
           /// [link \[bar](/uri)
           ///              ^^^^
        });
    }

    /// The link text may contain inline content:
    #[test]
    fn link_nested() {
        t(docstr! {
           /// [link *foo **bar** `#`*](/uri)
           ///                          ^^^^
        });
        t(docstr! {
           /// [![moon](moon.jpg)](/uri)
           ///                     ^^^^
        });
    }

    /// The link text may contain inline content:
    #[test]
    fn broken_bold() {
        t(docstr! {
           /// [foo *bar](baz*)
           ///            ^^^^
        });
    }
}
