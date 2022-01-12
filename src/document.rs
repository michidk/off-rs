use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use crate::geometry::{ColorFormat, Face, Vertex};
use crate::parser::{DocumentParser, ParserError};

#[derive(Debug)]
pub enum DocumentError {
    IOError(io::Error),
    ParserError(ParserError),
}

impl std::error::Error for DocumentError { }

impl std::fmt::Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DocumentError::IOError(e) => write!(f, "IO Error: {}", e),
            DocumentError::ParserError(e) => write!(f, "Parser Error: {}", e),
        }
    }
}

impl From<io::Error> for DocumentError {
    fn from(e: io::Error) -> Self {
        DocumentError::IOError(e)
    }
}

impl From<ParserError> for DocumentError {
    fn from(e: ParserError) -> Self {
        DocumentError::ParserError(e)
    }
}

pub type DocumentResult<D = OffDocument> = Result<D, DocumentError>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParserOptions {
    pub color_format: ColorFormat,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            color_format: Default::default(),
        }
    }
}

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

    pub fn from_path(path: &Path, options: ParserOptions) -> DocumentResult {
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

        Self::parse(&string, options)
    }

    pub fn parse(string: &str, options: ParserOptions) -> DocumentResult {
        DocumentParser::new(&string, options).parse()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn edge_count(&self) -> usize {
        self.faces.iter().map(|face| face.vertices.len() - 1).sum()
    }
}

impl FromStr for OffDocument {
    type Err = DocumentError;

    fn from_str(string: &str) -> DocumentResult {
        Self::parse(string, Default::default())
    }
}

impl Default for OffDocument {
    fn default() -> Self {
        Self::new()
    }
}
