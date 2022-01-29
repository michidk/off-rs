use std::fmt::{Debug, Display, Formatter};

/// Contains errors that occur while converting a color from or to a different format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    FromF32(String),
    FromU8(String),
    ToU8(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromF32(msg) => write!(f, "Failed to convert `f32` to `Color`: {}", msg),
            Self::FromU8(msg) => write!(f, "Failed to convert `u8` to `Color`: {}", msg),
            Self::ToU8(msg) => write!(f, "Failed to convert `Color` to `Vec<u8>`: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// A color stored as four [`f32`] values (red, green, blue, alpha) ranging from 0.0 to 1.0.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    /// Creates a new [`Color`] from the given `red`, `green`, `blue` and `alpha` values and checks for validity.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::FromF32`] of the color values are not between 0.0 and 1.0
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Result<Self, Error> {
        if !(0.0..=1.0).contains(&red)
            || !(0.0..=1.0).contains(&green)
            || !(0.0..=1.0).contains(&blue)
            || !(0.0..=1.0).contains(&alpha)
        {
            Err(Error::FromF32(format!(
                "Color values must be between 0.0 and 1.0, got: ({}, {}, {}, {})",
                red, green, blue, alpha
            )))
        } else {
            Ok(Self {
                red,
                green,
                blue,
                alpha,
            })
        }
    }
}

impl Default for Color {
    /// Returns the color white.
    fn default() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

impl From<Color> for Vec<f32> {
    /// Converts a [`Color`] to a [`Vec`] of four [`f32`] values.
    fn from(value: Color) -> Vec<f32> {
        vec![value.red, value.green, value.blue, value.alpha]
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl TryFrom<Color> for Vec<u8> {
    type Error = Error;

    /// Converts a [`Color`] to a [`Vec<u8>`]
    ///
    /// # Errors
    ///
    /// Returns [`Error::ToU8`] if the elements of [`Color`] are not in the range of 0.0 to 1.0.
    fn try_from(value: Color) -> Result<Vec<u8>, Error> {
        if !(0.0..=1.0).contains(&value.red)
            || !(0.0..=1.0).contains(&value.green)
            || !(0.0..=1.0).contains(&value.blue)
            || !(0.0..=1.0).contains(&value.alpha)
        {
            return Err(Error::ToU8(format!(
                "Color values must be between 0.0 and 1.0, got: {:?}",
                value
            )));
        }

        Ok(vec![
            (value.red * 255.0).round() as u8,
            (value.green * 255.0).round() as u8,
            (value.blue * 255.0).round() as u8,
            (value.alpha * 255.0).round() as u8,
        ])
    }
}

impl TryFrom<Vec<f32>> for Color {
    type Error = Error;

    /// Converts a [`Vec<f32>`] to a [`Color`]
    ///
    /// # Errors
    ///
    /// Returns [`Error::FromF32`] if `value` contains less than three or more than four elements.
    fn try_from(value: Vec<f32>) -> std::result::Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(Self::Error::FromF32(format!(
                "Invalid amount of arguments (expected: 3-4, actual: {})",
                value.len()
            )));
        }

        let alpha = if value.len() == 4 { value[3] } else { 1.0 };

        Color::new(value[0], value[1], value[2], alpha)
    }
}

impl TryFrom<Vec<u8>> for Color {
    type Error = Error;

    /// Converts a [`Vec<u8>`] to a [`Color`]
    ///
    /// # Errors
    ///
    /// Returns [`Error::FromU8`] if `value` contains less than four or more than four elements or if the elements are not in the range of 0 to 255.
    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        if 3 > value.len() || 4 < value.len() {
            return Err(Self::Error::FromU8(format!(
                "Invalid amount of arguments (expected: 3-4, actual: {})",
                value.len()
            )));
        }

        let alpha = if value.len() == 4 { value[3] } else { 255 };
        let val = [value[0], value[1], value[2], alpha];

        if !(0..=255).contains(&val[0])
            || !(0..=255).contains(&val[1])
            || !(0..=255).contains(&val[2])
            || !(0..=255).contains(&val[3])
        {
            return Err(Error::FromU8(format!(
                "Color values must be between 0 and 255, got: {:?}",
                val
            )));
        }

        Color::new(
            f32::from(val[0]) / 255.0,
            f32::from(val[1]) / 255.0,
            f32::from(val[2]) / 255.0,
            f32::from(val[3]) / 255.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn color() {
        let color = Color::new(0.1, 0.2, 0.3, 0.4).unwrap();
        assert_eq!(color.red, 0.1);
        assert_eq!(color.green, 0.2);
        assert_eq!(color.blue, 0.3);
        assert_eq!(color.alpha, 0.4);
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
            red: 1.0,
            green: 2.0,
            blue: 3.0,
            alpha: 4.0,
        };
        assert!(matches!(Vec::<u8>::try_from(color), Err(Error::ToU8(_))));
    }

    #[test]
    fn try_from_color_rgb() {
        let vec = vec![0.1, 0.2, 0.3, 0.4];
        let color = Color::try_from(vec);
        assert!(color.is_ok());
        assert_eq!(color.unwrap(), Color::new(0.1, 0.2, 0.3, 0.4).unwrap());
    }

    #[test]
    fn try_from_color_rgba() {
        let vec = vec![0.1, 0.2, 0.3, 0.4];
        let color = Color::try_from(vec);
        assert!(color.is_ok());
        assert_eq!(color.unwrap(), Color::new(0.1, 0.2, 0.3, 0.4).unwrap());
    }

    #[test]
    fn try_from_color_err_too_little_arguments() {
        let vec = vec![1.0, 2.0];
        let color = Color::try_from(vec);
        assert!(color.is_err());
        assert!(matches!(color.unwrap_err(), Error::FromF32(_)));
    }

    #[test]
    fn try_from_color_err_too_many_arguments() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let color = Color::try_from(vec);
        assert!(color.is_err());
        assert!(matches!(color.unwrap_err(), Error::FromF32(_)));
    }

    #[test]
    fn try_from_color_u8() {
        let vec = vec![128, 255, 0, 255];
        let color = Color::try_from(vec);
        assert!(color.is_ok());
        assert_eq!(
            color.unwrap(),
            Color::new(0.501_960_8, 1.0, 0.0, 1.0).unwrap()
        );
    }
}
