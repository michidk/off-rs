use crate::parser::color_format::ColorFormat;

/// Defines the options for the [`write`](`crate::writer::write`) function.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Options {
    /// The color format that is written to the `off` string.
    ///
    /// Formats without an alpha channel drop the alpha value of the colors.
    /// Integer formats round the colors to the nearest `u8`.
    pub color_format: ColorFormat,
}
