use crate::{Error, Result};

mod impls;

/// Parse an object from a stream of bytes with little endianness.
pub trait FromLeBytes: Sized {
    /// Parse an object from a stream of bytes with little endianness.
    ///
    /// # Errors
    /// Returns an [`Error`] if the stream terminates prematurely.
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>;

    /// Parse an object from a stream of bytes with little endianness
    /// that contains exactly the bytes to construct `Self`.
    ///
    /// # Errors
    /// Returns an [`Error`] if the stream terminates prematurely
    /// or is not exhausted after deserializing `Self`.
    fn from_le_bytes_exact<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let instance = Self::from_le_bytes(bytes)?;
        bytes.next().map_or_else(
            || Ok(instance),
            |next_byte| Err(Error::StreamNotExhausted(next_byte)),
        )
    }
}
