use crate::{FromLeStream, Result};

/// Convenience trait for decoding a value from a byte iterator.
pub trait Consume<T> {
    /// Consumes the complete iterator to decode a value of `T`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the stream terminates prematurely
    /// or contains excess bytes after the decoded value.
    fn consume(self) -> Result<T>;

    /// Decodes one value from the iterator and allows trailing bytes.
    ///
    /// Returns [`None`] when the iterator does not contain enough bytes for a
    /// complete value.
    fn consume_partial(self) -> Option<T>;
}

impl<T, I> Consume<T> for I
where
    T: FromLeStream,
    I: Iterator<Item = u8>,
{
    fn consume(self) -> Result<T> {
        T::from_le_stream_exact(self)
    }

    fn consume_partial(self) -> Option<T> {
        T::from_le_stream(self)
    }
}
