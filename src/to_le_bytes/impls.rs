mod container_iterator;

use crate::ToLeBytes;
use container_iterator::ContainerIterator;
use std::array::IntoIter;

impl ToLeBytes for bool {
    type Iter = IntoIter<u8, 1>;

    fn to_le_bytes(self) -> Self::Iter {
        u8::to_le_bytes(u8::from(self)).into_iter()
    }
}

impl ToLeBytes for u8 {
    type Iter = IntoIter<Self, 1>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for u16 {
    type Iter = IntoIter<u8, 2>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for u32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for u64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for i8 {
    type Iter = IntoIter<u8, 1>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for i16 {
    type Iter = IntoIter<u8, 2>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for i32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl ToLeBytes for i64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_bytes(self) -> Self::Iter {
        Self::to_le_bytes(self).into_iter()
    }
}

impl<T, const SIZE: usize> ToLeBytes for [T; SIZE]
where
    T: ToLeBytes,
{
    type Iter = ContainerIterator<Self>;

    fn to_le_bytes(self) -> Self::Iter {
        ContainerIterator::from(self)
    }
}

#[allow(clippy::cast_possible_truncation)]
#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> ToLeBytes for heapless::Vec<T, SIZE>
where
    T: ToLeBytes,
{
    type Iter = ContainerIterator<Self>;

    fn to_le_bytes(self) -> Self::Iter {
        ContainerIterator::from(self)
    }
}
