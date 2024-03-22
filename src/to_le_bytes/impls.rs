mod size_prefix_iterator;

use crate::ToLeBytes;
use std::array::IntoIter;
use std::iter::{empty, Empty, FlatMap};

impl ToLeBytes for () {
    type Iter = Empty<u8>;

    fn to_le_bytes(self) -> Self::Iter {
        empty()
    }
}

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
    type Iter = FlatMap<IntoIter<T, SIZE>, <T as ToLeBytes>::Iter, fn(T) -> <T as ToLeBytes>::Iter>;

    fn to_le_bytes(self) -> Self::Iter {
        self.into_iter().flat_map(<T as ToLeBytes>::to_le_bytes)
    }
}

#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> ToLeBytes for heapless::Vec<T, SIZE>
where
    T: Sized + ToLeBytes,
{
    type Iter = std::iter::Chain<
        size_prefix_iterator::SizePrefixIterator,
        FlatMap<
            <Self as IntoIterator>::IntoIter,
            <T as ToLeBytes>::Iter,
            fn(T) -> <T as ToLeBytes>::Iter,
        >,
    >;

    fn to_le_bytes(self) -> Self::Iter {
        size_prefix_iterator::SizePrefixIterator::new(self.len(), SIZE).chain(
            self.into_iter()
                .flat_map(<T as ToLeBytes>::to_le_bytes as fn(T) -> <T as ToLeBytes>::Iter),
        )
    }
}
