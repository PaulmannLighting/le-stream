#![cfg(feature = "heapless")]

use core::iter::Chain;
use std::marker::PhantomData;

use crate::{FromLeStream, Prefixed, ToLeStream};

/// A vector with a prefix of type [`u8`] that can hold up to [`u8::MAX`] elements.
pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;

impl<T> Prefixed<u8, ByteSizedVec<T>> {
    /// Create a new `Prefixed<u8, ByteSizedVec<T>>` with the given data.
    #[must_use]
    pub const fn new(data: ByteSizedVec<T>) -> Self {
        Self {
            data,
            prefix: PhantomData,
        }
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

        Some(Self::new(data))
    }
}

impl<T> ToLeStream for Prefixed<u8, ByteSizedVec<T>>
where
    T: ToLeStream,
{
    type Iter = Chain<<u8 as ToLeStream>::Iter, <ByteSizedVec<T> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        #[allow(clippy::cast_possible_truncation)]
        (self.data.len() as u8)
            .to_le_stream()
            .chain(self.data.to_le_stream())
    }
}

impl<T> From<ByteSizedVec<T>> for Prefixed<u8, ByteSizedVec<T>> {
    fn from(data: ByteSizedVec<T>) -> Self {
        Self::new(data)
    }
}
