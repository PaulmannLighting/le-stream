use crate::{Error, FromLeStream};

/// Consumes a stream of bytes with little endianness to crate an object.
pub trait Consume<T> {
    /// Consumes the iterator to create an instance of `T`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the stream terminates prematurely or contains excess bytes.
    fn consume(self) -> Result<T, Error>;

    /// Consumes the iterator partially to create an instance of `T`.
    fn consume_partial(self) -> Option<T>;
}

impl<T, I> Consume<T> for I
where
    T: FromLeStream,
    I: Iterator<Item = u8>,
{
    fn consume(self) -> Result<T, Error> {
        T::from_le_stream_exact(self)
    }

    fn consume_partial(self) -> Option<T> {
        T::from_le_stream(self)
    }
}
