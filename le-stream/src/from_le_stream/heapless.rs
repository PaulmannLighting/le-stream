#![cfg(feature = "heapless")]

use heapless::{String, Vec};

use crate::FromLeStream;

impl<T, const SIZE: usize> FromLeStream for Vec<T, SIZE, u8>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = u8::from_le_stream(bytes.by_ref())?;
        let mut result = Self::new();

        for _ in 0..size {
            result
                .push(T::from_le_stream(bytes.by_ref())?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)
    }
}

impl<const SIZE: usize> FromLeStream for String<SIZE, u8> {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Vec::from_le_stream(&mut bytes).and_then(|vec| Self::from_utf8(vec).ok())
    }
}
