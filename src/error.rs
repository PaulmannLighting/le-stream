use core::fmt::{Display, Formatter};

/// Result type with [`Error`] as error variant.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for byte stream operations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The byte stream was not exhausted.
    StreamNotExhausted(u8),
    /// The byte stream ended prematurely.
    UnexpectedEndOfStream,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StreamNotExhausted(next_byte) => {
                write!(f, "byte stream not exhausted: [{next_byte:#04X?}, ..]")
            }
            Self::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl core::error::Error for Error {}
