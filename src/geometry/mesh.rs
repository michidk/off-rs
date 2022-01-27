use super::{color::Color, position::Position};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vertex {
    pub position: Position,
    pub color: Option<Color>,
}

impl Vertex {
    #[must_use]
    pub fn new(position: Position, color: Option<Color>) -> Self {
        Self { position, color }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Face {
    pub vertices: Vec<usize>,
    pub color: Option<Color>,
}

impl Face {
    #[must_use]
    pub fn new(vertices: Vec<usize>, color: Option<Color>) -> Self {
        Self { vertices, color }
    }
}

impl From<Face> for Vec<usize> {
    fn from(value: Face) -> Vec<usize> {
        value.vertices
    }
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
