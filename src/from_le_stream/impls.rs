use std::fmt::Debug;
use std::iter::once;
use std::mem::zeroed;

use crate::{Error, FromLeStream, Result};

impl FromLeStream for () {
    fn from_le_stream<T>(_: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        Ok(())
    }
}

impl FromLeStream for bool {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        bytes
            .next()
            .ok_or(Error::UnexpectedEndOfStream)
            .map(|byte| byte != 0)
    }
}

impl FromLeStream for u8 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = Self>,
    {
        bytes.next().ok_or(Error::UnexpectedEndOfStream)
    }
}

impl FromLeStream for u16 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 2];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl FromLeStream for u32 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 4];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl FromLeStream for u64 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 8];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl FromLeStream for u128 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 16];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl FromLeStream for i8 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        bytes
            .next()
            .ok_or(Error::UnexpectedEndOfStream)
            .map(|byte| Self::from_le_bytes([byte]))
    }
}

impl FromLeStream for i16 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 2];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl FromLeStream for i32 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 4];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl FromLeStream for i64 {
    fn from_le_stream<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut buffer = [0; 8];

        for byte in &mut buffer {
            *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
        }

        Ok(Self::from_le_bytes(buffer))
    }
}

impl<T, const SIZE: usize> FromLeStream for [T; SIZE]
where
    T: FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        #[allow(unsafe_code)]
        // SAFETY: We will initialize all elements of the array in the for loop below.
        let mut result: [T; SIZE] = unsafe { zeroed() };

        for item in &mut result {
            *item = <T as FromLeStream>::from_le_stream(bytes)?;
        }

        Ok(result)
    }
}

impl<T> FromLeStream for Option<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        bytes.next().map_or_else(
            || Ok(None),
            |byte| T::from_le_stream(&mut once(byte).chain(bytes)).map(Some),
        )
    }
}

#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> FromLeStream for heapless::Vec<T, SIZE>
where
    T: Debug + FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = parse_size::<SIZE, I>(bytes)?;
        let mut result = Self::new();

        for _ in 0..size {
            result
                .push(<T as FromLeStream>::from_le_stream(bytes)?)
                .unwrap_or_else(|_| unreachable!());
        }

        Ok(result)
    }
}

#[cfg(feature = "heapless")]
fn parse_size<const SIZE: usize, T>(bytes: &mut T) -> Result<usize>
where
    T: Iterator<Item = u8>,
{
    if u8::try_from(SIZE).is_ok() {
        <u8 as FromLeStream>::from_le_stream(bytes).map(|size| size as usize)
    } else if u16::try_from(SIZE).is_ok() {
        <u16 as FromLeStream>::from_le_stream(bytes).map(|size| size as usize)
    } else if u32::try_from(SIZE).is_ok() {
        <u32 as FromLeStream>::from_le_stream(bytes).map(|size| size as usize)
    } else {
        <u64 as FromLeStream>::from_le_stream(bytes)
            .map(|size| usize::try_from(size).expect("usize exceeds u64"))
    }
}
