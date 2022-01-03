use std::{error, result};

use thiserror::Error;

use crate::document::{DocumentError, DocumentResult, OffDocument, Vertex};

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
}

pub type ParserResult<T = ()> = Result<T, ParserError>;

#[derive(Debug, PartialEq)]
enum ParserState {
    Header,
    Counts,
    Vertices,
    Faces,
    End,
}

pub struct DocumentParser<'a> {
    lines: Vec<&'a str>,
    line: usize,
    state: ParserState,
    document: OffDocument,
}

impl DocumentParser<'_> {
    pub(crate) fn parse(string: &str) -> DocumentResult {
        let lines = Self::preprocess(string);

        let mut parser = DocumentParser {
            lines: lines,
            line: 0,
            state: ParserState::Header,
            document: OffDocument::new(),
        };

        parser.parse_header()?;
        parser.parse_counts()?;
        parser.parse_vertices()?;

        Ok(parser.document)
    }

    // filters empty lines, comments and returns a vector
    fn preprocess(string: &str) -> Vec<&str> {
        Self::split(string)
    }

    fn pop(&mut self) -> &str {
        let res = self.lines[self.line];
        self.line += 1;
        res
    }

    fn split(string: &str) -> Vec<&str> {
        string
            .split(" ")
            .filter(|s| !s.is_empty())
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

        if self.pop() != "OFF" {
            return Err(ParserError::InvalidHeader);
        }

        self.state = self.next_state();
        Ok(())
    }

    fn parse_counts(&mut self) -> ParserResult {
        assert_eq!(self.state, ParserState::Counts, "State mismatch");

        let counts: Vec<&str> = Self::split(self.pop());

        let num: Vec<u32> = counts
            .into_iter()
            .map(|s| s.parse().map_err(|_| ParserError::InvalidCounts))
            .collect::<Result<Vec<u32>, ParserError>>()?;

        self.document.vertex_count = num[0];
        if num.len() > 1 {
            self.document.face_count = num[1];
        }
        if num.len() > 2 {
            self.document.edge_count = num[2];
        }

        self.state = self.next_state();
        Ok(())
    }

    fn parse_vertices(&mut self) -> ParserResult {
        assert_eq!(self.state, ParserState::Vertices, "State mismatch");

        for _ in 0..self.document.vertex_count {
            let counts: Vec<&str> = Self::split(self.pop());

            if counts.len() != 3 {
                return Err(ParserError::InvalidVertex);
            }

            let vertex: Vec<i32> = counts
                .into_iter()
                .map(|s| s.parse().map_err(|_| ParserError::InvalidVertex))
                .collect::<Result<Vec<i32>, ParserError>>()?;

            self.document.vertices.push(Vertex::from(vertex))

        }

        self.state = self.next_state();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn preprocess() {
        assert_eq!(
            DocumentParser::preprocess("Hello\n# this is a test\nWorld\n\n\n!"),
            vec!["Hello", "World", "!"]
        );
    }

    #[test]
    #[should_panic]
    #[allow(unused)]
    fn test_state() {
        let mut parser = DocumentParser {
            lines: vec!["OFF"],
            line: 0,
            state: ParserState::Header,
            document: OffDocument::new(),
        };
        parser.parse_counts();
    }

    #[test]
    fn parse_header() {
        let mut parser = DocumentParser {
            lines: vec!["ignore", "me", "OFF", "!"],
            line: 2,
            state: ParserState::Header,
            document: OffDocument::new(),
        };
        assert!(matches!(parser.parse_header(), Ok(_)));
        let mut parser = DocumentParser {
            lines: vec!["OOFF"],
            line: 0,
            state: ParserState::Header,
            document: OffDocument::new(),
        };
        assert!(matches!(
            parser.parse_header(),
            Err(ParserError::InvalidHeader)
        ));
    }

    #[test]
    fn parse_counts() {
        let mut parser = DocumentParser {
            lines: vec!["a12 3 4"],
            line: 0,
            state: ParserState::Counts,
            document: OffDocument::new(),
        };
        assert!(matches!(
            parser.parse_counts(),
            Err(ParserError::InvalidCounts)
        ));
        let mut parser = DocumentParser {
            lines: vec!["1 1337 42"],
            line: 0,
            state: ParserState::Counts,
            document: OffDocument::new(),
        };
        assert!(matches!(parser.parse_counts(), Ok(_)));
        assert_eq!(parser.document.vertex_count, 1);
        assert_eq!(parser.document.face_count, 1337);
        assert_eq!(parser.document.edge_count, 42);
    }
}
