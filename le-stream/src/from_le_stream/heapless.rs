#![cfg(feature = "heapless")]

use heapless::{LenType, String, Vec};

use crate::FromLeStream;

impl<T, const SIZE: usize, LenT> FromLeStream for Vec<T, SIZE, LenT>
where
    T: FromLeStream,
    LenT: LenType + FromLeStream + Into<usize>,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = LenT::from_le_stream(bytes.by_ref())?.into();
        let mut result = Self::new();

        for _ in 0..size {
            result
                .push(T::from_le_stream(bytes.by_ref())?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)
    }
}

impl<const SIZE: usize, LenT> FromLeStream for String<SIZE, LenT>
where
    LenT: LenType + FromLeStream + Into<usize>,
{
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Vec::from_le_stream(&mut bytes).and_then(|vec| Self::from_utf8(vec).ok())
    }
}
