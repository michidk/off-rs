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

pub type DocumentResult<T, D = OffDocument<T>> = Result<D, DocumentError>;

#[derive(Clone, PartialEq, Debug)]
pub struct OffDocument<T = f32> {
    pub vertices: Vec<Vertex<T>>,
    pub faces: Vec<Face<T>>,
}

impl<T> OffDocument<T> {
    pub(crate) fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn from_path(path: &Path) -> DocumentResult<T> {
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

    fn parse(string: &str) -> DocumentResult<T> {
        DocumentParser::new(&string).try_parse()
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

impl<T> FromStr for OffDocument<T> {
    type Err = DocumentError;

    fn from_str(string: &str) -> DocumentResult<T> {
        Self::parse(string)
    }
}

impl<T> Default for OffDocument<T> {
    fn default() -> Self {
        Self::new()
    }
}
