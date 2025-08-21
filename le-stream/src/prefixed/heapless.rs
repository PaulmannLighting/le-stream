#![cfg(feature = "heapless")]

use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use crate::{FromLeStream, Prefixed, ToLeStream};

impl<T, const CAPACITY: usize> Prefixed<u8, heapless::Vec<T, CAPACITY>> {
    /// Create a new `Prefixed<u8, ByteSizedVec<T>>` with the given data.
    #[must_use]
    pub const fn new(data: heapless::Vec<T, CAPACITY>) -> Self {
        Self {
            data,
            prefix: PhantomData,
        }
    }
}

impl<T, const CAPACITY: usize> FromLeStream for Prefixed<u8, heapless::Vec<T, CAPACITY>>
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

        assert!(
            CAPACITY >= usize::from(size),
            "Received size prefix exceeds the vector's capacity: {size} > {CAPACITY}"
        );

        let mut data = heapless::Vec::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|_| {
                    unreachable!("We asserted that the size prefix is within capacity.")
                });
        }

        Some(Self::new(data))
    }
}

impl<T, const CAPACITY: usize> ToLeStream for Prefixed<u8, heapless::Vec<T, CAPACITY>>
where
    T: ToLeStream,
{
    type Iter = Chain<<u8 as ToLeStream>::Iter, <heapless::Vec<T, CAPACITY> as ToLeStream>::Iter>;

    /// Convert a [`heapless::Vec`] with size prefixed by a `u8` into a little endian byte stream.
    ///
    /// # Panics
    ///
    /// Panics if the vector's size exceeds the capacity of a `u8`.
    fn to_le_stream(self) -> Self::Iter {
        u8::try_from(self.len())
            .expect("Vector size exceeds u8 capacity.")
            .to_le_stream()
            .chain(self.data.to_le_stream())
    }
}

impl<T, const CAPACITY: usize> From<heapless::Vec<T, CAPACITY>>
    for Prefixed<u8, heapless::Vec<T, CAPACITY>>
{
    fn from(data: heapless::Vec<T, CAPACITY>) -> Self {
        Self::new(data)
    }
}
