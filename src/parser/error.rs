use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
};

/// An error that occured while parsing the `off` string line by line.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    /// The [`Kind`] of the error.
    pub kind: Kind,
    /// The line number in the `off` string where the error occured.
    pub line_index: usize,
    /// An error message describing the problem.
    pub message: Option<Cow<'static, str>>,
}

impl Error {
    /// Creates a new [`Error`] with the given [`Kind`], line number and optionally a message.
    #[must_use]
    pub(crate) fn new(kind: Kind, line_index: usize, message: Option<Cow<'static, str>>) -> Self {
        Self {
            kind,
            line_index,
            message,
        }
    }

    /// Creates a new [`Error`] with the given [`Kind`] and line numbber and a string as message.
    #[must_use]
    pub(crate) fn with_message<M: Into<Cow<'static, str>>, O: Into<Option<M>>>(
        kind: Kind,
        line_index: usize,
        message: O,
    ) -> Self {
        Self::new(kind, line_index, message.into().map(Into::into))
    }

    /// Creates a new [`Error`] with the given [`Kind`] and line number.
    #[must_use]
    pub(crate) fn without_message(kind: Kind, line_index: usize) -> Self {
        Self::new(kind, line_index, None)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(msg) = &self.message {
            write!(f, "{} @ ln:{} - {}", self.kind, self.line_index + 1, msg)
        } else {
            write!(f, "{} @ ln:{}", self.kind, self.line_index + 1,)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// The `off` string is empty.
    Empty,
    /// An element is missing (e.g. counts, vertices, faces).
    Missing,
    /// A limit was exceeded.
    LimitExceeded,
    /// The header has a invalid format.
    InvalidHeader,
    /// The counts of vertices, faces and edges have an invalid format.
    InvalidCounts,
    /// The vertex position has an invalid format.
    InvalidVertexPosition,
    /// The color has an invalid format.
    InvalidColor,
    /// The face definition has an invalid format.
    InvalidFace,
    /// The face indicies have an invalid format.
    InvalidFaceIndex,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Debug::fmt(self, f)
    }
}
