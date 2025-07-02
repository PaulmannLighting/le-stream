use crate::{FromLeStream, Prefixed};

/// A vector with a prefix of type [`u8`] that can hold up to [`u8::MAX`] elements.
pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;

impl<T> Prefixed<u8, ByteSizedVec<T>> {
    /// Create a new prefixed vec from a heapless Vec.
    pub fn new(data: ByteSizedVec<T>) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            prefix: data.len() as u8,
            data,
        }
    }
}

impl<T> From<ByteSizedVec<T>> for Prefixed<u8, ByteSizedVec<T>> {
    fn from(vec: ByteSizedVec<T>) -> Self {
        Self::new(vec)
    }
}

impl<T> FromLeStream for Prefixed<u8, ByteSizedVec<T>>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = u8::from_le_stream(&mut bytes)?;

        let mut data = heapless::Vec::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(Self { prefix: size, data })
    }
}
