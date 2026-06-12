#![cfg(feature = "heapless")]

use core::iter::{Chain, FlatMap};

use heapless::{LenType, String, Vec};

use crate::ToLeStream;

impl<T, const SIZE: usize, LenT> ToLeStream for Vec<T, SIZE, LenT>
where
    T: ToLeStream,
    LenT: LenType + TryFrom<usize> + ToLeStream,
{
    type Iter = Chain<
        <LenT as ToLeStream>::Iter,
        FlatMap<
            <Self as IntoIterator>::IntoIter,
            <T as ToLeStream>::Iter,
            fn(T) -> <T as ToLeStream>::Iter,
        >,
    >;

    fn to_le_stream(self) -> Self::Iter {
        let len: LenT = self
            .len()
            .try_into()
            .unwrap_or_else(|_| unreachable!("Size cannot exceed LenT::MAX"));
        #[expect(trivial_casts)]
        len.to_le_stream()
            .chain(self.into_iter().flat_map(ToLeStream::to_le_stream as _))
    }
}

impl<const SIZE: usize, LenT> ToLeStream for String<SIZE, LenT>
where
    LenT: LenType + TryFrom<usize> + ToLeStream,
{
    type Iter = <Vec<u8, SIZE, LenT> as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        self.into_bytes().to_le_stream()
    }
}
