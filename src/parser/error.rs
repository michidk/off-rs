use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

/// A plain message used as the [`source`](StdError::source) of an [`Error`] that has no underlying error.
#[derive(Debug)]
struct Message(Cow<'static, str>);

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl StdError for Message {}

/// An error that occured while parsing the `off` string line by line.
///
/// The details of what went wrong are available through [`source`](StdError::source).
/// They are deliberately not part of the [`Display`] output, so that error reporters
/// walking the chain do not print them twice.
///
/// Two errors are equal if they have the same [`Kind`], line and rendered source.
#[derive(Debug, Clone)]
pub struct Error {
    /// The [`Kind`] of the error.
    pub kind: Kind,
    /// The line number in the `off` string where the error occured.
    pub line_index: usize,
    /// The underlying error or message describing the problem.
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
}

impl Error {
    /// Creates a new [`Error`] with the given [`Kind`] and line number and a string as message.
    #[must_use]
    pub(crate) fn with_message<M: Into<Cow<'static, str>>>(
        kind: Kind,
        line_index: usize,
        message: M,
    ) -> Self {
        Self::with_source(kind, line_index, Message(message.into()))
    }

    /// Creates a new [`Error`] with the given [`Kind`] and line number caused by the given error.
    #[must_use]
    pub(crate) fn with_source<E: StdError + Send + Sync + 'static>(
        kind: Kind,
        line_index: usize,
        source: E,
    ) -> Self {
        Self {
            kind,
            line_index,
            source: Some(Arc::new(source)),
        }
    }

    /// Creates a new [`Error`] with the given [`Kind`] and line number.
    #[must_use]
    pub(crate) fn without_message(kind: Kind, line_index: usize) -> Self {
        Self {
            kind,
            line_index,
            source: None,
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|e| e as &(dyn StdError + 'static))
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.line_index == other.line_index
            && self.source.as_ref().map(ToString::to_string)
                == other.source.as_ref().map(ToString::to_string)
    }
}

impl Eq for Error {}

impl Hash for Error {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.line_index.hash(state);
        self.source.as_ref().map(ToString::to_string).hash(state);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} @ ln:{}", self.kind, self.line_index + 1)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn source_returns_message() {
        let error = Error::with_message(Kind::InvalidHeader, 0, "something went wrong");
        assert_eq!(
            error.source().map(ToString::to_string),
            Some("something went wrong".to_string())
        );
    }

    #[test]
    fn source_returns_underlying_error() {
        let cause = "x".parse::<u8>().unwrap_err();
        let error = Error::with_source(Kind::InvalidCounts, 2, cause.clone());
        assert_eq!(
            error.source().map(ToString::to_string),
            Some(cause.to_string())
        );
    }

    #[test]
    fn source_without_message() {
        assert!(Error::without_message(Kind::Empty, 0).source().is_none());
    }

    #[test]
    fn display_excludes_source() {
        let error = Error::with_message(Kind::InvalidHeader, 4, "details");
        assert_eq!(error.to_string(), "InvalidHeader @ ln:5");
    }

    #[test]
    fn equality_and_hash_consider_source() {
        let a = Error::with_message(Kind::Missing, 1, "a");
        let same = Error::with_message(Kind::Missing, 1, String::from("a"));
        let other = Error::with_message(Kind::Missing, 1, "b");

        assert_eq!(a, same);
        assert_ne!(a, other);
        assert_ne!(a, Error::without_message(Kind::Missing, 1));
        assert_eq!(a.clone(), a);

        let set: HashSet<_> = [a, same, other].into_iter().collect();
        assert_eq!(set.len(), 2);
    }
}
