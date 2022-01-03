use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeometryError {
    #[error(
        "A vertex needs three integers to describe the x, y and z component of it's position."
    )]
    VertexOutOfBounds,
    #[error(
        "A face needs at least integers that reference vertices indices to describe the polygon."
    )]
    FaceOutOfBounds,
}

#[derive(Debug, PartialEq, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl From<Position> for Vec<f64> {
    fn from(value: Position) -> Vec<f64> {
        vec![value.x, value.y, value.z]
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl From<Color> for Vec<f64> {
    fn from(value: Color) -> Vec<f64> {
        vec![value.r, value.g, value.b, value.a]
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }
}

impl TryFrom<Vec<f64>> for Vertex {
    type Error = GeometryError;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(GeometryError::VertexOutOfBounds);
        }
        if value.len() < 6 {
            Ok(Self::new(
                Position::new(value[0], value[1], value[2]),
                Color::default(),
            ))
        } else if value.len() < 7 {
            Ok(Self::new(
                Position::new(value[0], value[1], value[2]),
                Color {
                    r: value[3],
                    g: value[4],
                    b: value[5],
                    ..Default::default()
                },
            ))
        } else {
            Ok(Self::new(
                Position::new(value[0], value[1], value[2]),
                Color::new(value[3], value[4], value[5], value[6]),
            ))
        }
    }
}

impl From<Vertex> for Vec<f64> {
    fn from(value: Vertex) -> Vec<f64> {
        vec![
            value.position.x,
            value.position.y,
            value.position.z,
            value.color.r,
            value.color.g,
            value.color.b,
            value.color.a,
        ]
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Face {
    pub vertex_count: u32,
    pub vertices: Vec<u32>,
}

impl Face {
    pub fn new(vertices: Vec<u32>) -> Self {
        Self {
            vertex_count: vertices.len() as u32,
            vertices,
        }
    }
}

impl TryFrom<Vec<u32>> for Face {
    type Error = GeometryError;

    fn try_from(value: Vec<u32>) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(GeometryError::FaceOutOfBounds);
        }
        Ok(Self::new(value))
    }
}

impl From<Face> for Vec<u32> {
    fn from(value: Face) -> Vec<u32> {
        value.vertices
    }
}
