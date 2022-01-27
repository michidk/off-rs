use std::{iter::Enumerate, str::Lines};

// line iterator by github.com/Shemnei
#[derive(Debug, Clone)]
pub(super) struct OffLines<'a> {
    lines: Enumerate<Lines<'a>>,
}

impl<'a> OffLines<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            lines: s.lines().enumerate(),
        }
    }
}

impl<'a> Iterator for OffLines<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        for (line_index, mut line) in self.lines.by_ref() {
            if let Some(comment_index) = line.find('#') {
                line = &line[..comment_index];
            }

            // Trim after removing comments to prevent the following `Hello # World` => `Hello `
            // (should be `Hello`)
            line = line.trim();

            if !line.is_empty() {
                return Some((line_index, line));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let str = r#"this
        is
        a
        test
        "#;

        let mut lines = OffLines::new(str).peekable();
        assert_eq!(lines.next(), Some((0, "this")));
        assert_eq!(lines.next(), Some((1, "is")));
        assert_eq!(lines.next(), Some((2, "a")));
        assert_eq!(lines.next(), Some((3, "test")));
        assert_eq!(lines.next(), None);
    }
}
