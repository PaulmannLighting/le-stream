use crate::ToLeBytes;
use std::array::IntoIter;
use std::iter::{empty, Empty};

impl ToLeBytes for bool {
    type Iter = IntoIter<u8, 1>;

    fn to_le_bytes(&self) -> Self::Iter {
        u8::to_le_bytes(u8::from(*self)).into_iter()
    }
}

impl ToLeBytes for u8 {
    type Iter = IntoIter<Self, 1>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for u16 {
    type Iter = IntoIter<u8, 2>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for u32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for u64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for i8 {
    type Iter = IntoIter<u8, 1>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for i16 {
    type Iter = IntoIter<u8, 2>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for i32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl ToLeBytes for i64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_bytes(&self) -> Self::Iter {
        Self::to_le_bytes(*self).into_iter()
    }
}

impl<I, const SIZE: usize> ToLeBytes for [I; SIZE]
where
    I: Copy + Default + ToLeBytes,
    <I as ToLeBytes>::Iter: 'static,
{
    type Iter = Box<dyn Iterator<Item = u8>>;

    fn to_le_bytes(&self) -> Self::Iter {
        let mut iterator: Box<dyn Iterator<Item = u8>> = Box::<Empty<u8>>::default();

        for item in self {
            iterator = Box::new(iterator.chain(<I as ToLeBytes>::to_le_bytes(item)));
        }

        iterator
    }
}

#[allow(clippy::cast_possible_truncation)]
#[cfg(feature = "heapless")]
impl<I, const SIZE: usize> ToLeBytes for heapless::Vec<I, SIZE>
where
    I: ToLeBytes,
    <I as ToLeBytes>::Iter: Iterator<Item = u8> + 'static,
{
    type Iter = Box<dyn Iterator<Item = u8> + 'static>;

    fn to_le_bytes(&self) -> Self::Iter {
        let mut iterator: Box<dyn Iterator<Item = u8>> = Box::new(empty());

        if u8::try_from(SIZE).is_ok() {
            iterator = Box::new(<u8 as ToLeBytes>::to_le_bytes(&(self.len() as u8)));
        } else if u16::try_from(SIZE).is_ok() {
            iterator = Box::new(<u16 as ToLeBytes>::to_le_bytes(&(self.len() as u16)));
        } else if u32::try_from(SIZE).is_ok() {
            iterator = Box::new(<u32 as ToLeBytes>::to_le_bytes(&(self.len() as u32)));
        } else if u64::try_from(SIZE).is_ok() {
            iterator = Box::new(<u64 as ToLeBytes>::to_le_bytes(&(self.len() as u64)));
        }

        for item in self {
            iterator = Box::new(iterator.chain(<I as ToLeBytes>::to_le_bytes(item)));
        }

        iterator
    }
}
