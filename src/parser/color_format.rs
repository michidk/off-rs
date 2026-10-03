use crate::geometry::color::{Color, Error};

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
        if self.has_alpha() { 4 } else { 3 }
    }

    /// Parses a [`Color`] from the given channel strings according to this format.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ChannelCount`] if the number of `parts` does not match [`ColorFormat::channel_count`],
    /// [`Error::Parse`] if a channel is not a valid number for this format,
    /// or the conversion error of [`Color`] if a channel is out of range.
    pub fn parse(&self, parts: &[&str]) -> Result<Color, Error> {
        if parts.len() != self.channel_count() {
            return Err(Error::ChannelCount(format!(
                "expected {}, actual: {}",
                self.channel_count(),
                parts.len()
            )));
        }

        if self.is_float() {
            let channels = parts
                .iter()
                .map(|s| {
                    s.parse::<f32>()
                        .map_err(|err| Error::Parse(format!("`{s}` is not a float ({err})")))
                })
                .collect::<Result<Vec<f32>, Error>>()?;

            Color::try_from(channels)
        } else {
            let channels = parts
                .iter()
                .map(|s| {
                    s.parse::<u8>()
                        .map_err(|err| Error::Parse(format!("`{s}` is not a u8 ({err})")))
                })
                .collect::<Result<Vec<u8>, Error>>()?;

            Color::try_from(channels)
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

    #[test]
    fn parse_float() {
        assert_eq!(
            ColorFormat::RGBFloat.parse(&["1.0", "0.5", "0.0"]),
            Ok(Color::new(1.0, 0.5, 0.0, 1.0).unwrap())
        );
        assert_eq!(
            ColorFormat::RGBAFloat.parse(&["1.0", "0.5", "0.0", "0.25"]),
            Ok(Color::new(1.0, 0.5, 0.0, 0.25).unwrap())
        );
    }

    #[test]
    fn parse_integer() {
        assert_eq!(
            ColorFormat::RGBInteger.parse(&["255", "0", "255"]),
            Ok(Color::new(1.0, 0.0, 1.0, 1.0).unwrap())
        );
        assert_eq!(
            ColorFormat::RGBAInteger.parse(&["255", "0", "255", "0"]),
            Ok(Color::new(1.0, 0.0, 1.0, 0.0).unwrap())
        );
    }

    #[test]
    fn parse_wrong_channel_count() {
        assert!(matches!(
            ColorFormat::RGBFloat.parse(&["1.0", "0.5"]),
            Err(Error::ChannelCount(_))
        ));
        assert!(matches!(
            ColorFormat::RGBInteger.parse(&["1", "2", "3", "4"]),
            Err(Error::ChannelCount(_))
        ));
    }

    #[test]
    fn parse_invalid_number() {
        assert!(matches!(
            ColorFormat::RGBFloat.parse(&["1.0", "x", "0.0"]),
            Err(Error::Parse(_))
        ));
        // floats are not valid integer channels
        assert!(matches!(
            ColorFormat::RGBInteger.parse(&["255", "128.0", "0"]),
            Err(Error::Parse(_))
        ));
        assert!(matches!(
            ColorFormat::RGBInteger.parse(&["256", "0", "0"]),
            Err(Error::Parse(_))
        ));
    }

    #[test]
    fn parse_out_of_range() {
        assert!(matches!(
            ColorFormat::RGBFloat.parse(&["2.0", "0.0", "0.0"]),
            Err(Error::FromF32(_))
        ));
    }
}
