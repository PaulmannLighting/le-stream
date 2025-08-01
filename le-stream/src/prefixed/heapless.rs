#![cfg(feature = "heapless")]

use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use log::{error, warn};

use crate::{FromLeStream, Prefixed, ToLeStream};

/// A vector with a prefix of type [`u8`] that can hold up to [`u8::MAX`] elements.
pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;

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
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = u8::from_le_stream(&mut bytes)?;

        if CAPACITY < usize::from(size) {
            warn!("Received size prefix exceeds the vector's capacity: {size} > {CAPACITY}");
        }

        let mut data = heapless::Vec::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|item| error!("Vector overflow. Discarding item: {item:?}"));
        }

        Some(Self::new(data))
    }
}

impl<T, const CAPACITY: usize> ToLeStream for Prefixed<u8, heapless::Vec<T, CAPACITY>>
where
    T: ToLeStream,
{
    type Iter = Chain<<u8 as ToLeStream>::Iter, <heapless::Vec<T, CAPACITY> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        #[allow(clippy::cast_possible_truncation)]
        (self.data.len() as u8)
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
