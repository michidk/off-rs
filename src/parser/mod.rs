pub mod error;
mod iter;
mod utils;

use crate::{
    document::{DocumentResult, OffDocument, ParserOptions},
    geometry::{Color, Face, Position, Vertex},
};

use self::{
    error::{ParserError, ParserErrorKind},
    iter::OffLines,
    utils::{ConvertVec, StrParts},
};

pub type ParserResult<T = ()> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct DocumentParser<'a> {
    lines: OffLines<'a>,
    prev_line_index: usize,
    vertex_count: usize,
    face_count: usize,
    edge_count: usize,
    document: OffDocument,
    options: ParserOptions,
}

impl<'a> DocumentParser<'a> {
    pub fn new<S: AsRef<str>>(s: &'a S, options: ParserOptions) -> Self {
        let lines = OffLines::new(s.as_ref());

        DocumentParser {
            lines,
            prev_line_index: 0,
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

    fn next_line(&mut self) -> Option<(usize, &'a str)> {
        let (line_index, line) = self.lines.next()?;

        self.prev_line_index = line_index;

        Some((line_index, line))
    }

    fn parse_header(&mut self) -> ParserResult {
        let (line_index, line) = self
            .next_line()
            .ok_or_else(|| ParserError::without_message(ParserErrorKind::Empty, 0))?;

        if line != "OFF" {
            return Err(ParserError::with_message(
                ParserErrorKind::InvalidHeader,
                line_index,
                "First non-comment line should be `OFF`",
            ));
        }

        Ok(())
    }

    fn parse_counts(&mut self) -> ParserResult {
        let (line_index, line) = self.next_line().ok_or_else(|| {
            ParserError::with_message(
                ParserErrorKind::Missing,
                self.prev_line_index + 1,
                "No counts present",
            )
        })?;

        let counts: Vec<&str> = line.split_line();

        // let num: Vec<usize> = counts
        //     .into_iter()
        //     .map(|s| {
        //         s.parse().map_err(|err| {
        //             ParserError::with_message(
        //                 ParserErrorKind::InvalidCounts,
        //                 line_index,
        //                 format!("Failed to parse count as number ({})", err),
        //             )
        //         })
        //     })
        //     .collect::<Result<Vec<usize>, ParserError>>()?;

        let num: Vec<usize> = counts.convert_vec().map_err(|err| {
            ParserError::with_message(
                ParserErrorKind::InvalidCounts,
                line_index,
                format!("Failed to parse count as number ({})", err),
            )
        })?;

        match num[..] {
            [vertex_count, face_count, edge_count] => {
                self.vertex_count = vertex_count;
                self.face_count = face_count;
                self.edge_count = edge_count;
            }
            [vertex_count, face_count] => {
                self.vertex_count = vertex_count;
                self.face_count = face_count;
            }
            _ => {
                return Err(ParserError::with_message(
                    ParserErrorKind::InvalidCounts,
                    line_index,
                    format!(
                        "Invalid amount of counts present (expected: 2-3, actual: {})",
                        num.len()
                    ),
                ));
            }
        }

        // Check for limits
        if self.vertex_count > self.options.limits.vertex_count {
            return Err(ParserError::with_message(
                ParserErrorKind::LimitExceeded,
                line_index,
                format!(
                    "Vertext count exceeds limit (limit: {}, actual: {})",
                    self.options.limits.vertex_count, self.vertex_count
                ),
            ));
        }

        if self.face_count > self.options.limits.face_count {
            return Err(ParserError::with_message(
                ParserErrorKind::LimitExceeded,
                line_index,
                format!(
                    "Face count exceeds limit (limit: {}, actual: {})",
                    self.options.limits.face_count, self.face_count
                ),
            ));
        }

        Ok(())
    }

    fn parse_vertices(&mut self) -> ParserResult {
        for _ in 0..self.vertex_count {
            let (line_index, line) = self.next_line().ok_or_else(|| {
                ParserError::with_message(
                    ParserErrorKind::Missing,
                    self.prev_line_index + 1,
                    "Expected vertex definition",
                )
            })?;

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

        if parts.len() < 3 {
            return Err(ParserError::with_message(
                ParserErrorKind::InvalidVertexPosition,
                line_index,
                format!(
                    "Not enough parts for position (expected: >= 3, actual: {})",
                    parts.len()
                ),
            ));
        }

        let position = self.parse_position(line_index, &parts[0..=2])?;

        let color = if parts.len() > 3 {
            Some(self.parse_color(line_index, &parts[3..])?)
        } else {
            None
        };

        Ok(Vertex { position, color })
    }

    fn parse_position(&mut self, line_index: usize, parts: &[&str]) -> ParserResult<Position> {
        if parts.len() != 3 {
            return Err(ParserError::with_message(
                ParserErrorKind::InvalidVertexPosition,
                line_index,
                format!(
                    "Invalid number of coordinates given (expected: 3, actual: {})",
                    parts.len()
                ),
            ));
        }

        let position_parts: Vec<f32> = parts
            .iter()
            .map(|s| {
                s.parse().map_err(|err| {
                    ParserError::with_message(
                        ParserErrorKind::InvalidVertexPosition,
                        line_index,
                        format!("Failed to parse coordinate as number: ({})", err),
                    )
                })
            })
            .collect::<Result<Vec<f32>, ParserError>>()?;

        // TODO: should not be try_from as we check for validity above?
        Position::try_from(position_parts)
    }

    fn parse_color(&mut self, line_index: usize, parts: &[&str]) -> ParserResult<Color> {
        if parts.len() != self.options.color_format.element_count() {
            return Err(ParserError::with_message(
                ParserErrorKind::InvalidColor,
                line_index,
                format!(
                    "Invalid number of color elements given (expected: {}, actual: {})",
                    self.options.color_format.element_count(),
                    parts.len()
                ),
            ));
        }

        let color_float = if self.options.color_format.is_float() {
            // directly parse as f32
            parts
                .iter()
                .map(|s| {
                    s.parse::<f32>().map_err(|err| {
                        ParserError::with_message(
                            ParserErrorKind::InvalidColor,
                            line_index,
                            format!("Failed to parse color as float: {}", err),
                        )
                    })
                })
                .collect::<Result<Vec<f32>, ParserError>>()
        } else {
            // parse as u8 and convert to f32
            parts
                .iter()
                .map(|s| {
                    s.parse::<u8>().map(|val| val as f32).map_err(|err| {
                        ParserError::with_message(
                            ParserErrorKind::InvalidColor,
                            line_index,
                            format!("Failed to parse color as u8: {}", err),
                        )
                    })
                })
                .collect::<Result<Vec<f32>, ParserError>>()
        }?;

        Color::try_from(color_float).map_err(|err| {
            ParserError::with_message(
                ParserErrorKind::InvalidColor,
                line_index,
                format!("Failed to parse color: {}", err),
            )
        })
    }

    fn parse_faces(&mut self) -> ParserResult {
        for _ in 0..self.face_count {
            let (line_index, line) = self.next_line().ok_or_else(|| {
                ParserError::with_message(
                    ParserErrorKind::Missing,
                    self.prev_line_index + 1,
                    "Expected face definition",
                )
            })?;

            let parts: Vec<&str> = line.split_line();
            let face = self.parse_face(line_index, &parts)?;
            self.document.faces.push(face);
        }

        Ok(())
    }

    fn parse_face(&mut self, line_index: usize, mut parts: &[&str]) -> ParserResult<Face> {
        if parts.len() < 4 {
            return Err(ParserError::with_message(
                ParserErrorKind::InvalidFace,
                line_index,
                format!("Not enough arguments. At least three vertex indicies required (e.g. `3 1 2 3`). {} arguments given", parts.len()),
            ));
        }

        let vertex_count: usize = parts[0].parse().map_err(|err| {
            ParserError::with_message(
                ParserErrorKind::InvalidFace,
                line_index,
                format!("Failed to parse vertex count for face definition: {}", err),
            )
        })?;

        if vertex_count > self.options.limits.face_vertex_count {
            return Err(ParserError::with_message(
                ParserErrorKind::LimitExceeded,
                line_index,
                format!(
                    "Vertex count of face exceeds limit (limit: {}, actual: {})",
                    self.options.limits.face_vertex_count, vertex_count
                ),
            ));
        }

        // "Consume" vertex_count
        parts = &parts[1..];

        // // sanity check
        // if face_str.len() != vertex_count as usize {
        //     return Err(ParserError::InvalidFace);
        // }

        // faces are polygons and might have to be triangulated later. Therefore we require at least three vertices
        // if vertex_count < 3 {
        //     return Err(ParserError::InvalidFace);
        // }
        let vertices = self.parse_face_index(line_index, vertex_count, parts)?;

        // "Consume" vertex indexes
        parts = &parts[vertex_count..];

        let color = if !parts.is_empty() {
            Some(self.parse_color(line_index, parts)?)
        } else {
            None
        };

        Ok(Face { vertices, color })
    }

    fn parse_face_index(
        &mut self,
        line_index: usize,
        vertex_count: usize,
        parts: &[&str],
    ) -> ParserResult<Vec<usize>> {
        // TODO: dont work if we have colors
        // if vertex_str.len() != 3 {
        //     return Err(ParserError::InvalidVertex);
        // }

        let vertices: Vec<usize> = parts
            .iter()
            .take(vertex_count)
            .map(|s| {
                s.parse().map_err(|err| {
                    ParserError::with_message(
                        ParserErrorKind::InvalidFace,
                        line_index,
                        format!("Failed to parse vertex index as number: ({})", err),
                    )
                })
            })
            .collect::<Result<Vec<usize>, ParserError>>()?;

        if vertices.len() != vertex_count {
            return Err(ParserError::with_message(
                ParserErrorKind::InvalidFaceIndex,
                line_index,
                format!(
                    "Invalid number of face indexes given (expected: {}, actual: {})",
                    vertex_count,
                    vertices.len()
                ),
            ));
        }

        Ok(vertices)
    }

    fn finalize(self) -> DocumentResult {
        Ok(self.document)
    }
}

impl TryFrom<Vec<f32>> for Color {
    type Error = ParserError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(Self::Error::with_message(
                ParserErrorKind::InvalidColor,
                0,
                format!(
                    "Invalid amount of arguments (expected: 3-4, actual: {})",
                    value.len()
                ),
            ));
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
    type Error = ParserError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(Self::Error::with_message(
                ParserErrorKind::InvalidVertexPosition,
                0,
                format!(
                    "Invalid amount of arguments (expected: 3, actual: {})",
                    value.len()
                ),
            ));
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
