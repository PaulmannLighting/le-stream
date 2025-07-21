use crate::{Error, Result};

mod core;
mod heapless;
mod intx;
mod macaddr;
mod std;

/// Parse an object from a stream of bytes with little endianness.
pub trait FromLeStream: Sized {
    /// Parse an object from a stream of bytes with little endianness.
    ///
    /// # Errors
    /// Returns [`None`] if the stream terminates prematurely.
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>;

    /// Parse an object from a stream of bytes with little endianness
    /// that contains exactly the bytes to construct `Self`.
    ///
    /// # Errors
    /// Returns an [`Error`] if the stream terminates prematurely
    /// or is not exhausted after deserializing `Self`.
    fn from_le_stream_exact<T>(mut bytes: T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let instance = Self::from_le_stream(&mut bytes).ok_or(Error::UnexpectedEndOfStream)?;
        bytes.next().map_or_else(
            || Ok(instance),
            |next_byte| Err(Error::StreamNotExhausted(next_byte)),
        )
    }

    /// Parse an object from a slice of bytes with little endianness
    /// that contains exactly the bytes to construct `Self`.
    ///
    /// # Errors
    /// Returns an [`Error`] if the buffer is too small or contains excess data.
    fn from_le_slice(bytes: &[u8]) -> Result<Self> {
        Self::from_le_stream_exact(bytes.iter().copied())
    }
}
