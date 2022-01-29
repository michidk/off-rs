use super::color_format::ColorFormat;

/// Defines the options for the [`Parser`](`crate::parser::Parser`).
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Options {
    /// The color format that is parsed from the `off` string.
    pub color_format: ColorFormat,
    /// The limits that are checked while parsing the `off` string.
    pub limits: Limits,
}

/// Defines limits for the [`Parser`](`crate::parser::Parser`).
///
/// # Note
///
/// When these limits are exceeded during the [`parse`](`crate::parser::Parser::<'_>::parse`)
/// processes an error will be returned.
///
/// Use the [`Default`](`Limits::default`) implementation for reasonable values.
///
/// # Examples
///
/// ```rust
/// use off_rs::parser::options::Limits;
/// let limits = Limits::default();
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Limits {
    /// Defines the maximum amount of vertices the parser accepts.
    pub vertex_count: usize,

    /// Defines the maximum amount of faces the parser accepts.
    pub face_count: usize,

    /// Defines the maximum amount of vertices per face the parser accepts.
    pub face_vertex_count: usize,
}

impl Default for Limits {
    /// Creates a new [`Limits`] with reasonable values.
    fn default() -> Self {
        Self {
            vertex_count: 10000,
            face_count: 1000,
            face_vertex_count: 64,
        }
    }
}

impl Limits {
    /// Limits instance with all values set to their respective maximum value.
    pub const MAX: Self = Self {
        vertex_count: usize::MAX,
        face_count: usize::MAX,
        face_vertex_count: usize::MAX,
    };

    /// Limits instance with all values set to their respective minimum value.
    pub const MIN: Self = Self {
        vertex_count: usize::MIN,
        face_count: usize::MIN,
        face_vertex_count: usize::MIN,
    };
}
