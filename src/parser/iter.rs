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
        while let Some((line_index, mut line)) = self.lines.next() {
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
