use std::num::TryFromIntError;

use crate::{FromLeStream, ToLeStream};

use iter::SizePrefixIterator;

mod iter;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Size {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl Size {
    pub fn parse<const SIZE: usize>(bytes: impl Iterator<Item = u8>) -> Option<Self> {
        if u8::try_from(SIZE).is_ok() {
            <u8 as FromLeStream>::from_le_stream(bytes).map(Self::U8)
        } else if u16::try_from(SIZE).is_ok() {
            <u16 as FromLeStream>::from_le_stream(bytes).map(Self::U16)
        } else if u32::try_from(SIZE).is_ok() {
            <u32 as FromLeStream>::from_le_stream(bytes).map(Self::U32)
        } else if u64::try_from(SIZE).is_ok() {
            <u64 as FromLeStream>::from_le_stream(bytes).map(Self::U64)
        } else if u128::try_from(SIZE).is_ok() {
            <u128 as FromLeStream>::from_le_stream(bytes).map(Self::U128)
        } else {
            panic!("usize exceeds u128");
        }
    }
}

impl ToLeStream for Size {
    type Iter = SizePrefixIterator;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::U8(value) => SizePrefixIterator::U8(value.to_le_stream()),
            Self::U16(value) => SizePrefixIterator::U16(value.to_le_stream()),
            Self::U32(value) => SizePrefixIterator::U32(value.to_le_stream()),
            Self::U64(value) => SizePrefixIterator::U64(value.to_le_stream()),
            Self::U128(value) => SizePrefixIterator::U128(value.to_le_stream()),
        }
    }
}

impl From<u8> for Size {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for Size {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for Size {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for Size {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<u128> for Size {
    fn from(value: u128) -> Self {
        Self::U128(value)
    }
}

impl TryFrom<usize> for Size {
    type Error = TryFromIntError;

    fn try_from(size: usize) -> Result<Self, Self::Error> {
        u8::try_from(size).map_or_else(
            |_| {
                u16::try_from(size).map_or_else(
                    |_| {
                        u32::try_from(size).map_or_else(
                            |_| {
                                u64::try_from(size).map_or_else(
                                    |_| u128::try_from(size).map(Self::U128),
                                    |size| Ok(Self::U64(size)),
                                )
                            },
                            |size| Ok(Self::U32(size)),
                        )
                    },
                    |size| Ok(Self::U16(size)),
                )
            },
            |size| Ok(Self::U8(size)),
        )
    }
}

impl TryFrom<Size> for usize {
    type Error = TryFromIntError;

    fn try_from(size: Size) -> Result<Self, Self::Error> {
        match size {
            Size::U8(value) => Ok(value.into()),
            Size::U16(value) => Ok(value.into()),
            Size::U32(value) => value.try_into(),
            Size::U64(value) => value.try_into(),
            Size::U128(value) => value.try_into(),
        }
    }
}
