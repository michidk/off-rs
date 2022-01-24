pub mod error;
mod iter;
mod utils;

use crate::{
    geometry::{Color, Face, Position, Vertex},
    mesh::{Mesh, ParserOptions},
};

use self::{
    error::{Error, Kind},
    iter::OffLines,
    utils::{ConvertVec, StrParts},
};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lines: OffLines<'a>,
    prev_line_index: usize,
    vertex_count: usize,
    face_count: usize,
    edge_count: usize,
    document: Mesh,
    options: ParserOptions,
}

impl<'a> Parser<'a> {
    pub fn new<S: AsRef<str>>(s: &'a S, options: ParserOptions) -> Self {
        let lines = OffLines::new(s.as_ref());

        Parser {
            lines,
            prev_line_index: 0,
            vertex_count: 0,
            face_count: 0,
            edge_count: 0,
            document: Mesh::new(),
            options,
        }
    }

    pub fn parse(mut self) -> crate::mesh::Result {
        self.parse_header()?;
        self.parse_counts()?;
        self.parse_vertices()?;
        self.parse_faces()?;

        // TODO: valitdate the counts

        Ok(self.finalize())
    }

    fn next_line(&mut self) -> Option<(usize, &'a str)> {
        let (line_index, line) = self.lines.next()?;

        self.prev_line_index = line_index;

        Some((line_index, line))
    }

    fn parse_header(&mut self) -> Result {
        let (line_index, line) = self
            .next_line()
            .ok_or_else(|| Error::without_message(Kind::Empty, 0))?;

        if line != "OFF" {
            return Err(Error::with_message(
                Kind::InvalidHeader,
                line_index,
                "First non-comment line should be `OFF`",
            ));
        }

        Ok(())
    }

    fn parse_counts(&mut self) -> Result {
        let (line_index, line) = self.next_line().ok_or_else(|| {
            Error::with_message(Kind::Missing, self.prev_line_index + 1, "No counts present")
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
            Error::with_message(
                Kind::InvalidCounts,
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
                return Err(Error::with_message(
                    Kind::InvalidCounts,
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
            return Err(Error::with_message(
                Kind::LimitExceeded,
                line_index,
                format!(
                    "Vertext count exceeds limit (limit: {}, actual: {})",
                    self.options.limits.vertex_count, self.vertex_count
                ),
            ));
        }

        if self.face_count > self.options.limits.face_count {
            return Err(Error::with_message(
                Kind::LimitExceeded,
                line_index,
                format!(
                    "Face count exceeds limit (limit: {}, actual: {})",
                    self.options.limits.face_count, self.face_count
                ),
            ));
        }

        Ok(())
    }

    fn parse_vertices(&mut self) -> Result {
        for _ in 0..self.vertex_count {
            let (line_index, line) = self.next_line().ok_or_else(|| {
                Error::with_message(
                    Kind::Missing,
                    self.prev_line_index + 1,
                    "Expected vertex definition",
                )
            })?;

            let parts = line.split_line();
            let vertex = self.parse_vertex(line_index, &parts)?;
            self.document.vertices.push(vertex);
        }

        Ok(())
    }

    fn parse_vertex(&mut self, line_index: usize, parts: &[&str]) -> Result<Vertex> {
        if parts.len() < 3 {
            return Err(Error::with_message(
                Kind::InvalidVertexPosition,
                line_index,
                format!(
                    "Not enough parts for position (expected: >= 3, actual: {})",
                    parts.len()
                ),
            ));
        }

        let position = Parser::parse_position(line_index, &parts[0..=2])?;

        let color = if parts.len() > 3 {
            Some(self.parse_color(line_index, &parts[3..])?)
        } else {
            None
        };

        Ok(Vertex { position, color })
    }

    fn parse_position(line_index: usize, parts: &[&str]) -> Result<Position> {
        if parts.len() != 3 {
            return Err(Error::with_message(
                Kind::InvalidVertexPosition,
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
                    Error::with_message(
                        Kind::InvalidVertexPosition,
                        line_index,
                        format!("Failed to parse coordinate as number: ({})", err),
                    )
                })
            })
            .collect::<Result<Vec<f32>>>()?;

        Position::try_from(position_parts)
    }

    fn parse_color(&mut self, line_index: usize, parts: &[&str]) -> Result<Color> {
        if parts.len() != self.options.color_format.element_count() {
            return Err(Error::with_message(
                Kind::InvalidColor,
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
                        Error::with_message(
                            Kind::InvalidColor,
                            line_index,
                            format!("Failed to parse color as float: {}", err),
                        )
                    })
                })
                .collect::<Result<Vec<f32>>>()
        } else {
            // parse as u8 and convert to f32
            parts
                .iter()
                .map(|s| {
                    s.parse::<u8>().map(f32::from).map_err(|err| {
                        Error::with_message(
                            Kind::InvalidColor,
                            line_index,
                            format!("Failed to parse color as u8: {}", err),
                        )
                    })
                })
                .collect::<Result<Vec<f32>>>()
        }?;

        Color::try_from(color_float).map_err(|err| {
            Error::with_message(
                Kind::InvalidColor,
                line_index,
                format!("Failed to parse color: {}", err),
            )
        })
    }

    fn parse_faces(&mut self) -> Result {
        for _ in 0..self.face_count {
            let (line_index, line) = self.next_line().ok_or_else(|| {
                Error::with_message(
                    Kind::Missing,
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

    fn parse_face(&mut self, line_index: usize, mut parts: &[&str]) -> Result<Face> {
        if parts.len() < 4 {
            return Err(Error::with_message(
                Kind::InvalidFace,
                line_index,
                format!("Not enough arguments. At least three vertex indicies required (e.g. `3 1 2 3`). {} arguments given", parts.len()),
            ));
        }

        let vertex_count: usize = parts[0].parse().map_err(|err| {
            Error::with_message(
                Kind::InvalidFace,
                line_index,
                format!("Failed to parse vertex count for face definition: {}", err),
            )
        })?;

        if vertex_count > self.options.limits.face_vertex_count {
            return Err(Error::with_message(
                Kind::LimitExceeded,
                line_index,
                format!(
                    "Vertex count of face exceeds limit (limit: {}, actual: {})",
                    self.options.limits.face_vertex_count, vertex_count
                ),
            ));
        }

        // "Consume" vertex_count
        parts = &parts[1..];

        // TODO
        // // sanity check
        // if face_str.len() != vertex_count as usize {
        //     return Err(ParserError::InvalidFace);
        // }

        // TODO
        // faces are polygons and might have to be triangulated later. Therefore we require at least three vertices
        // if vertex_count < 3 {
        //     return Err(ParserError::InvalidFace);
        // }
        let vertices = Parser::parse_face_indices(line_index, vertex_count, parts)?;

        // "Consume" vertex indexes
        parts = &parts[vertex_count..];

        let color = if parts.is_empty() {
            None
        } else {
            Some(self.parse_color(line_index, parts)?)
        };

        Ok(Face { vertices, color })
    }

    fn parse_face_indices(
        line_index: usize,
        vertex_count: usize,
        parts: &[&str],
    ) -> Result<Vec<usize>> {
        let vertices: Vec<usize> = parts
            .iter()
            .take(vertex_count)
            .map(|s| {
                s.parse().map_err(|err| {
                    Error::with_message(
                        Kind::InvalidFaceIndex,
                        line_index,
                        format!("Failed to parse vertex index as number: ({})", err),
                    )
                })
            })
            .collect::<Result<Vec<usize>>>()?;

        if vertices.len() != vertex_count {
            return Err(Error::with_message(
                Kind::InvalidFaceIndex,
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

    fn finalize(self) -> Mesh {
        self.document
    }
}

impl TryFrom<Vec<f32>> for Color {
    type Error = Error;

    fn try_from(value: Vec<f32>) -> std::result::Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(Self::Error::with_message(
                Kind::InvalidColor,
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
    type Error = Error;

    fn try_from(value: Vec<f32>) -> std::result::Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(Self::Error::with_message(
                Kind::InvalidVertexPosition,
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

    // TODO: test parse_header
    // TODO: test parse_counts
    // TODO: test parse_vertices
    // TODO: test parse_vertex
    // TODO: test DocumentParser::parse_position
    // TODO: test parse_color
    // TODO: test parse_Faces
    // TODO: test parse_face

    #[test]
    fn parse_face() {
        let mut parser = Parser::new(&"", ParserOptions::default());
        let result = parser.parse_face(0, &["3", "1", "2", "3"]);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Face {
                vertices: vec![1, 2, 3],
                color: None
            }
        );
    }

    #[test]
    fn parse_face_more() {
        let mut parser = Parser::new(&"", ParserOptions::default());
        let result = parser.parse_face(0, &["4", "2", "3", "1", "1337"]);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Face {
                vertices: vec![2, 3, 1, 1337],
                color: None
            }
        );
    }

    #[test]
    fn parse_face_too_little_parts() {
        let mut parser = Parser::new(&"", ParserOptions::default());
        let result = parser.parse_face(0, &["6", "1", "2", "3"]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error {
                kind: Kind::InvalidFaceIndex,
                ..
            }
        ));
    }

    #[test]
    fn parse_face_too_many_parts() {
        let mut parser = Parser::new(&"", ParserOptions::default());
        let result = parser.parse_face(0, &["3", "2", "3", "2", "3"]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error {
                kind: Kind::InvalidColor,
                ..
            }
        ));
    }

    #[test]
    fn parse_face_no_number() {
        let mut parser = Parser::new(&"", ParserOptions::default());
        let result = parser.parse_face(0, &["3", "1", "asdf", "3"]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error {
                kind: Kind::InvalidFaceIndex,
                ..
            }
        ));
    }

    // TODO: face colors

    #[test]
    fn parse_face_index() {
        let result = Parser::parse_face_indices(0, 3, &["1", "2", "3"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn parse_face_index_more() {
        let result = Parser::parse_face_indices(0, 5, &["1", "2", "3", "1", "1337"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 1, 1337]);
    }

    #[test]
    fn parse_face_index_too_little_parts() {
        let result = Parser::parse_face_indices(0, 5, &["1", "2", "3"]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error {
                kind: Kind::InvalidFaceIndex,
                ..
            }
        ));
    }

    #[test]
    fn parse_face_index_too_many_parts() {
        let result = Parser::parse_face_indices(0, 3, &["1", "2", "3", "2", "3"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn parse_face_index_no_number() {
        let result = Parser::parse_face_indices(0, 3, &["1", "asdf", "3"]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error {
                kind: Kind::InvalidFaceIndex,
                ..
            }
        ));
    }

    #[test]
    fn try_from_color_rgb() {
        let vec = vec![1.0, 2.0, 3.0];
        let color = Color::try_from(vec);
        assert!(color.is_ok());
        assert_eq!(color.unwrap(), Color::new(1.0, 2.0, 3.0, 1.0));
    }

    #[test]
    fn try_from_color_rgba() {
        let vec = vec![1.0, 2.0, 3.0, 4.0];
        let color = Color::try_from(vec);
        assert!(color.is_ok());
        assert_eq!(color.unwrap(), Color::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn try_from_color_err_too_little_arguments() {
        let vec = vec![1.0, 2.0];
        let color = Color::try_from(vec);
        assert!(color.is_err());
        assert!(matches!(
            color.unwrap_err(),
            Error {
                kind: Kind::InvalidColor,
                ..
            }
        ));
    }

    #[test]
    fn try_from_color_err_too_many_arguments() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let color = Color::try_from(vec);
        assert!(color.is_err());
        assert!(matches!(
            color.unwrap_err(),
            Error {
                kind: Kind::InvalidColor,
                ..
            }
        ));
    }

    #[test]
    fn try_from_positiom() {
        let vec = vec![1.0, 2.0, 3.0];
        let position = Position::try_from(vec);
        assert!(position.is_ok());
        assert_eq!(position.unwrap(), Position::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn try_from_positiom_too_little_arguments() {
        let vec = vec![1.0, 2.0];
        let position = Position::try_from(vec);
        assert!(position.is_err());
        assert!(matches!(
            position.unwrap_err(),
            Error {
                kind: Kind::InvalidVertexPosition,
                ..
            }
        ));
    }

    #[test]
    fn try_from_positiom_too_many_arguments() {
        let vec = vec![1.0, 2.0, 3.0, 4.0];
        let position = Position::try_from(vec);
        assert!(position.is_err());
        assert!(matches!(
            position.unwrap_err(),
            Error {
                kind: Kind::InvalidVertexPosition,
                ..
            }
        ));
    }
}
