use core::iter::once;
use core::marker::PhantomData;

use crate::FromLeStream;

impl FromLeStream for () {
    fn from_le_stream<T>(_: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Some(())
    }
}

impl<T> FromLeStream for PhantomData<T> {
    fn from_le_stream<I>(_: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        Some(Self)
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
