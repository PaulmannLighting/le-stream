#![cfg(feature = "heapless")]
use crate::ToLeBytes;
use std::array::IntoIter;

#[derive(Debug)]
pub enum SizePrefixIterator {
    U8(IntoIter<u8, 1>),
    U16(IntoIter<u8, 2>),
    U32(IntoIter<u8, 4>),
    U64(IntoIter<u8, 8>),
}

impl SizePrefixIterator {
    #[allow(clippy::cast_possible_truncation)]
    fn new(len: usize, capacity: usize) -> Self {
        if u8::try_from(capacity).is_ok() {
            Self::U8(<u8 as ToLeBytes>::to_le_bytes(len as u8))
        } else if u16::try_from(capacity).is_ok() {
            Self::U16(<u16 as ToLeBytes>::to_le_bytes(len as u16))
        } else if u32::try_from(capacity).is_ok() {
            Self::U32(<u32 as ToLeBytes>::to_le_bytes(len as u32))
        } else if u64::try_from(capacity).is_ok() {
            Self::U64(<u64 as ToLeBytes>::to_le_bytes(len as u64))
        } else {
            unreachable!("container size exceeds u64");
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
        }
    }
}

impl<T, const SIZE: usize> From<&heapless::Vec<T, SIZE>> for SizePrefixIterator
where
    T: ToLeBytes,
{
    fn from(vec: &heapless::Vec<T, SIZE>) -> Self {
        Self::new(vec.len(), SIZE)
    }
}
