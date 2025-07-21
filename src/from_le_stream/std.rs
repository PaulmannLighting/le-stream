#![cfg(feature = "std")]

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
