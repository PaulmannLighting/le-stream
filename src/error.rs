use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    StreamNotExhausted,
    UnexpectedEndOfStream,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StreamNotExhausted => write!(f, "byte stream not exhausted"),
            Self::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl std::error::Error for Error {}
