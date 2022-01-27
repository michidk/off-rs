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
}
