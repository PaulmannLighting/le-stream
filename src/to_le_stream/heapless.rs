#![cfg(feature = "heapless")]

use core::iter::FlatMap;

use heapless::{String, Vec};

use crate::ToLeStream;

impl<T, const SIZE: usize> ToLeStream for Vec<T, SIZE>
where
    T: ToLeStream,
{
    type Iter = FlatMap<<Self as IntoIterator>::IntoIter, T::Iter, fn(T) -> T::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter().flat_map(ToLeStream::to_le_stream)
    }
}

#[cfg(feature = "heapless")]
impl<const SIZE: usize> ToLeStream for String<SIZE> {
    type Iter = <Vec<u8, SIZE> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        self.into_bytes().into_iter()
    }
}
