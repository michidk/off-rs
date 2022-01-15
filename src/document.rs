use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use crate::geometry::{ColorFormat, Face, Vertex};
use crate::parser::error::ParserError;
use crate::parser::DocumentParser;

#[derive(Debug)]
pub enum DocumentError {
    IOError(io::Error),
    ParserError(ParserError),
}

impl std::error::Error for DocumentError {}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Limits {
    pub vertex_count: usize,
    pub face_count: usize,
    pub face_vertex_count: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            vertex_count: 2048,
            face_count: 4096,
            face_vertex_count: 64,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ParserOptions {
    pub color_format: ColorFormat,
    pub limits: Limits,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct OffDocument {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl OffDocument {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub fn from_path(path: &Path, options: ParserOptions) -> DocumentResult {
        let mut file = File::open(path).map_err(DocumentError::IOError)?;

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
