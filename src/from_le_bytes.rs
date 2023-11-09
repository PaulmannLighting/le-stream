mod impls;
use crate::Result;

pub trait FromLeBytes: Sized {
    /// Parse an object from a stream of bytes with little endianness.
    ///
    /// # Errors
    /// Returns an [`Error`] if the stream terminates prematurely.
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>;
}
