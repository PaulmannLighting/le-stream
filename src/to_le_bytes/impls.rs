use crate::ToLeBytes;
use std::array::IntoIter;
use std::iter::Empty;

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
