#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod geometry;
pub mod parser;

use crate::geometry::mesh::Mesh;
use crate::parser::options::Options;
use crate::parser::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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

pub trait FromPath {
    fn from_path<P: AsRef<Path>>(path: P, options: Options) -> Result;
}

impl FromPath for Mesh {
    fn from_path<P: AsRef<Path>>(path: P, options: Options) -> Result {
        let mut file = File::open(path).map_err(Error::IOError)?;

        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Ok(_) => {}
            Err(inner) => return Err(Error::IOError(inner)),
        };

        Self::parse(&string, options)
    }
}

pub trait Parse {
    fn parse(string: &str, options: Options) -> Result;
}

impl Parse for Mesh {
    fn parse(string: &str, options: Options) -> Result {
        Parser::new(&string, options).parse()
    }
}
