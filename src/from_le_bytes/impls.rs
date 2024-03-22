use crate::{Error, FromLeBytes, Result};
use std::iter::once;
use std::mem::zeroed;

impl FromLeBytes for () {
    fn from_le_bytes<T>(_: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        Ok(())
    }
}

impl FromLeBytes for bool {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        bytes
            .next()
            .ok_or(Error::UnexpectedEndOfStream)
            .map(|byte| byte != 0)
    }
}

impl FromLeBytes for u8 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = Self>,
    {
        bytes.next().ok_or(Error::UnexpectedEndOfStream)
    }
}

impl FromLeBytes for u16 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
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

impl FromLeBytes for u32 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
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

impl FromLeBytes for u64 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
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

impl FromLeBytes for i8 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        bytes
            .next()
            .ok_or(Error::UnexpectedEndOfStream)
            .map(|byte| Self::from_le_bytes([byte]))
    }
}

impl FromLeBytes for i16 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
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

impl FromLeBytes for i32 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
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

impl FromLeBytes for i64 {
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
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

impl<T, const SIZE: usize> FromLeBytes for [T; SIZE]
where
    T: FromLeBytes,
{
    fn from_le_bytes<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = unsafe { zeroed::<[T; SIZE]>() };

        for item in &mut result {
            *item = <T as FromLeBytes>::from_le_bytes(bytes)?;
        }

        Ok(result)
    }
}

impl<T> FromLeBytes for Option<T>
where
    T: FromLeBytes,
{
    fn from_le_bytes<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        match bytes.next() {
            Some(byte) => Ok(Some(T::from_le_bytes(&mut once(byte).chain(bytes))?)),
            None => Ok(None),
        }
    }
}

#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> FromLeBytes for heapless::Vec<T, SIZE>
where
    T: std::fmt::Debug + FromLeBytes,
{
    fn from_le_bytes<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize;

        if u8::try_from(SIZE).is_ok() {
            size = <u8 as FromLeBytes>::from_le_bytes(bytes)? as usize;
        } else if u16::try_from(SIZE).is_ok() {
            size = <u16 as FromLeBytes>::from_le_bytes(bytes)? as usize;
        } else if u32::try_from(SIZE).is_ok() {
            size = <u32 as FromLeBytes>::from_le_bytes(bytes)? as usize;
        } else {
            size = usize::try_from(<u64 as FromLeBytes>::from_le_bytes(bytes)?)
                .expect("invalid usize");
        }

        let mut result = Self::new();

        for _ in 0..size {
            result
                .push(<T as FromLeBytes>::from_le_bytes(bytes)?)
                .expect("buffer overflow");
        }

        Ok(result)
    }
}
