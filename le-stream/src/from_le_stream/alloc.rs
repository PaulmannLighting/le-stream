#![cfg(feature = "alloc")]

//! Deserialization for unprefixed allocated collections.
//!
//! `Vec<T>` and `Box<[T]>` do not read a length prefix. They repeatedly
//! deserialize complete `T` values until the byte stream is exhausted. Use
//! `Prefixed<P, D>` for allocated collections whose stream representation must
//! include an explicit length.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::once;

use crate::FromLeStream;

impl<T> FromLeStream for Vec<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = Self::new();

        while let Some(byte) = bytes.next() {
            result.push(T::from_le_stream(once(byte).chain(&mut bytes))?);
        }

        Some(result)
    }
}

impl<T> FromLeStream for Box<[T]>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        Vec::from_le_stream(&mut bytes).map(Vec::into_boxed_slice)
    }
}

impl<T> FromLeStream for Box<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        T::from_le_stream(&mut bytes).map(Self::new)
    }
}
