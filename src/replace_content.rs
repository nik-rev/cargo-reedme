use std::ops::Range;

use tracing::{info, trace};

/// Replace a part of the string with something else
pub struct ReplaceContent {
    /// Location of text to replace
    ///
    /// Range can be empty to insert text
    pub range: Range<usize>,
    /// What to replace that text with
    pub content: String,
}

impl ReplaceContent {
    /// Applies all `replacements` to the given `string`
    pub fn replace_all(string: String, replacements: impl IntoIterator<Item = Self>) -> String {
        trace!(content = %string);

        replacements
            .into_iter()
            .fold((0isize, string), |(offset, mut string), replace| {
                let start = replace.range.start.strict_add_signed(offset);
                let end = replace.range.end.strict_add_signed(offset);

                info!(replacing = %&string[start..end], with = %replace.content);

                string.replace_range(start..end, &replace.content);

                let offset_current = replace.content.len() as isize - (end - start) as isize;

                (offset + offset_current, string)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    use super::*;

    #[track_caller]
    fn t<'a>(
        input: &str,
        replacements: impl IntoIterator<Item = (&'a str, &'a str)>,
        result: &str,
    ) {
        let replacements = replacements.into_iter().map(|(replace, with)| {
            let start = input
                .find(replace)
                .unwrap_or_else(|| panic!("failed to find pattern: {replace}"));

            let range = start..start + replace.len();

            assert_eq!(replace, &input[range.clone()], "invalid range");

            ReplaceContent {
                range,
                content: with.to_string(),
            }
        });

        let replaced = ReplaceContent::replace_all(input.to_string(), replacements);

        assert_str_eq!(result, replaced, "wrong replacement");
    }

    #[test]
    fn simple() {
        t(
            "00 aaa bbb ccc dd",
            [("aaa", "11"), ("bbb", "222"), ("ccc", "3333")],
            "00 11 222 3333 dd",
        );
    }

    #[test]
    fn out_of_order() {
        // NOTE: Replacement for "ccc" comes before "aaa" in the list
        t(
            "aaa bbb ccc",
            [("ccc", "333"), ("aaa", "111")],
            "111 bbb 333",
        );
    }

    #[test]
    fn emojis() {
        t(
            "🦀 is a crab, aaa is a test",
            [("aaa", "success")],
            "🦀 is a crab, success is a test",
        );
    }

    #[test]
    fn boundaries() {
        t("A", [("A", "Alphabet")], "Alphabet");
    }
}
