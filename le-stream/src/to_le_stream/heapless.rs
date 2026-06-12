#![cfg(feature = "heapless")]

use core::iter::{Chain, FlatMap};

use heapless::{String, Vec};

use crate::ToLeStream;

impl<T, const SIZE: usize> ToLeStream for Vec<T, SIZE, u8>
where
    T: ToLeStream,
{
    type Iter = Chain<
        <u8 as ToLeStream>::Iter,
        FlatMap<
            <Self as IntoIterator>::IntoIter,
            <T as ToLeStream>::Iter,
            fn(T) -> <T as ToLeStream>::Iter,
        >,
    >;

    fn to_le_stream(self) -> Self::Iter {
        #[expect(clippy::cast_possible_truncation)]
        // LenT is u8, so this cast is safe.
        let len = self.len() as u8;
        #[expect(trivial_casts)]
        len.to_le_stream()
            .chain(self.into_iter().flat_map(ToLeStream::to_le_stream as _))
    }
}

impl<const SIZE: usize> ToLeStream for String<SIZE, u8> {
    type Iter = <Vec<u8, SIZE, u8> as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        self.into_bytes().to_le_stream()
    }
}
