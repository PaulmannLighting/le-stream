use std::array::IntoIter;

use crate::ToLeStream;

#[derive(Debug)]
pub enum SizePrefixIterator {
    U8(IntoIter<u8, 1>),
    U16(IntoIter<u8, 2>),
    U32(IntoIter<u8, 4>),
    U64(IntoIter<u8, 8>),
    U128(IntoIter<u8, 16>),
}

impl SizePrefixIterator {
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(len: usize, capacity: usize) -> Self {
        if u8::try_from(capacity).is_ok() {
            Self::U8(<u8 as ToLeStream>::to_le_stream(len as u8))
        } else if u16::try_from(capacity).is_ok() {
            Self::U16(<u16 as ToLeStream>::to_le_stream(len as u16))
        } else if u32::try_from(capacity).is_ok() {
            Self::U32(<u32 as ToLeStream>::to_le_stream(len as u32))
        } else if u64::try_from(capacity).is_ok() {
            Self::U64(<u64 as ToLeStream>::to_le_stream(len as u64))
        } else if u128::try_from(capacity).is_ok() {
            Self::U128(<u128 as ToLeStream>::to_le_stream(len as u128))
        } else {
            unreachable!("container size exceeds u128");
        }
    }
}

impl Iterator for SizePrefixIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::U8(header) => header.next(),
            Self::U16(header) => header.next(),
            Self::U32(header) => header.next(),
            Self::U64(header) => header.next(),
            Self::U128(header) => header.next(),
        }
    }
}
