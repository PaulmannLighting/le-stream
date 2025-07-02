use crate::{FromLeStream, Prefixed};

/// A vector with a prefix of type [`u16`] that can hold up to [`u16::MAX`] elements.
pub type WordSizedVec<T> = heapless::Vec<T, { u16::MAX as usize }>;

impl<T> Prefixed<u16, WordSizedVec<T>> {
    /// Create a new prefixed vec from a heapless Vec.
    pub fn new(data: WordSizedVec<T>) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            prefix: data.len() as u16,
            data,
        }
    }
}

impl<T> From<WordSizedVec<T>> for Prefixed<u16, WordSizedVec<T>> {
    fn from(vec: WordSizedVec<T>) -> Self {
        Self::new(vec)
    }
}

impl<T> FromLeStream for Prefixed<u16, WordSizedVec<T>>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = u16::from_le_stream(&mut bytes)?;

        let mut data = heapless::Vec::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(Self { prefix: size, data })
    }
}
