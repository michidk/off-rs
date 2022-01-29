/// The different color formats that can be parsed.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColorFormat {
    /// Parses the red, green and blue values as floating point values ranging from (0.0, 0.0, 0.0) to (1.0, 1.0, 1.0)
    RGBFloat,
    /// Parses the red, green, blue and alpha values as floating point values ranging from (0.0, 0.0, 0.0, 0.0) to (1.0, 1.0, 1.0, 1.0)
    RGBAFloat,
    /// Parses the red, green and blue values as integers ranging from (0, 0, 0) to (255, 255, 255)
    RGBInteger,
    /// Parses the red, green, blue and alpha values as integers ranging from (0, 0, 0, 0) to (255, 255, 255, 255)
    RGBAInteger,
}

impl ColorFormat {
    /// Returns whether the color format is a floating point format.
    #[must_use]
    pub fn is_float(&self) -> bool {
        matches!(self, ColorFormat::RGBFloat | ColorFormat::RGBAFloat)
    }

    /// Returns whether the color format is an integer format.
    #[must_use]
    pub fn is_integer(&self) -> bool {
        !self.is_float()
    }

    /// Returns whether the color format contains an alpha channel.
    #[must_use]
    pub fn has_alpha(&self) -> bool {
        matches!(self, ColorFormat::RGBAFloat | ColorFormat::RGBAInteger)
    }

    /// Returns the number of channels in the color format.
    #[must_use]
    pub fn channel_count(&self) -> usize {
        if self.has_alpha() {
            4
        } else {
            3
        }
    }
}

impl Default for ColorFormat {
    /// The default color format is [`RGBFloat`].
    // Because this format is specified in the implementation of the Princeton Shape Benchmark.
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

        assert_eq!(ColorFormat::RGBFloat.channel_count(), 3);
        assert_eq!(ColorFormat::RGBAFloat.channel_count(), 4);
        assert_eq!(ColorFormat::RGBInteger.channel_count(), 3);
        assert_eq!(ColorFormat::RGBAInteger.channel_count(), 4);
    }
}
