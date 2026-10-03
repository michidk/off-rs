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
pub mod writer;

use crate::geometry::mesh::Mesh;
use crate::parser::Parser;
use crate::parser::options::Options;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

/// Contains errors that occur during parsing.
#[derive(Debug)]
pub enum Error {
    /// An IO error occurred while reading the file.
    IOError(io::Error),
    /// An error occurred during parsing the `off` data.
    ParserError(crate::parser::error::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::ParserError(e) => Some(e),
        }
    }
}

/// The details are available through [`source`](std::error::Error::source).
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(_) => write!(f, "Failed to read the off file"),
            Error::ParserError(_) => write!(f, "Failed to parse the off data"),
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
    }

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

/// Writes a [`crate::geometry::mesh::Mesh`] in the `off` format to the given [`std::io::Write`]r.
///
/// # Examples
///
/// ```rust
/// use off_rs::geometry::{mesh::{Face, Mesh, Vertex}, position::Position};
///
/// let mesh = Mesh {
///     vertices: vec![
///         Vertex::new(Position::new(1.0, 0.0, 0.0), None),
///         Vertex::new(Position::new(0.0, 1.0, 0.0), None),
///         Vertex::new(Position::new(0.0, 0.0, 1.0), None),
///     ],
///     faces: vec![Face::new(vec![0, 1, 2], None)],
/// };
///
/// let mut buffer = Vec::new();
/// off_rs::write(&mesh, &mut buffer, Default::default()).unwrap();
///
/// assert_eq!(String::from_utf8(buffer).unwrap(), "OFF\n3 1 0\n1 0 0\n0 1 0\n0 0 1\n3 0 1 2\n");
/// ```
///
/// # Errors
///
/// Will return [`writer::error::Error`] if the mesh can not be represented in the `off` format
/// or if an error occurs while writing.
pub fn write<W: Write>(
    mesh: &Mesh,
    writer: W,
    options: writer::options::Options,
) -> writer::Result {
    writer::write(mesh, writer, options)
}

/// Writes a [`crate::geometry::mesh::Mesh`] to an `off` string.
///
/// # Errors
///
/// Will return [`writer::error::Error`] if the mesh can not be represented in the `off` format.
pub fn to_off_string(mesh: &Mesh, options: writer::options::Options) -> writer::Result<String> {
    let mut buffer = Vec::new();
    writer::write(mesh, &mut buffer, options)?;

    // everything written is ASCII, so nothing is replaced
    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

/// Writes a [`crate::geometry::mesh::Mesh`] to the file at the given [`std::path::Path`], replacing its content.
///
/// # Errors
///
/// Will return [`writer::error::Error`] if the file can not be created, the mesh can not be
/// represented in the `off` format or if an error occurs while writing.
pub fn to_path<P: AsRef<Path>>(
    mesh: &Mesh,
    path: P,
    options: writer::options::Options,
) -> writer::Result {
    let file = File::create(path)?;
    writer::write(mesh, BufWriter::new(file), options)
}
