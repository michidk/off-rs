use super::{color::Color, position::Position};

/// Represents a vertex of a mesh.
/// A vertex contains a position and optionally a vertex color.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: Position,
    /// The color of the vertex.
    pub color: Option<Color>,
}

impl Vertex {
    /// Creates a new [`Vertex`].
    #[must_use]
    pub fn new(position: Position, color: Option<Color>) -> Self {
        Self { position, color }
    }
}

/// Represents a face of a mesh.
/// A face contains a list of vertex indicies and optionally a color.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Face {
    /// The list of vertex indicies that make up the face.
    pub vertices: Vec<usize>,
    /// The color of the face.
    pub color: Option<Color>,
}

impl Face {
    /// Creates a new [`Face`].
    #[must_use]
    pub fn new(vertices: Vec<usize>, color: Option<Color>) -> Self {
        Self { vertices, color }
    }
}

impl From<Face> for Vec<usize> {
    /// Converts a [`Face`] to a [`Vec<usize>`] containing the vertex indicies.
    fn from(value: Face) -> Vec<usize> {
        value.vertices
    }
}

/// Represents a mesh.
/// A mesh contains a list of vertices and a list of faces.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Mesh {
    /// The list of vertices.
    pub vertices: Vec<Vertex>,
    /// The list of faces.
    pub faces: Vec<Face>,
}

impl Mesh {
    /// Creates a new [`Mesh`].
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Returns the number of vertices in the mesh.
    #[must_use]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of faces in the mesh.
    #[must_use]
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Calculates the number of edges in the mesh.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.faces.iter().map(|face| face.vertices.len() - 1).sum()
    }
}
