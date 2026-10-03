use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    io,
};

use crate::geometry::color;

/// The element of a mesh an [`Error`] refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    /// The vertex with the given index.
    Vertex(usize),
    /// The face with the given index.
    Face(usize),
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Vertex(index) => write!(f, "vertex {index}"),
            Element::Face(index) => write!(f, "face {index}"),
        }
    }
}

/// An error that occurred while writing a mesh to the `off` format.
///
/// Like the parser error, the underlying cause is available through [`source`](StdError::source).
#[derive(Debug)]
pub enum Error {
    /// An IO error occurred while writing.
    Io(io::Error),
    /// A face references a vertex that does not exist.
    InvalidFaceIndex {
        /// The index of the face.
        face: usize,
        /// The vertex index that is out of bounds.
        index: usize,
        /// The number of vertices of the mesh.
        vertex_count: usize,
    },
    /// A face has less than three vertices.
    InvalidFace {
        /// The index of the face.
        face: usize,
        /// The number of vertices of the face.
        vertex_count: usize,
    },
    /// A color can not be represented in the chosen color format.
    InvalidColor {
        /// The element that has the color.
        element: Element,
        /// The reason the color is invalid.
        source: color::Error,
    },
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(source) => Some(source),
            Error::InvalidColor { source, .. } => Some(source),
            Error::InvalidFaceIndex { .. } | Error::InvalidFace { .. } => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(_) => write!(f, "Failed to write the off data"),
            Error::InvalidFaceIndex {
                face,
                index,
                vertex_count,
            } => write!(
                f,
                "Face {face} references vertex {index}, but the mesh only has {vertex_count} vertices"
            ),
            Error::InvalidFace { face, vertex_count } => write!(
                f,
                "Face {face} has {vertex_count} vertices, but at least 3 are required"
            ),
            Error::InvalidColor { element, .. } => write!(f, "Invalid color of {element}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
