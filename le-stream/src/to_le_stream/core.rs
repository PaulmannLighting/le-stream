use core::array::IntoIter;
use core::iter::{Empty, FlatMap, empty};
use core::marker::PhantomData;

use crate::ToLeStream;

mod option_iterator;

macro_rules! impl_primitives {
    ($($typ:ty,)+) => {
        $(
            impl ToLeStream for $typ {
                type Iter = IntoIter<u8, { Self::BITS as usize / 8 }>;

                fn to_le_stream(self) -> Self::Iter {
                    self.to_le_bytes().into_iter()
                }
            }
        )+
    };
}

impl_primitives!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
);

impl ToLeStream for f32 {
    type Iter = IntoIter<u8, 4>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for f64 {
    type Iter = IntoIter<u8, 8>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for () {
    type Iter = Empty<u8>;

    fn to_le_stream(self) -> Self::Iter {
        empty()
    }
}

impl<T> ToLeStream for PhantomData<T> {
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
