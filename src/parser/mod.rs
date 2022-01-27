pub mod error;
mod iter;
pub mod options;
mod utils;

use crate::geometry::{
    color::Color,
    mesh::{Face, Mesh, Vertex},
    position::Position,
};

use self::{
    error::{Error, Kind},
    iter::OffLines,
    options::Options,
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
    options: Options,
}

impl<'a> Parser<'a> {
    pub fn new<S: AsRef<str>>(s: &'a S, options: Options) -> Self {
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

    pub fn parse(mut self) -> crate::Result {
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

        Position::try_from(position_parts).map_err(|err| {
            Error::with_message(
                Kind::InvalidVertexPosition,
                line_index,
                format!("Failed to parse position: ({})", err),
            )
        })
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

        if self.options.color_format.is_float() {
            // parse as f32
            let color_parts = parts
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
                .collect::<Result<Vec<f32>>>()?;

            Color::try_from(color_parts).map_err(|err| {
                Error::with_message(
                    Kind::InvalidColor,
                    line_index,
                    format!("Failed to parse color: {}", err),
                )
            })
        } else {
            // parse as u8
            let color_parts = parts
                .iter()
                .map(|s| {
                    s.parse::<u8>().map_err(|err| {
                        Error::with_message(
                            Kind::InvalidColor,
                            line_index,
                            format!("Failed to parse color as u8: {}", err),
                        )
                    })
                })
                .collect::<Result<Vec<u8>>>()?;

            Color::try_from(color_parts).map_err(|err| {
                Error::with_message(
                    Kind::InvalidColor,
                    line_index,
                    format!("Failed to parse color: {}", err),
                )
            })
        }
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

#[cfg(test)]
#[allow(unused)]
mod tests {

    use crate::geometry::color_format::ColorFormat;

    use super::*;

    #[test]
    fn parse_header() {
        let mut parser = Parser::new(&"OFF", Options::default());
        assert!(parser.parse_header().is_ok());
    }

    #[test]
    fn parse_header_missing() {
        let mut parser = Parser::new(&"", Options::default());
        let header = parser.parse_header();
        assert!(header.is_err());
        assert!(matches!(
            header.unwrap_err(),
            Error {
                kind: Kind::Empty,
                ..
            }
        ));
    }

    #[test]
    fn parse_header_invalid() {
        let mut parser = Parser::new(&"COFF", Options::default());
        let header = parser.parse_header();
        assert!(header.is_err());
        assert!(matches!(
            header.unwrap_err(),
            Error {
                kind: Kind::InvalidHeader,
                ..
            }
        ));
    }

    #[test]
    fn parse_counts() {
        let mut parser = Parser::new(&"8 6 12", Options::default());
        assert!(parser.parse_counts().is_ok());
        assert_eq!(parser.vertex_count, 8);
        assert_eq!(parser.face_count, 6);
        assert_eq!(parser.edge_count, 12);
    }

    #[test]
    fn parse_counts_missing() {
        let mut parser = Parser::new(&"", Options::default());
        let counts = parser.parse_counts();
        assert!(counts.is_err());
        assert!(matches!(
            counts.unwrap_err(),
            Error {
                kind: Kind::Missing,
                ..
            }
        ));
    }

    #[test]
    fn parse_counts_too_many() {
        let mut parser = Parser::new(&"8 6 12 16", Options::default());
        let counts = parser.parse_counts();
        assert!(counts.is_err());
        assert!(matches!(
            counts.unwrap_err(),
            Error {
                kind: Kind::InvalidCounts,
                ..
            }
        ));
    }

    #[test]
    fn parse_counts_limits() {
        let mut parser = Parser::new(&"999999999999 888888888888 777777777", Options::default());
        let counts = parser.parse_counts();
        assert!(counts.is_err());
        assert!(matches!(
            counts.unwrap_err(),
            Error {
                kind: Kind::LimitExceeded,
                ..
            }
        ));
    }

    #[test]
    fn parse_vertices() {
        let mut parser = Parser::new(
            &"3.0 1.0 2.0 0.1 0.2 0.3 1.0\n1.0 2.0 3.0 0.1 0.2 0.3 1.0",
            Options::default(),
        );
        parser.vertex_count = 2;
        let result = parser.parse_vertices();
        assert!(result.is_ok());
        assert!(parser.next_line().is_none());
        assert!(parser.document.vertices.len() == 2);
        assert!(
            parser.document.vertices[0]
                == Vertex::new(
                    Position::new(3.0, 1.0, 2.0),
                    Some(Color::new(0.1, 0.2, 0.3, 1.0).unwrap()),
                )
        );
        assert!(
            parser.document.vertices[1]
                == Vertex::new(
                    Position::new(1.0, 2.0, 3.0),
                    Some(Color::new(0.1, 0.2, 0.3, 1.0).unwrap()),
                )
        );
    }

    #[test]
    fn parse_vertex() {
        let mut parser = Parser::new(&"", Options::default());

        let vertex = parser.parse_vertex(0, &["1.0", "2.0", "3.0"]);
        assert!(vertex.is_ok());
        assert_eq!(
            vertex.unwrap(),
            Vertex::new(Position::new(1.0, 2.0, 3.0), None)
        );
    }

    #[test]
    fn parse_vertex_too_few_parts() {
        let mut parser = Parser::new(&"", Options::default());

        let vertex = parser.parse_vertex(0, &["1.0", "2.0"]);
        assert!(vertex.is_err());
        assert!(matches!(
            vertex.unwrap_err(),
            Error {
                kind: Kind::InvalidVertexPosition,
                ..
            }
        ));
    }

    #[test]
    fn parse_position() {
        let position = Parser::parse_position(0, &["1", "2", "3"]);
        assert!(position.is_ok());
        assert_eq!(
            position.unwrap(),
            Position {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn parse_position_no_number() {
        let position = Parser::parse_position(0, &["1", "2", "a"]);
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
    fn parse_position_too_few_parts() {
        let position = Parser::parse_position(0, &["1", "2"]);
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
    fn parse_position_too_many_parts() {
        let position = Parser::parse_position(0, &["1", "2", "3", "5"]);
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
    fn parse_color_rgbfloat() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBFloat,
                ..Options::default()
            },
        );
        let color = parser.parse_color(0, &["1.0", "0.5", "0.3"]);
        assert!(color.is_ok());
        assert_eq!(
            color.unwrap(),
            Color {
                r: 1.0,
                g: 0.5,
                b: 0.3,
                a: 1.0,
            }
        );
    }

    #[test]
    fn parse_color_rgbafloat() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBAFloat,
                ..Options::default()
            },
        );
        let color = parser.parse_color(0, &["1.0", "0.5", "0.3", "0.5"]);
        assert!(color.is_ok());
        assert_eq!(
            color.unwrap(),
            Color {
                r: 1.0,
                g: 0.5,
                b: 0.3,
                a: 0.5,
            }
        );
    }

    #[test]
    fn parse_color_rgbinterger() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBInteger,
                ..Options::default()
            },
        );
        let color = parser.parse_color(0, &["255", "128", "0"]);
        assert!(color.is_ok());
        assert_eq!(
            color.unwrap(),
            Color {
                r: 1.0,
                g: 0.501_960_8,
                b: 0.0,
                a: 1.0,
            }
        );
    }

    #[test]
    fn parse_color_rgbinterger_fail() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBInteger,
                ..Options::default()
            },
        );
        let color = parser.parse_color(0, &["255", "128.0", "0"]);
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
    fn parse_color_rgbainterger() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBAInteger,
                ..Options::default()
            },
        );
        let color = parser.parse_color(0, &["255", "128", "0", "255"]);
        assert!(color.is_ok());
        assert_eq!(
            color.unwrap(),
            Color {
                r: 1.0,
                g: 0.501_960_8,
                b: 0.0,
                a: 1.0,
            }
        );
    }

    #[test]
    fn parse_color_element_count() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBFloat,
                ..Options::default()
            },
        );
        let color = parser.parse_color(0, &["1.0", "0.5", "0.3", "0.4"]);
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
    fn parse_faces() {
        let mut parser = Parser::new(
            &"3 1 2 3 0.1 0.2 0.3 1.0\n3 3 2 1 0.2 0.3 0.4 1.0",
            Options::default(),
        );
        parser.face_count = 2;
        let result = parser.parse_faces();
        assert!(result.is_ok());
        assert!(parser.next_line().is_none());
        assert!(parser.document.faces.len() == 2);
        assert!(parser.document.faces[0].vertices == vec![1, 2, 3]);
        assert!(
            parser.document.faces[0].color
                == Some(Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                })
        );
        assert!(parser.document.faces[1].vertices == vec![3, 2, 1]);
        assert!(
            parser.document.faces[1].color
                == Some(Color {
                    r: 0.2,
                    g: 0.3,
                    b: 0.4,
                    a: 1.0,
                })
        );
    }

    #[test]
    fn parse_face() {
        let mut parser = Parser::new(&"", Options::default());
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
        let mut parser = Parser::new(&"", Options::default());
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
        let mut parser = Parser::new(&"", Options::default());
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
        let mut parser = Parser::new(&"", Options::default());
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
        let mut parser = Parser::new(&"", Options::default());
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

    #[test]
    fn parse_face_color() {
        let mut parser = Parser::new(&"", Options::default());
        let result = parser.parse_face(0, &["3", "1", "2", "3", "0.1", "0.2", "0.3", "0.4"]);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Face {
                vertices: vec![1, 2, 3],
                color: Some(Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 0.4
                })
            }
        );
    }

    #[test]
    fn parse_face_color_fail() {
        let mut parser = Parser::new(&"", Options::default());
        let result = parser.parse_face(0, &["3", "1", "2", "3", "0.1", "0.2"]);
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
    fn parse_face_color_fail_no_alpha() {
        let mut parser = Parser::new(
            &"",
            Options {
                color_format: ColorFormat::RGBFloat,
                ..Options::default()
            },
        );
        let result = parser.parse_face(0, &["3", "1", "2", "3", "0.1", "0.2", "0.3"]);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Face {
                vertices: vec![1, 2, 3],
                color: Some(Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0
                })
            }
        );
    }

    #[test]
    fn parse_face_color_fail_no_alpha_fail() {
        let mut parser = Parser::new(&"", Options::default());
        let result = parser.parse_face(0, &["3", "1", "2", "3", "0.1", "0.2", "0.3"]);
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
}
