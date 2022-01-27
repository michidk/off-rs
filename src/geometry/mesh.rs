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
