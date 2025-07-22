use crate::FromLeStream;

/// Parse an object from a stream of bytes with little endianness with a leading tag.
pub trait FromLeStreamTagged: Sized {
    /// The prefixed tag type.
    type Tag;

    /// Parse an object from a stream of bytes with little endianness having the respective tag prefix.
    ///
    /// # Errors
    ///
    /// Returns the tag if it is invalid.
    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>;
}

impl<T> FromLeStream for T
where
    T: FromLeStreamTagged<Tag: FromLeStream>,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<T>
    where
        I: Iterator<Item = u8>,
    {
        T::from_le_stream_tagged(T::Tag::from_le_stream(&mut bytes)?, bytes)
            .ok()
            .flatten()
    }
}
