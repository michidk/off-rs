use std::{
    iter::{Enumerate, Peekable},
    str::{FromStr, Lines}, borrow::Cow,
};

use thiserror::Error;

use crate::{
    document::{DocumentResult, OffDocument, ParserOptions},
    geometry::{Color, ColorFormat, Face, GeometryError, Position, Vertex},
};

// line iterator by github.com/Shemnei
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

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError {
    kind: ParserErrorKind,
    line_index: usize,
    message: Option<Cow<'static, str>>,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, line_index: usize, message: Option<Cow<'static, str>>) -> Self {
        Self {
            kind,
            line_index,
            message,
        }
    }

    pub fn with_message<M: Into<Cow<'static, str>>, O: Into<Option<M>>>(
        kind: ParserErrorKind,
        line_index: usize,
        message: O,
    ) -> Self {
        Self {
            kind,
            line_index,
            message: message.into().map(|inner| inner.into()),
        }
    }

    pub fn without_message(kind: ParserErrorKind, line_index: usize) -> Self {
        Self {
            kind,
            line_index,
            message: None,
        }
    }
}

impl std::fmt::Display for ParserError {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserErrorKind {
    Empty,
    Missing,
    Invalid,
    InvalidHeader,
    InvalidCounts,
    InvalidVertex,
    InvalidColor,
    InvalidFace,
}

impl std::fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}

pub type ParserResult<T = ()> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct DocumentParser<'a> {
    lines: Peekable<OffLines<'a>>,
    vertex_count: usize,
    face_count: usize,
    edge_count: usize,
    document: OffDocument,
    options: ParserOptions,
}

impl<'a> DocumentParser<'a> {
    pub fn new<S: AsRef<str>>(s: &'a S, options: ParserOptions) -> Self {
        let lines = OffLines::new(s.as_ref()).peekable();

        DocumentParser {
            lines,
            vertex_count: 0,
            face_count: 0,
            edge_count: 0,
            document: OffDocument::new(),
            options,
        }
    }

    pub fn parse(mut self) -> DocumentResult {
        self.parse_header()?;
        self.parse_counts()?;
        self.parse_vertices()?;
        self.parse_faces()?;

        // TODO: valitdate the counts

        self.finalize()
    }

    fn parse_header(&mut self) -> ParserResult {
        let (line_index, line) = self
            .lines
            .next()
            .ok_or_else(|| ParserError::without_message(ParserErrorKind::Empty, 0))?;

        if line != "OFF" {
            return Err(ParserError::with_message(ParserErrorKind::InvalidHeader, line_index, "First non-comment line should be `OFF`"));
        }

        Ok(())
    }

    fn parse_counts(&mut self) -> ParserResult {
        let (line_index, line) = self
            .lines
            .next()
            .ok_or_else(|| ParserError::InvalidCounts)?;
        let counts: Vec<&str> = line.split_line();

        let num: Vec<usize> = counts
            .into_iter()
            .map(|s| s.parse().map_err(|_| ParserError::InvalidCounts))
            .collect::<Result<Vec<usize>, ParserError>>()?;

        // TODO: dont check that this element exist because we already have the code for that somewhere
        self.vertex_count = num[0];
        match num[1..] {
            [face_count, edge_count, ..] => {
                self.face_count = face_count;
                self.edge_count = edge_count;
            }
            [face_count] => {
                self.face_count = face_count;
            }
            [] => {}
        }

        Ok(())
    }

    fn parse_vertices(&mut self) -> ParserResult {
        for _ in 0..self.vertex_count {
            let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| ParserError::InvalidVertex)?;

            let parts = line.split_line();
            let vertex = self.parse_vertex(line_index, parts)?;
            self.document.vertices.push(vertex);
        }

        Ok(())
    }

    fn parse_vertex(&mut self, line_index: usize, parts: Vec<&str>) -> ParserResult<Vertex> {
        // TODO: dont work if we have colors
        // if vertex_str.len() != 3 {
        //     return Err(ParserError::InvalidVertex);
        // }

        let position = self.parse_position(line_index, parts.clone())?;

        let color = if parts.len() > 3 {
            Some(self.parse_color(line_index, parts[4..].to_vec())?)
        } else {
            None
        };

        Ok(Vertex { position, color })
    }

    fn parse_position(&mut self, line_index: usize, parts: Vec<&str>) -> ParserResult<Position> {

        if parts.len() != 3 {
            return Err(ParserError::InvalidVertex);
        }

        let position_parts: Vec<f32> = parts
            .into_iter()
            .take(3)
            .map(|s| s.parse().map_err(|_| ParserError::InvalidVertex))
            .collect::<Result<Vec<f32>, ParserError>>()?;

        Ok(Position::try_from(position_parts)?)
    }

    fn parse_color(&mut self, line_index: usize, parts: Vec<&str>) -> ParserResult<Color> {
        let color_elems = parts
            .into_iter()
            .take(self.options.color_format.element_count());

        let color_float = if self.options.color_format.is_float() {
            // directly parse as f32
            color_elems
                .map(|s| s.parse::<f32>().map_err(|_| ParserError::InvalidVertex))
                .collect::<Result<Vec<f32>, ParserError>>()
            // TODO: check size
        } else {
            // parse as u8 and convert to f32
            color_elems
                .map(|s| {
                    s.parse::<u8>()
                        .map(|val| val as f32)
                        .map_err(|_| ParserError::InvalidVertex)
                })
                .collect::<Result<Vec<f32>, ParserError>>()
            // TODO: check size
        }?;
        Color::try_from(color_float).map_err(|_| ParserError::InvalidVertex)
    }

    fn parse_faces(&mut self) -> ParserResult {
        for _ in 0..self.face_count {
            let (line_index, line) = self.lines.next().ok_or_else(|| ParserError::InvalidFace)?;
            let mut parts: Vec<&str> = line.split_line();

            let face = self.parse_face(line_index, parts)?;
            self.document.faces.push(face);
        }

        Ok(())
    }

    fn parse_face(&mut self, line_index: usize, mut parts: Vec<&str>) -> ParserResult<Face> {
        let vertex_count: usize = parts[0].parse().map_err(|_| ParserError::InvalidFace)?;
        parts = parts[1..].to_vec();

        // // sanity check
        // if face_str.len() != vertex_count as usize {
        //     return Err(ParserError::InvalidFace);
        // }

        // faces are polygons and might have to be triangulated later. Therefore we require at least three vertices
        // if vertex_count < 3 {
        //     return Err(ParserError::InvalidFace);
        // }
        let vertices = self.parse_face_index(line_index, vertex_count, parts.clone())?;

        let color = if parts.len() > 3 {
            Some(self.parse_color(line_index, parts[4..].to_vec())?)
        } else {
            None
        };
        Ok(Face { vertices, color })
    }

    fn parse_face_index(
        &mut self,
        line_index: usize,
        vertex_count: usize,
        parts: Vec<&str>,
    ) -> ParserResult<Vec<usize>> {
        // TODO: dont work if we have colors
        // if vertex_str.len() != 3 {
        //     return Err(ParserError::InvalidVertex);
        // }

        let vertices: Vec<usize> = parts
            .into_iter()
            .take(vertex_count)
            .map(|s| s.parse().map_err(|_| ParserError::InvalidFace))
            .collect::<Result<Vec<usize>, ParserError>>()?;

        Ok(vertices)
    }

    fn finalize(self) -> DocumentResult {
        Ok(self.document)
    }
}

trait StrParts<'a> {
    fn split_line(self) -> Vec<&'a str>;
}

impl<'a> StrParts<'a> for &'a str {
    fn split_line(self) -> Vec<&'a str> {
        self.split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map_while(|s| (!s.starts_with("#")).then(|| s))
            // .map_while(|s| (!s.starts_with("#")).then_some(s)); currently still unstable (https://github.com/rust-lang/rust/issues/80967)
            .collect()
    }
}

trait ConvertVec {
    fn convert_vec<T, G>(self) -> Result<Vec<T>, G>
    where
        T: FromStr<Err = G>;
}

impl ConvertVec for Vec<&str> {
    fn convert_vec<T, G>(self) -> Result<Vec<T>, G>
    where
        T: FromStr<Err = G>,
    {
        self.into_iter()
            .map(FromStr::from_str)
            .collect::<Result<Vec<T>, G>>()
    }
}

impl TryFrom<Vec<f32>> for Color {
    type Error = GeometryError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(GeometryError::ColorOutOfBounds);
        }

        let alpha = if value.len() == 4 { value[3] } else { 1.0 };

        Ok(Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: alpha,
        })
    }
}

impl TryFrom<Vec<f32>> for Position {
    type Error = GeometryError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(GeometryError::VertexOutOfBounds);
        }

        Ok(Self {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {

    use super::*;

    #[test]
    fn test_split_line() {
        assert_eq!("".split_line(), Vec::<&str>::new());
        assert_eq!("1 2 3".split_line(), vec!["1", "2", "3"]);
    }

    // #[test]
    // #[should_panic]
    // #[allow(unused)]
    // fn test_state() {
    //     let mut parser = DocumentParser {
    //         lines: vec!["OFF"],
    //         line: 0,
    //         state: ParserState::Header,
    //         document: OffDocument::new(),
    //     };
    //     parser.parse_counts();
    // }

    // #[test]
    // fn parse_header() {
    //     let mut parser = DocumentParser {
    //         lines: vec!["ignore", "me", "OFF", "!"],
    //         line: 2,
    //         state: ParserState::Header,
    //         document: OffDocument::new(),
    //     };
    //     assert!(matches!(parser.parse_header(), Ok(_)));
    //     let mut parser = DocumentParser {
    //         lines: vec!["OOFF"],
    //         line: 0,
    //         state: ParserState::Header,
    //         document: OffDocument::new(),
    //     };
    //     assert!(matches!(
    //         parser.parse_header(),
    //         Err(ParserError::InvalidHeader)
    //     ));
    // }

    // #[test]
    // fn parse_counts() {
    //     let mut parser = DocumentParser {
    //         lines: vec!["a12 3 4"],
    //         line: 0,
    //         state: ParserState::Counts,
    //         document: OffDocument::new(),
    //     };
    //     assert!(matches!(
    //         parser.parse_counts(),
    //         Err(ParserError::InvalidCounts)
    //     ));
    //     let mut parser = DocumentParser {
    //         lines: vec!["1 1337 42"],
    //         line: 0,
    //         state: ParserState::Counts,
    //         document: OffDocument::new(),
    //     };
    //     assert!(matches!(parser.parse_counts(), Ok(_)));
    //     assert_eq!(parser.document.vertex_count(), 1);
    //     assert_eq!(parser.document.face_count(), 1337);
    //     assert_eq!(parser.document.edge_count(), 42);
    // }
}
