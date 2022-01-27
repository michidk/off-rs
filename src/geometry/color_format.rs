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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_format() {
        assert!(ColorFormat::RGBFloat.is_float());
        assert!(ColorFormat::RGBAFloat.is_float());
        assert!(!ColorFormat::RGBInteger.is_float());
        assert!(!ColorFormat::RGBAInteger.is_float());

        assert!(!ColorFormat::RGBFloat.is_integer());
        assert!(!ColorFormat::RGBAFloat.is_integer());
        assert!(ColorFormat::RGBInteger.is_integer());
        assert!(ColorFormat::RGBAInteger.is_integer());

        assert!(!ColorFormat::RGBFloat.has_alpha());
        assert!(ColorFormat::RGBAFloat.has_alpha());
        assert!(!ColorFormat::RGBInteger.has_alpha());
        assert!(ColorFormat::RGBAInteger.has_alpha());

        assert_eq!(ColorFormat::RGBFloat.element_count(), 3);
        assert_eq!(ColorFormat::RGBAFloat.element_count(), 4);
        assert_eq!(ColorFormat::RGBInteger.element_count(), 3);
        assert_eq!(ColorFormat::RGBAInteger.element_count(), 4);
    }
}
