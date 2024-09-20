use crate::{Error, Result};

mod impls;

/// Parse an object from a stream of bytes with little endianness.
pub trait FromLeStream: Sized {
    /// Parse an object from a stream of bytes with little endianness.
    ///
    /// # Errors
    /// Returns [`None`] if the stream terminates prematurely.
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>;

    /// Parse an object from a stream of bytes with little endianness
    /// that contains exactly the bytes to construct `Self`.
    ///
    /// # Errors
    /// Returns an [`Error`] if the stream terminates prematurely
    /// or is not exhausted after deserializing `Self`.
    fn from_le_stream_exact<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let instance = Self::from_le_stream(bytes).ok_or(Error::UnexpectedEndOfStream)?;
        bytes.next().map_or_else(
            || Ok(instance),
            |next_byte| Err(Error::StreamNotExhausted(next_byte)),
        )
    }
}
