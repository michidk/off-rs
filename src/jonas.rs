use std::iter::Enumerate;
use std::{
    borrow::Cow,
    str::{FromStr, Lines},
};

use crate::parse::OffParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    kind: ErrorKind,
    line_index: usize,
    message: Option<Cow<'static, str>>,
}

impl Error {
    pub fn new(kind: ErrorKind, line_index: usize, message: Option<Cow<'static, str>>) -> Self {
        Self {
            kind,
            line_index,
            message,
        }
    }

    pub fn with_message<M: Into<Cow<'static, str>>, O: Into<Option<M>>>(
        kind: ErrorKind,
        line_index: usize,
        message: O,
    ) -> Self {
        Self {
            kind,
            line_index,
            message: message.into().map(|inner| inner.into()),
        }
    }

    pub fn without_message(kind: ErrorKind, line_index: usize) -> Self {
        Self {
            kind,
            line_index,
            message: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} @ ln:{}{}",
            self.kind,
            self.line_index + 1,
            self.message
                .as_ref()
                .map(|msg| format!(" - {}", msg))
                .unwrap_or_else(|| String::new())
        )
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Empty,
    Missing,
    Invalid,
    InvalidMagic,
    InvalidCounts,
    InvalidVertex,
    InvalidColor,
    InvalidFace,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Face {
    indexes: Vec<usize>,
    color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Off {
    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    edge_count: Option<usize>,
}

impl FromStr for Off {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OffParser::new(&s).try_parse()
    }
}

#[derive(Debug, Clone)]
pub struct OffLines<'a> {
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

pub mod parse {
    use std::iter::Peekable;

    use crate::{Color, Error, ErrorKind, Face, Off, OffLines, Result, Vec3};

    pub struct OffParser<'a> {
        lines: Peekable<OffLines<'a>>,
    }

    impl<'a> OffParser<'a> {
        pub fn new<S: AsRef<str>>(s: &'a S) -> Self {
            let lines = OffLines::new(s.as_ref()).peekable();

            Self { lines }
        }

        pub fn try_parse(mut self) -> Result<Off> {
            let _ = self.try_consume_magic()?;
            let (vertex_count, face_count, edge_count) = self.try_consume_counts()?;

            let vertices = self.try_consume_vertices(vertex_count)?;
            let faces = self.try_consume_faces(face_count, vertex_count)?;

            // TODO: Validate face indexes

            if let Some((line_index, _)) = self.lines.next() {
                Err(Error::with_message(
                    ErrorKind::Invalid,
                    line_index,
                    "Unexpected lines after OFF definition",
                ))
            } else {
                Ok(Off {
                    vertices,
                    faces,
                    edge_count,
                })
            }
        }

        fn try_consume_magic(&mut self) -> Result<()> {
            let (line_index, line) = self
                .lines
                .peek()
                .ok_or_else(|| Error::without_message(ErrorKind::Empty, 0))?;

            if let Some(suffix) = line.strip_prefix("OFF") {
                if suffix.is_empty() {
                    // valid magic
                    // consume peeked item
                    let _ = self.lines.next().expect("Next item not present");
                } else {
                    // trailing characters; invalid magic
                    return Err(Error::with_message(
                        ErrorKind::InvalidMagic,
                        *line_index,
                        "Trailing characters after magic",
                    ));
                }
            }

            Ok(())
        }

        fn try_consume_counts(&mut self) -> Result<(usize, usize, Option<usize>)> {
            let (line_index, line) = self.lines.next().ok_or_else(|| {
                Error::with_message(
                    ErrorKind::Missing,
                    0,
                    "No counts for vertices, faces and edges present",
                )
            })?;

            let counts = line
                .split_whitespace()
                .map(|w| w.parse::<usize>())
                // Take one more than we expect/want so that we can check bellow
                // if we got the expected amount or more.
                .take(4)
                .collect::<Result<Vec<usize>, _>>()
                .map_err(|err| {
                    Error::with_message(
                        ErrorKind::InvalidCounts,
                        line_index,
                        format!("Failed to parse count as number ({})", err),
                    )
                })?;

            match counts[..] {
                [vertices, faces, edges] => Ok((vertices, faces, Some(edges))),
                [vertices, faces] => Ok((vertices, faces, None)),
                _ => Err(Error::with_message(
                    ErrorKind::InvalidCounts,
                    line_index,
                    format!(
                        "Invalid number of counts given (expected: 2-3, actual: {})",
                        counts.len()
                    ),
                )),
            }
        }

        fn try_consume_vertices(&mut self, vertex_count: usize) -> Result<Vec<Vec3>> {
            (0..vertex_count)
                .map(|_| self.try_consume_vertex())
                .collect()
        }

        fn try_consume_vertex(&mut self) -> Result<Vec3> {
            let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| Error::with_message(ErrorKind::Missing, 0, "Expected vertex"))?;

            let coords = line
                .split_whitespace()
                .map(|w| w.parse::<f32>())
                // Take one more than we expect/want so that we can check bellow
                // if we got the expected amount or more.
                .take(4)
                .collect::<Result<Vec<f32>, _>>()
                .map_err(|err| {
                    Error::with_message(
                        ErrorKind::InvalidVertex,
                        line_index,
                        format!("Failed to parse coordinate as number ({})", err),
                    )
                })?;

            if let [x, y, z] = coords[..] {
                Ok(Vec3::xyz(x, y, z))
            } else {
                Err(Error::with_message(
                    ErrorKind::InvalidVertex,
                    line_index,
                    format!(
                        "Invalid number of coordinates given (expected: 3, actual: {})",
                        coords.len()
                    ),
                ))
            }
        }

        fn try_consume_faces(
            &mut self,
            face_count: usize,
            vertex_count: usize,
        ) -> Result<Vec<Face>> {
            (0..face_count)
                .map(|_| self.try_consume_face(vertex_count))
                .collect()
        }

        fn try_consume_face(&mut self, vertex_count: usize) -> Result<Face> {
            let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| Error::with_message(ErrorKind::Missing, 0, "Expected face"))?;

            let mut words = line.split_whitespace();

            let vertex_index_count = words
                .next()
                .ok_or_else(|| {
                    Error::with_message(
                        ErrorKind::InvalidFace,
                        line_index,
                        "Expected number of vertices",
                    )
                })?
                .parse::<usize>()
                .map_err(|err| {
                    Error::with_message(
                        ErrorKind::InvalidFace,
                        line_index,
                        format!("Failed to parse vertex count as number ({})", err),
                    )
                })?;

            let mut vertex_indexes = Vec::with_capacity(vertex_index_count);

            for i in 0..vertex_index_count {
                let vertex_index = words
                    .next()
                    .ok_or_else(|| {
                        Error::with_message(
                            ErrorKind::InvalidFace,
                            line_index,
                            format!("Expected vertex index ({}/{})", i, vertex_index_count),
                        )
                    })?
                    .parse::<usize>()
                    .map_err(|err| {
                        Error::with_message(
                            ErrorKind::InvalidFace,
                            line_index,
                            format!(
                                "Failed to parse vertex index as number ({}/{}; {})",
                                i, vertex_index_count, err
                            ),
                        )
                    })?;

                if vertex_index >= vertex_count {
                    return Err(Error::with_message(
                        ErrorKind::InvalidFace,
                        line_index,
                        format!(
                            "Vertex index out of bounds ({}/{})",
                            vertex_index, vertex_count
                        ),
                    ));
                }

                vertex_indexes.push(vertex_index);
            }

            // Check for color
            let color_values = words
                .map(|word| word.parse::<u8>())
                .take(4)
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|err| {
                    Error::with_message(
                        ErrorKind::InvalidCounts,
                        line_index,
                        format!("Failed to parse color value as number ({})", err),
                    )
                })?;

            if let [r, g, b] = color_values[..] {
                Ok(Face {
                    indexes: vertex_indexes,
                    color: Some(Color::rgb(r, g, b)),
                })
            } else if color_values.is_empty() {
                Ok(Face {
                    indexes: vertex_indexes,
                    color: None,
                })
            } else {
                Err(Error::with_message(
                    ErrorKind::InvalidVertex,
                    line_index,
                    format!(
                        "Invalid number of color values given (expected: 3, actual: {})",
                        color_values.len()
                    ),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki() {
        let content = r#"OFF
# cube.off
# A cube

8 6 12
 1.0  0.0 1.4142
 0.0  1.0 1.4142
-1.0  0.0 1.4142
 0.0 -1.0 1.4142
 1.0  0.0 0.0
 0.0  1.0 0.0
-1.0  0.0 0.0
 0.0 -1.0 0.0
4  0 1 2 3  255 0 0 #red
4  7 4 0 3  0 255 0 #green
4  4 5 1 0  0 0 255 #blue
4  5 6 2 1  0 255 0
4  3 2 6 7  0 0 255
4  6 5 4 7  255 0 0"#;

        let off = content.parse::<Off>();

        println!("{:#?}", off);
    }
}
