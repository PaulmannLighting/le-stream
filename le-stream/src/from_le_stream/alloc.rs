#![cfg(feature = "alloc")]

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
        let (lower, upper) = bytes.size_hint();
        let mut result = Self::with_capacity(upper.unwrap_or(lower));

        while let Some(byte) = bytes.next() {
            result.push(T::from_le_stream(once(byte).chain(&mut bytes))?);
        }

        result.shrink_to_fit();
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
