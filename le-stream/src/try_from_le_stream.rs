use crate::FromLeStream;

/// Try to parse an object from a stream of bytes with little endianness.
pub trait TryFromLeStream<T>: TryFrom<T> {
    /// Try to parse an object from a stream of bytes with little endianness.
    ///
    /// # Errors
    ///
    /// Returns `None` if the stream terminates prematurely, or `Some(error)` if the parsing fails.
    fn try_from_le_stream<I>(bytes: I) -> Result<Self, Option<<Self as TryFrom<T>>::Error>>
    where
        I: Iterator<Item = u8>;
}

impl<T, U> TryFromLeStream<T> for U
where
    T: FromLeStream,
    U: TryFrom<T>,
{
    fn try_from_le_stream<I>(bytes: I) -> Result<Self, Option<<U as TryFrom<T>>::Error>>
    where
        I: Iterator<Item = u8>,
    {
        T::from_le_stream(bytes).map_or_else(|| Err(None), |value| U::try_from(value).map_err(Some))
    }
}
