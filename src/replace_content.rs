use std::ops::Range;

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
        replacements
            .into_iter()
            .fold((0isize, string), |(offset, mut string), replace| {
                let start = replace.range.start.strict_add_signed(offset);
                let end = replace.range.end.strict_add_signed(offset);

                string.replace_range(start..end, &replace.content);

                let offset_current = replace.content.len() as isize - (end - start) as isize;

                (offset + offset_current, string)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_iter() {
        let content = "00 aaa bbb ccc dd";

        let replacements = [
            ("aaa", "11"),
            // "aaa bbb ccc"
            //      ^^^
            ("bbb", "222"),
            // "aaa bbb ccc"
            //      ^^^
            ("ccc", "3333"),
        ]
        .iter()
        .map(|(replace, with)| {
            let start = content
                .find(replace)
                .unwrap_or_else(|| panic!("failed to find pattern: {replace}"));

            let range = start..start + replace.len();

            assert_eq!(*replace, &content[range.clone()], "invalid range");

            ReplaceContent {
                range,
                content: with.to_string(),
            }
        });

        let replaced = ReplaceContent::replace_all(content.to_string(), replacements);

        assert_eq!(replaced, "00 11 222 3333 dd");
    }
}
