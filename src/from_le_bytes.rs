mod impls;

use crate::{Error, Result};

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

        if bytes.peekable().peek().is_some() {
            Err(Error::StreamNotExhausted)
        } else {
            Ok(instance)
        }
    }
}
