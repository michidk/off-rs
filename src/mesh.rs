use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use crate::geometry::{ColorFormat, Face, Vertex};
use crate::parser::Parser;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParserError(crate::parser::error::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IO Error: {}", e),
            Error::ParserError(e) => write!(f, "Parser Error: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<crate::parser::error::Error> for Error {
    fn from(e: crate::parser::error::Error) -> Self {
        Error::ParserError(e)
    }
}

pub type Result<D = Mesh> = std::result::Result<D, Error>;

/// Defines limits for the [`Parser`](`crate::parser::Parser`).
///
/// # Note
///
/// When these limits are exceeded during the [`parse`](`crate::parser::Parser::parse`)
/// processes an error will be returned.
///
/// Use the [`Default`](`Limits::default`) implementation for reasonable values.
///
/// # Examples
///
/// ```rust
/// # use off_rs::mesh::Limits;
/// let limits = Limits::default();
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Limits {
    /// Defines the maximum amount of vertices the parser accepts.
    pub vertex_count: usize,

    /// Defines the maximum amount of faces the parser accepts.
    pub face_count: usize,

    /// Defines the maximum amount of vertices per face the parser accepts.
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

impl Limits {
    /// Limits instance with all values set to their respective maximum value.
    pub const MAX: Self = Self {
        vertex_count: usize::MAX,
        face_count: usize::MAX,
        face_vertex_count: usize::MAX,
    };

    /// Limits instance with all values set to their respective minimum value.
    pub const MIN: Self = Self {
        vertex_count: usize::MIN,
        face_count: usize::MIN,
        face_vertex_count: usize::MIN,
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ParserOptions {
    pub color_format: ColorFormat,
    pub limits: Limits,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl Mesh {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn from_path(path: &Path, options: ParserOptions) -> Result {
        let mut file = File::open(path).map_err(Error::IOError)?;

        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Ok(_) => {}
            Err(inner) => return Err(Error::IOError(inner)),
        };

        Self::parse(&string, options)
    }

    pub fn parse(string: &str, options: ParserOptions) -> Result {
        Parser::new(&string, options).parse()
    }

    #[must_use]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    #[must_use]
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.faces.iter().map(|face| face.vertices.len() - 1).sum()
    }
}

impl FromStr for Mesh {
    type Err = Error;

    fn from_str(string: &str) -> Result {
        Self::parse(string, ParserOptions::default())
    }
}
