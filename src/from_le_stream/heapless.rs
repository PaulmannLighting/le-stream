#![cfg(feature = "heapless")]

use core::iter::once;

use heapless::{String, Vec};

use crate::FromLeStream;

impl<T, const SIZE: usize> FromLeStream for Vec<T, SIZE>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = Self::new();

        for _ in 0..SIZE {
            let Some(byte) = bytes.next() else {
                break;
            };

            result
                .push(T::from_le_stream(once(byte).chain(&mut bytes))?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)
    }
}

impl<const SIZE: usize> FromLeStream for String<SIZE> {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Vec::<u8, SIZE>::from_le_stream(&mut bytes).and_then(|vec| Self::from_utf8(vec).ok())
    }
}
