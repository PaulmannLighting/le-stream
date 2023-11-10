use crate::ToLeBytes;
use std::array::IntoIter;
use std::iter::FlatMap;

type ItemFlatMap<T> = FlatMap<
    <T as IntoIterator>::IntoIter,
    <<T as IntoIterator>::Item as ToLeBytes>::Iter,
    fn(<T as IntoIterator>::Item) -> <<T as IntoIterator>::Item as ToLeBytes>::Iter,
>;

pub enum ContainerIterator<T>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: ToLeBytes,
{
    Const(ItemFlatMap<T>),
    U8(IntoIter<u8, 1>, ItemFlatMap<T>),
    U16(IntoIter<u8, 2>, ItemFlatMap<T>),
    U32(IntoIter<u8, 4>, ItemFlatMap<T>),
    U64(IntoIter<u8, 8>, ItemFlatMap<T>),
}

impl<T> ContainerIterator<T>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: ToLeBytes,
{
    #[allow(clippy::cast_possible_truncation)]
    fn dynamically_sized(items: T, len: usize, capacity: usize) -> Self {
        if u8::try_from(capacity).is_ok() {
            Self::U8(
                <u8 as ToLeBytes>::to_le_bytes(len as u8),
                items
                    .into_iter()
                    .flat_map(<<T as IntoIterator>::Item as ToLeBytes>::to_le_bytes),
            )
        } else if u16::try_from(capacity).is_ok() {
            Self::U16(
                <u16 as ToLeBytes>::to_le_bytes(len as u16),
                items
                    .into_iter()
                    .flat_map(<<T as IntoIterator>::Item as ToLeBytes>::to_le_bytes),
            )
        } else if u32::try_from(capacity).is_ok() {
            Self::U32(
                <u32 as ToLeBytes>::to_le_bytes(len as u32),
                items
                    .into_iter()
                    .flat_map(<<T as IntoIterator>::Item as ToLeBytes>::to_le_bytes),
            )
        } else if u64::try_from(capacity).is_ok() {
            Self::U64(
                <u64 as ToLeBytes>::to_le_bytes(len as u64),
                items
                    .into_iter()
                    .flat_map(<<T as IntoIterator>::Item as ToLeBytes>::to_le_bytes),
            )
        } else {
            unreachable!("container size exceeds u64");
        }
    }

    fn const_sized(items: T) -> Self {
        Self::Const(
            items
                .into_iter()
                .flat_map(<<T as IntoIterator>::Item as ToLeBytes>::to_le_bytes),
        )
    }
}

impl<T> Iterator for ContainerIterator<T>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: ToLeBytes,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Const(items) => items.next(),
            Self::U8(header, items) => header.next().map_or_else(|| items.next(), Some),
            Self::U16(header, items) => header.next().map_or_else(|| items.next(), Some),
            Self::U32(header, items) => header.next().map_or_else(|| items.next(), Some),
            Self::U64(header, items) => header.next().map_or_else(|| items.next(), Some),
        }
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for ContainerIterator<[T; SIZE]>
where
    T: ToLeBytes,
{
    fn from(array: [T; SIZE]) -> Self {
        Self::const_sized(array)
    }
}

impl<T> From<Vec<T>> for ContainerIterator<Vec<T>>
where
    T: ToLeBytes,
{
    fn from(vec: Vec<T>) -> Self {
        let len = vec.len();
        Self::dynamically_sized(vec, len, usize::MAX)
    }
}

#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> From<heapless::Vec<T, SIZE>>
    for ContainerIterator<heapless::Vec<T, SIZE>>
where
    T: ToLeBytes,
{
    fn from(vec: heapless::Vec<T, SIZE>) -> Self {
        let len = vec.len();
        Self::dynamically_sized(vec, len, SIZE)
    }
}
