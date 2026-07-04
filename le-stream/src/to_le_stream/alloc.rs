#![cfg(feature = "alloc")]

//! Serialization for unprefixed allocated collections.
//!
//! `Vec<T>` and `Box<[T]>` serialize by concatenating their elements without a
//! length prefix. Use `Prefixed<P, D>` for allocated collections whose stream
//! representation must include an explicit length.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::FlatMap;

use crate::ToLeStream;

impl<T> ToLeStream for Vec<T>
where
    T: ToLeStream,
{
    type Iter = FlatMap<<Self as IntoIterator>::IntoIter, T::Iter, fn(T) -> T::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter().flat_map(ToLeStream::to_le_stream)
    }
}

impl<T> ToLeStream for Box<[T]>
where
    T: ToLeStream,
{
    type Iter = FlatMap<<Self as IntoIterator>::IntoIter, T::Iter, fn(T) -> T::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter().flat_map(ToLeStream::to_le_stream)
    }
}

impl<T> ToLeStream for Box<T>
where
    T: ToLeStream,
{
    type Iter = <T as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        ToLeStream::to_le_stream(*self)
    }
}
