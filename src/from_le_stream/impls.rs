use std::fmt::Debug;
use std::iter::once;

use crate::FromLeStream;

impl FromLeStream for () {
    fn from_le_stream<T>(_: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Some(())
    }
}

impl FromLeStream for bool {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        bytes.next().map(|byte| byte != 0)
    }
}

impl FromLeStream for u8 {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = Self>,
    {
        bytes.next()
    }
}

impl FromLeStream for u16 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for u32 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for u64 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for u128 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for usize {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i8 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i16 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i32 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i64 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl<T, const SIZE: usize> FromLeStream for [T; SIZE]
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut array = [const { None }; SIZE];

        for item in &mut array {
            item.replace(T::from_le_stream(&mut bytes)?);
        }

        Some(array.map(Option::unwrap))
    }
}

impl<T> FromLeStream for Option<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        bytes.next().map_or_else(
            || Some(None),
            |byte| T::from_le_stream(once(byte).chain(bytes)).map(Some),
        )
    }
}

impl<T> FromLeStream for Vec<T>
where
    T: Debug + FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = Self::new();

        while let Some(byte) = bytes.next() {
            result.push(<T as FromLeStream>::from_le_stream(
                once(byte).chain(&mut bytes),
            )?);
        }

        Some(result)
    }
}

#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> FromLeStream for heapless::Vec<T, SIZE>
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
                .push(<T as FromLeStream>::from_le_stream(
                    once(byte).chain(&mut bytes),
                )?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)
    }
}

#[cfg(feature = "heapless")]
impl<const SIZE: usize> FromLeStream for heapless::String<SIZE> {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        heapless::Vec::<u8, SIZE>::from_le_stream(&mut bytes)
            .and_then(|vec| Self::from_utf8(vec).ok())
    }
}

#[cfg(feature = "macaddr")]
impl FromLeStream for macaddr::MacAddr6 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; 6]>::from_le_stream(bytes).map(|[f, e, d, c, b, a]| Self::new(a, b, c, d, e, f))
    }
}

#[cfg(feature = "macaddr")]
impl FromLeStream for macaddr::MacAddr8 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; 8]>::from_le_stream(bytes)
            .map(|[h, g, f, e, d, c, b, a]| Self::new(a, b, c, d, e, f, g, h))
    }
}
