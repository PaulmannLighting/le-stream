#![cfg(feature = "heapless")]

use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use crate::{FromLeStream, Prefixed, ToLeStream};

type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;

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
    T: FromLeStream + Debug,
{
    /// Read a [`heapless::Vec`] with size prefixed by a `u8` from a little endian byte stream.
    ///
    /// # Panics
    ///
    /// Panics if the size prefix exceeds the vector's capacity.
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = u8::from_le_stream(&mut bytes)?;
        let mut data = heapless::Vec::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|_| unreachable!("Size cannot exceed vector capacity."));
        }

        Some(Self::new(data))
    }
}

impl<T> ToLeStream for Prefixed<u8, ByteSizedVec<T>>
where
    T: ToLeStream,
{
    type Iter = Chain<<u8 as ToLeStream>::Iter, <ByteSizedVec<T> as ToLeStream>::Iter>;

    /// Convert a [`heapless::Vec`] with size prefixed by a `u8` into a little endian byte stream.
    ///
    /// # Panics
    ///
    /// Panics if the vector's size exceeds the capacity of a `u8`.
    fn to_le_stream(self) -> Self::Iter {
        u8::try_from(self.len())
            .unwrap_or_else(|_| unreachable!("Size cannot exceed u8 capacity."))
            .to_le_stream()
            .chain(self.data.to_le_stream())
    }
}

impl<T> From<ByteSizedVec<T>> for Prefixed<u8, ByteSizedVec<T>> {
    fn from(data: ByteSizedVec<T>) -> Self {
        Self::new(data)
    }
}
