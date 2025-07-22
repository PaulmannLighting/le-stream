use crate::FromLeStream;

/// Internal result type.
type Result<T> = core::result::Result<T, FromLeStreamTaggedError<<T as FromLeStreamTagged>::Tag>>;

/// Indicates an error when parsing an object from a stream of bytes with little endianness with a leading tag.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FromLeStreamTaggedError<T> {
    /// The tag is invalid.
    InvalidTag(T),
    /// The byte stream terminated prematurely.
    UnexpectedEndOfStream,
}

/// Parse an object from a stream of bytes with little endianness with a leading tag.
pub trait FromLeStreamTagged: Sized {
    /// The prefixed tag type.
    type Tag;

    /// Parse an object from a stream of bytes with little endianness having the respective tag prefix.
    ///
    /// # Errors
    ///
    /// Returns an [`FromLeStreamTaggedError`] if the tag is invalid or the stream terminates prematurely.
    fn from_le_stream_tagged<I>(tag: Self::Tag, bytes: I) -> Result<Self>
    where
        I: Iterator<Item = u8>;
}

impl<T> FromLeStream for T
where
    T: FromLeStreamTagged<Tag: FromLeStream>,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<T>
    where
        I: Iterator<Item = u8>,
    {
        T::from_le_stream_tagged(T::Tag::from_le_stream(&mut bytes)?, bytes).ok()
    }
}
