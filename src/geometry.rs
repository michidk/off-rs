#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<Position> for Vec<f32> {
    fn from(value: Position) -> Vec<f32> {
        vec![value.x, value.y, value.z]
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColorFormat {
    RGBFloat,    // (0.0, 0.0, 0.0) to (1.0, 1.0, 1.0)
    RGBAFloat,   // (0.0, 0.0, 0.0, 0.0) to (1.0, 1.0, 1.0, 1.0)
    RGBInteger,  // (0, 0, 0) to (255, 255, 255)
    RGBAInteger, // (0, 0, 0, 0) to (255, 255, 255, 255)
}

impl ColorFormat {
    #[must_use]
    pub fn is_float(&self) -> bool {
        matches!(self, ColorFormat::RGBFloat | ColorFormat::RGBAFloat)
    }

    #[must_use]
    pub fn is_integer(&self) -> bool {
        !self.is_float()
    }

    #[must_use]
    pub fn has_alpha(&self) -> bool {
        matches!(self, ColorFormat::RGBAFloat | ColorFormat::RGBAInteger)
    }

    #[must_use]
    pub fn element_count(&self) -> usize {
        if self.has_alpha() {
            4
        } else {
            3
        }
    }
}

impl Default for ColorFormat {
    fn default() -> Self {
        ColorFormat::RGBAFloat
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    #[must_use]
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

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
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
