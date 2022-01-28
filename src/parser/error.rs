use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    pub kind: Kind,
    pub line_index: usize,
    pub message: Option<Cow<'static, str>>,
}

impl Error {
    #[must_use]
    pub(crate) fn new(kind: Kind, line_index: usize, message: Option<Cow<'static, str>>) -> Self {
        Self {
            kind,
            line_index,
            message,
        }
    }

    pub(crate) fn with_message<M: Into<Cow<'static, str>>, O: Into<Option<M>>>(
        kind: Kind,
        line_index: usize,
        message: O,
    ) -> Self {
        Self::new(kind, line_index, message.into().map(Into::into))
    }

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
    Empty,
    Missing,
    LimitExceeded,
    Invalid,
    InvalidHeader,
    InvalidCounts,
    InvalidVertexPosition,
    InvalidColor,
    InvalidFace,
    InvalidFaceIndex,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Debug::fmt(self, f)
    }
}
