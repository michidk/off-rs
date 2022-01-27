use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    FromF32(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromF32(msg) => write!(f, "Failed to convert `f32` to `Position`: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

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

impl TryFrom<Vec<f32>> for Position {
    type Error = Error;

    fn try_from(value: Vec<f32>) -> std::result::Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(Self::Error::FromF32(format!(
                "Invalid amount of arguments (expected: 3, actual: {})",
                value.len()
            )));
        }

        Ok(Self {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn position() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(pos.x, 1.0,);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn position_from() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(Vec::from(pos), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn try_from_positiom() {
        let vec = vec![1.0, 2.0, 3.0];
        let position = Position::try_from(vec);
        assert!(position.is_ok());
        assert_eq!(position.unwrap(), Position::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn try_from_positiom_too_little_arguments() {
        let vec = vec![1.0, 2.0];
        let position = Position::try_from(vec);
        assert!(position.is_err());
        assert!(matches!(position.unwrap_err(), Error::FromF32(_)));
    }

    #[test]
    fn try_from_positiom_too_many_arguments() {
        let vec = vec![1.0, 2.0, 3.0, 4.0];
        let position = Position::try_from(vec);
        assert!(position.is_err());
        assert!(matches!(position.unwrap_err(), Error::FromF32(_)));
    }
}
