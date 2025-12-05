use core::fmt::{Debug, Display, Formatter};

/// Result type with [`Error`] as error variant.
pub type Result<T> = core::result::Result<T, Error<T>>;

/// Error type for byte stream operations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error<T> {
    /// The byte stream was not exhausted.
    StreamNotExhausted {
        /// The instance that was being parsed.
        instance: T,
        /// The next byte in the stream.
        next_byte: u8,
    },
    /// The byte stream ended prematurely.
    UnexpectedEndOfStream,
}

impl<T> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StreamNotExhausted { next_byte, .. } => {
                write!(f, "byte stream not exhausted: [{next_byte:#04X?}, ..]")
            }
            Self::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl<T> core::error::Error for Error<T> where T: Debug {}
