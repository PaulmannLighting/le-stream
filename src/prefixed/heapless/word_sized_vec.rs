use std::iter::Chain;

use crate::{FromLeStream, Prefixed, ToLeStream};

/// A vector with a prefix of type [`u16`] that can hold up to [`u16::MAX`] elements.
pub type WordSizedVec<T> = heapless::Vec<T, { u16::MAX as usize }>;

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

        Some(Self::new(data))
    }
}

impl<T> ToLeStream for Prefixed<u16, WordSizedVec<T>>
where
    T: ToLeStream,
{
    type Iter = Chain<<u16 as ToLeStream>::Iter, <WordSizedVec<T> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        #[allow(clippy::cast_possible_truncation)]
        (self.data.len() as u16)
            .to_le_stream()
            .chain(self.data.to_le_stream())
    }
}
