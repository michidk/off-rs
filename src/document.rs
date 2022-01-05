use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

use crate::geometry::{Face, Vertex};
use crate::parser::{DocumentParser, ParserError};

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Parser Error")]
    ParserError(#[from] ParserError),
}

pub type DocumentResult<T = OffDocument> = Result<T, DocumentError>;

#[derive(Clone, PartialEq, Debug)]
pub struct OffDocument {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl OffDocument {
    pub(crate) fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn from_path(path: &Path) -> DocumentResult {
        let file_result = File::open(path);

        let mut file = match file_result {
            Ok(file) => file,
            Err(inner) => return Err(DocumentError::IOError(inner)),
        };

        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Ok(_) => {}
            Err(inner) => return Err(DocumentError::IOError(inner)),
        };

        Self::parse(&string)
    }

    fn parse(string: &str) -> DocumentResult {
        DocumentParser::parse(string)
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn face_count(&self) -> usize {
        self.faces.len()
    }

    fn edge_count(&self) -> usize {
        self.faces.iter().map(|face| face.vertices.len() - 1).sum()
    }
}

impl FromStr for OffDocument {
    type Err = DocumentError;

    fn from_str(string: &str) -> DocumentResult {
        Self::parse(string)
    }
}

impl Default for OffDocument {
    fn default() -> Self {
        Self::new()
    }
}
