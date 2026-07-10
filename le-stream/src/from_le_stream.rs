use crate::{Error, Result};

mod alloc;
mod bytes;
mod core;
mod heapless;
mod intx;
mod macaddr;

/// Parses a value from a stream of little-endian bytes.
///
/// Implementations read exactly the bytes required to construct `Self` and
/// leave any remaining bytes in the input iterator. Use
/// [`from_le_stream_exact`](Self::from_le_stream_exact) when trailing bytes
/// should be treated as an error.
pub trait FromLeStream: Sized {
    /// Parses a value from the front of a little-endian byte stream.
    ///
    /// The method returns [`None`] when the stream ends before a complete value
    /// can be decoded. Implementations may leave unread bytes in `bytes` after
    /// successfully decoding one value.
    ///
    /// # Examples
    ///
    /// ```
    /// use le_stream::FromLeStream;
    ///
    /// let mut bytes = [0x34, 0x12, 0xFF].into_iter();
    ///
    /// assert_eq!(u16::from_le_stream(&mut bytes), Some(0x1234));
    /// assert_eq!(bytes.next(), Some(0xFF));
    /// ```
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>;

    /// Parses a value from a little-endian byte stream and requires exhaustion.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnexpectedEndOfStream`] if the stream ends before a
    /// complete value can be decoded, or [`Error::StreamNotExhausted`] if a
    /// value was decoded but more bytes remain.
    fn from_le_stream_exact<T>(mut bytes: T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let instance = Self::from_le_stream(&mut bytes).ok_or(Error::UnexpectedEndOfStream)?;

        if let Some(next_byte) = bytes.next() {
            Err(Error::StreamNotExhausted {
                instance,
                next_byte,
            })
        } else {
            Ok(instance)
        }
    }

    /// Parses a value from a little-endian byte slice and requires exhaustion.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the slice is too short or contains bytes after
    /// the decoded value.
    fn from_le_slice(bytes: &[u8]) -> Result<Self> {
        Self::from_le_stream_exact(bytes.iter().copied())
    }
}
