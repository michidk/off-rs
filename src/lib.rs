#![warn(clippy::pedantic)]

//! A simple `.off` file parser.
//!
//! # Usage
//!
//! ```rust
//!    let off_string = r#"
//!OFF
//!3 1
//!1.0 0.0 0.0
//!0.0 1.0 0.0
//!0.0 0.0 1.0
//!4  0 1 2 3  255 0 0 # red
//!"#;
//!
//!let mesh = off_rs::parse(
//!    off_string,
//!    Default::default() // optional ParserOptions
//!);
//! ```

pub mod geometry;
pub mod parser;

use crate::geometry::mesh::Mesh;
use crate::parser::options::Options;
use crate::parser::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Contains errors that occur during parsing.
#[derive(Debug)]
pub enum Error {
    /// An IO error occurred while reading the file.
    IOError(io::Error),
    /// An error occurred during parsing the `off` data.
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

/// This result may contain the parsed [`crate::geometry::mesh::Mesh`] or the [`self::Result`] that occurred.
pub type Result<D = Mesh> = std::result::Result<D, Error>;

/// Parse a [`crate::geometry::mesh::Mesh`] from a [`std::path::Path`] pointing to an `.off` file.
///
/// # Errors
///
/// Will return `self::Error` if an error occurs while reading the file or parsing the `off` data.
pub fn from_path<P: AsRef<Path>>(path: P, options: Options) -> Result {
    let mut file = File::open(path).map_err(Error::IOError)?;

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => {}
        Err(inner) => return Err(Error::IOError(inner)),
    };

    parse(&string, options)
}

/// Directly parse a [`crate::geometry::mesh::Mesh`] from an `off` string.
///
/// # Examples
///
/// ```rust
///     let off_string = r#"
///OFF
///3 1
///1.0 0.0 0.0
///0.0 1.0 0.0
///0.0 0.0 1.0
///4  0 1 2 3  1.0 0.0 0.0 1.0 # red
///"#;
///
///    let mesh = off_rs::parse(
///        off_string,
///        Default::default(), // optional ParserOptions
///    );
///
///    println!("{:#?}", mesh);
/// ```
///
/// # Errors
///
/// Will return `self::Error` if an error occurs while parsing the `off` data.
pub fn parse(string: &str, options: Options) -> Result {
    Parser::new(&string, options).parse()
}
