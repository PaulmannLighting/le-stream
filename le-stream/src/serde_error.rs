use core::fmt::{Display, Formatter};
use core::result::Result;

use serde::{de, ser};

/// Error type for Serde-based byte stream operations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SerdeError {
    /// The serializer ran out of output space.
    BufferTooSmall,
    /// A custom Serde error occurred.
    Custom,
    /// The deserializer encountered an invalid `char` value.
    InvalidChar,
    /// The byte stream was not exhausted.
    StreamNotExhausted {
        /// The next byte in the stream.
        next_byte: u8,
    },
    /// The requested Serde data model shape is unsupported by this format.
    Unsupported,
    /// The byte stream ended prematurely.
    UnexpectedEndOfStream,
}

/// Result type with [`SerdeError`] as error variant.
pub type SerdeResult<T> = Result<T, SerdeError>;

impl Display for SerdeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooSmall => write!(f, "output buffer too small"),
            Self::Custom => write!(f, "custom serde error"),
            Self::InvalidChar => write!(f, "invalid char value"),
            Self::StreamNotExhausted { next_byte } => {
                write!(f, "byte stream not exhausted: [{next_byte:#04X?}, ..]")
            }
            Self::Unsupported => write!(f, "unsupported serde shape"),
            Self::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl core::error::Error for SerdeError {}

impl ser::Error for SerdeError {
    fn custom<T>(_message: T) -> Self
    where
        T: Display,
    {
        Self::Custom
    }
}

impl de::Error for SerdeError {
    fn custom<T>(_message: T) -> Self
    where
        T: Display,
    {
        Self::Custom
    }
}
