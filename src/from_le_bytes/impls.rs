use crate::{Error, FromLeBytes, Result};

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

impl<I, const SIZE: usize> FromLeBytes for [I; SIZE]
where
    I: Copy + Default + FromLeBytes,
{
    fn from_le_bytes<T>(bytes: &mut T) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut result = [I::default(); SIZE];

        for item in &mut result {
            *item = <I as FromLeBytes>::from_le_bytes(bytes)?;
        }

        Ok(result)
    }
}
