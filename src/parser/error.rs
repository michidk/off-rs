use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub line_index: usize,
    pub message: Option<Cow<'static, str>>,
}

impl ParserError {
    pub fn new(
        kind: ParserErrorKind,
        line_index: usize,
        message: Option<Cow<'static, str>>,
    ) -> Self {
        Self {
            kind,
            line_index,
            message,
        }
    }

    pub fn with_message<M: Into<Cow<'static, str>>, O: Into<Option<M>>>(
        kind: ParserErrorKind,
        line_index: usize,
        message: O,
    ) -> Self {
        Self::new(kind, line_index, message.into().map(|inner| inner.into()))
    }

    pub fn without_message(kind: ParserErrorKind, line_index: usize) -> Self {
        Self::new(kind, line_index, None)
    }
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} in line {}{}",
            self.kind,
            self.line_index + 1,
            self.message
                .as_ref()
                .map(|msg| format!(" - {}", msg))
                .unwrap_or_else(String::new)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserErrorKind {
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

impl std::fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}
