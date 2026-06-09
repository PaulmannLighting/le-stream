#![cfg(feature = "heapless")]

use core::iter::FlatMap;

use heapless::{LenType, String, Vec};

use crate::ToLeStream;

impl<T, const SIZE: usize, LenT> ToLeStream for Vec<T, SIZE, LenT>
where
    T: ToLeStream,
    LenT: LenType,
{
    type Iter = FlatMap<<Self as IntoIterator>::IntoIter, T::Iter, fn(T) -> T::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter().flat_map(ToLeStream::to_le_stream)
    }
}

impl<const SIZE: usize, LenT> ToLeStream for String<SIZE, LenT>
where
    LenT: LenType,
{
    type Iter = <Vec<u8, SIZE, LenT> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        self.into_bytes().into_iter()
    }
}
