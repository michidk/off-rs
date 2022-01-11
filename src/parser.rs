use std::{str::{FromStr, Lines}, iter::{Enumerate, Peekable}};

use thiserror::Error;

use crate::{
    document::{DocumentResult, OffDocument},
    geometry::{Face, GeometryError, Vertex},
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


// TODO: error kind that we have line index and message
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("First line should be `OFF`")]
    InvalidHeader,
    #[error(
        "Invalid count format. Second line should be: `<verticex count> <face count> <edge count>`"
    )]
    InvalidCounts,
    #[error("Vertex has wrong format. Should be: `<x> <y> <z>`")]
    InvalidVertex,
    #[error("Face has wrong format. Should be: `<vertex count> <vertex index>*`")]
    InvalidFace,
    #[error("Invalid geometry")]
    GeometryError(#[from] GeometryError),
}

pub type ParserResult<T = ()> = Result<T, ParserError>;

#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    Header,
    Counts,
    Vertices,
    Faces,
    End,
}

#[derive(Debug, Clone)]
pub struct DocumentParser<'a> {
    lines: Peekable<OffLines<'a>>,
    vertex_count: usize,
    face_count: usize,
    edge_count: usize,
    state: ParserState,
    document: OffDocument<T>,
}

impl<'a> DocumentParser<'a> {

    pub fn new<S: AsRef<str>>(s: &'a S) -> Self {
        let lines = OffLines::new(s.as_ref()).peekable();

        DocumentParser {
            lines,
            vertex_count: 0,
            face_count: 0,
            edge_count: 0,
            state: ParserState::Header,
            document: OffDocument<T>::new(),
        }
    }

    pub fn try_parse(mut self) -> DocumentResult<T> {
        self.parse_header()?;
        self.parse_counts()?;
        self.parse_vertices()?;
        self.parse_faces()?;

        // TODO: valitdate the counts

        self.finalize()
    }

    fn split(string: &str) -> Vec<&str> {
        string
            .split_whitespace()
            .filter(|s| !s.starts_with("#"))
            .map(|s| s.trim())
            .collect()
    }

    fn next_state(&self) -> ParserState {
        match self.state {
            ParserState::Header => ParserState::Counts,
            ParserState::Counts => ParserState::Vertices,
            ParserState::Vertices => ParserState::Faces,
            ParserState::Faces => ParserState::End,
            ParserState::End => panic!("There is no state after the end state"),
        }
    }

    fn parse_header(&mut self) -> ParserResult {
        assert_eq!(self.state, ParserState::Header, "State mismatch");

        let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| ParserError::InvalidHeader)?;

        if line != "OFF" {
            return Err(ParserError::InvalidHeader);
        }

        self.state = self.next_state();
        Ok(())
    }

    fn parse_counts(&mut self) -> ParserResult {
        assert_eq!(self.state, ParserState::Counts, "State mismatch");

        let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| ParserError::InvalidCounts)?;
        let counts: Vec<&str> = Self::split(line);

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

        self.state = self.next_state();
        Ok(())
    }

    fn parse_vertices(&mut self) -> ParserResult {
        assert_eq!(self.state, ParserState::Vertices, "State mismatch");

        for _ in 0..self.vertex_count {
            let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| ParserError::InvalidVertex)?;
            let vertex_str: Vec<&str> = Self::split(line);

            if vertex_str.len() != 3 {
                return Err(ParserError::InvalidVertex);
            }

            let vertex: Vec<f32> = vertex_str
                .into_iter()
                .map(|s| s.parse().map_err(|_| ParserError::InvalidVertex))
                .collect::<Result<Vec<f32>, ParserError>>()?;

            self.document.vertices.push(Vertex::try_from(vertex)?)
        }

        self.state = self.next_state();
        Ok(())
    }

    fn parse_faces(&mut self) -> ParserResult {
        assert_eq!(self.state, ParserState::Faces, "State mismatch");

        for _ in 0..self.face_count {
            let (line_index, line) = self
                .lines
                .next()
                .ok_or_else(|| ParserError::InvalidFace)?;
            let mut face_str: Vec<&str> = Self::split(line);

            let vertex_count: u32 = face_str[0].parse().map_err(|_| ParserError::InvalidFace)?;
            face_str = face_str.into_iter().skip(1).collect();

            // // sanity check
            // if face_str.len() != vertex_count as usize {
            //     return Err(ParserError::InvalidFace);
            // }

            // faces are polygons and might have to be triangulated later. Therefore we require at least three vertices
            // if vertex_count < 3 {
            //     return Err(ParserError::InvalidFace);
            // }

            let face: Vec<usize> = face_str
                .into_iter()
                .map(|s| s.parse().map_err(|_| ParserError::InvalidFace))
                .collect::<Result<Vec<usize>, ParserError>>()?;

            self.document.faces.push(Face::try_from(face)?);
        }

        self.state = self.next_state();
        Ok(())
    }

    fn finalize(self) -> DocumentResult {
        assert_eq!(self.state, ParserState::End, "State mismatch");

        Ok(self.document)
    }
}

trait StrParts<'a> {
    fn parts(self) -> Vec<&'a str>;
}

impl<'a> StrParts<'a> for &'a str {
    fn parts(self) -> Vec<&'a str> {
        self.split_whitespace()
            .filter(|s| !s.is_empty())
            .filter(|s| !s.starts_with("#"))
            .map(|s| s.trim())
            .collect()
    }
}

trait ConvertVec {
    fn convert_vec<T, G>(self) -> Result<Vec<T>, G> where T: FromStr<Err = G>;
}

impl ConvertVec for Vec<&str> {
    fn convert_vec<T, G>(self) -> Result<Vec<T>, G> where T: FromStr<Err = G> {
        self.into_iter()
            .map(FromStr::from_str)
            .collect::<Result<Vec<T>, G>>()
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
