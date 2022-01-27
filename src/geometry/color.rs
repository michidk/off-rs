use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    FromF32(String),
    ToU8(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromF32(msg) => write!(f, "Failed to convert `f32` to `Color`: {}", msg),
            Self::ToU8(msg) => write!(f, "Failed to convert `Color` to `Vec<u8>`: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Result<Self, Error> {
        if !(0.0..=1.0).contains(&r)
            || !(0.0..=1.0).contains(&g)
            || !(0.0..=1.0).contains(&b)
            || !(0.0..=1.0).contains(&a)
        {
            Err(Error::FromF32(format!(
                "Color values must be between 0.0 and 1.0, got: ({}, {}, {}, {})",
                r, g, b, a
            )))
        } else {
            Ok(Self { r, g, b, a })
        }
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
impl TryFrom<Color> for Vec<u8> {
    type Error = Error;

    fn try_from(value: Color) -> Result<Vec<u8>, Error> {
        if value.r > 1.0
            || value.g > 1.0
            || value.b > 1.0
            || value.a > 1.0
            || value.r < 0.0
            || value.g < 0.0
            || value.b < 0.0
            || value.a < 0.0
        {
            return Err(Error::ToU8(format!(
                "Color values must be between 0.0 and 1.0, got: {:?}",
                value
            )));
        }

        Ok(vec![
            (value.r * 255.0).round() as u8,
            (value.g * 255.0).round() as u8,
            (value.b * 255.0).round() as u8,
            (value.a * 255.0).round() as u8,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn color() {
        let color = Color::new(0.1, 0.2, 0.3, 0.4).unwrap();
        assert_eq!(color.r, 0.1);
        assert_eq!(color.g, 0.2);
        assert_eq!(color.b, 0.3);
        assert_eq!(color.a, 0.4);
    }

    #[test]
    fn color_fail() {
        let color = Color::new(1.0, 2.0, 3.0, 4.0);
        assert!(matches!(color, Err(Error::FromF32(_))));
    }

    #[test]
    fn color_from() {
        let color = Color::new(0.1, 0.2, 0.3, 0.4).unwrap();
        assert_eq!(Vec::<f32>::from(color), vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn color_from_u8() {
        let color = Color::new(0.5, 0.7, 0.0, 0.33331).unwrap();
        assert_eq!(Vec::<u8>::try_from(color), Ok(vec![128, 179, 0, 85]));
    }

    #[test]
    fn color_from_u8_fail() {
        let color = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
            a: 4.0,
        };
        assert!(matches!(Vec::<u8>::try_from(color), Err(Error::ToU8(_))));
    }
}
