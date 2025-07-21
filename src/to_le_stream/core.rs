use core::array::IntoIter;
use core::iter::{empty, Empty, FlatMap};

use crate::ToLeStream;

mod option_iterator;

impl ToLeStream for () {
    type Iter = Empty<u8>;

    fn to_le_stream(self) -> Self::Iter {
        empty()
    }
}

impl ToLeStream for bool {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}

impl ToLeStream for u8 {
    type Iter = IntoIter<Self, 1>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for u16 {
    type Iter = IntoIter<u8, 2>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for u32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for u64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for u128 {
    type Iter = IntoIter<u8, 16>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for usize {
    type Iter = IntoIter<u8, { size_of::<Self>() }>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for i8 {
    type Iter = IntoIter<u8, 1>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for i16 {
    type Iter = IntoIter<u8, 2>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for i32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for i64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl<T, const SIZE: usize> ToLeStream for [T; SIZE]
where
    T: ToLeStream,
{
    type Iter = FlatMap<IntoIter<T, SIZE>, T::Iter, fn(T) -> T::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter().flat_map(T::to_le_stream)
    }
}

impl<T> ToLeStream for Option<T>
where
    T: ToLeStream,
{
    type Iter = option_iterator::OptionIterator<T>;

    fn to_le_stream(self) -> Self::Iter {
        option_iterator::OptionIterator::new(self)
    }
}
