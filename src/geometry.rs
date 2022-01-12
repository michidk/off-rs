use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeometryError {
    #[error(
        "A vertex needs three integers to describe the x, y and z component of it's position."
    )]
    VertexOutOfBounds,
    #[error(
        "A face needs at least three integers that reference vertices indices to describe the polygon."
    )]
    FaceOutOfBounds,
    #[error(
        "A color needs three (RGB) to four (RGBA) values that describe the color (either between 0-1 or 0-255)."
    )]
    ColorOutOfBounds,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<Position> for Vec<f32> {
    fn from(value: Position) -> Vec<f32> {
        vec![value.x, value.y, value.z]
    }
}

// TODO: move to parser module
impl TryFrom<Vec<f32>> for Position {
    type Error = GeometryError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(GeometryError::VertexOutOfBounds);
        }

        Ok(Self {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColorFormat {
    RGBFloat,    // (0.0, 0.0, 0.0) to (1.0, 1.0, 1.0)
    RGBAFloat,   // (0.0, 0.0, 0.0, 0.0) to (1.0, 1.0, 1.0, 1.0)
    RGBInteger,  // (0, 0, 0) to (255, 255, 255)
    RGBAInteger, // (0, 0, 0, 0) to (255, 255, 255, 255)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
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

impl From<Color> for Vec<f32> {
    fn from(value: Color) -> Vec<f32> {
        vec![value.r, value.g, value.b, value.a]
    }
}

// TODO: move to parser module
impl TryFrom<Vec<f32>> for Color {
    type Error = GeometryError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(GeometryError::ColorOutOfBounds);
        }

        let alpha = if value.len() == 4 { value[3] } else { 1.0 };

        Ok(Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: alpha,
        })
    }
}

impl From<Color> for Vec<u8> {
    fn from(value: Color) -> Vec<u8> {
        vec![
            (value.r * 255.0) as u8,
            (value.g * 255.0) as u8,
            (value.b * 255.0) as u8,
            (value.a * 255.0) as u8,
        ]
    }
}

// TODO: move to parser module
impl TryFrom<Vec<u8>> for Color {
    type Error = GeometryError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(GeometryError::ColorOutOfBounds);
        }

        let alpha = if value.len() == 4 {
            value[3] as f32 / 255.0
        } else {
            1.0
        };

        Ok(Self {
            r: value[0] as f32 / 255.0,
            g: value[1] as f32 / 255.0,
            b: value[2] as f32 / 255.0,
            a: alpha,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vertex {
    pub position: Position,
    pub color: Option<Color>,
}

impl Vertex {
    pub fn new(position: Position, color: Option<Color>) -> Self {
        Self { position, color }
    }
}

// TODO: move to parser module
impl TryFrom<Vec<f32>> for Vertex {
    type Error = GeometryError;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(GeometryError::VertexOutOfBounds);
        }
        if value.len() < 6 {
            Ok(Self::new(value.try_into()?, None))
        } else {
            let pos = value[1..=3].to_vec().try_into()?;

            // check the color arguments
            if value[4..].iter().any(|x| *x >= 0.0 || *x <= 1.0) {
                // values 0.0 to 1.0
                Ok(Self::new(pos, Some(value[4..].to_vec().try_into()?)))
            } else if value[4..].iter().any(|x| *x >= 0.0 || *x <= 255.0) {
                // values ranging from 0 to 255
                let color: Vec<u8> = value[4..].iter().map(|x| *x as u8).collect();
                Ok(Self::new(pos, Some(color.try_into()?)))
            } else {
                Err(GeometryError::ColorOutOfBounds)
            }
        }
    }
}

impl From<Vertex> for Vec<f32> {
    fn from(value: Vertex) -> Vec<f32> {
        if let Some(color) = value.color {
            vec![
                value.position.x,
                value.position.y,
                value.position.z,
                color.r,
                color.g,
                color.b,
                color.a,
            ]
        } else {
            vec![value.position.x, value.position.y, value.position.z]
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Face {
    pub vertices: Vec<usize>,
    pub color: Option<Color>,
}

impl Face {
    pub fn new(vertices: Vec<usize>, color: Option<Color>) -> Self {
        Self { vertices, color }
    }
}

// TODO: move to parser module
impl TryFrom<Vec<usize>> for Face {
    type Error = GeometryError;

    fn try_from(value: Vec<usize>) -> Result<Self, Self::Error> {
        // let vertex_count: u32 = face_str[0].parse().map_err(|_| ParserError::InvalidFace)?;
        // face_str = face_str.into_iter().skip(1).collect();

        if value.len() < 3 {
            return Err(GeometryError::FaceOutOfBounds);
        }
        Ok(Self::new(value, None))
    }
}

impl From<Face> for Vec<usize> {
    fn from(value: Face) -> Vec<usize> {
        value.vertices
    }
}
