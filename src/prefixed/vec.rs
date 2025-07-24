#![cfg(feature = "std")]

use core::iter::Chain;

use crate::{FromLeStream, Prefixed, ToLeStream};

impl<P, T> Prefixed<P, Vec<T>> {
    /// Return a slice of the data contained in the vector.
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<P, T> FromLeStream for Prefixed<P, Vec<T>>
where
    P: Copy + FromLeStream + Into<usize>,
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let prefix = P::from_le_stream(&mut bytes)?;
        let size: usize = prefix.into();

        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?);
        }

        Some(Self::new(data))
    }
}

impl<P, T> ToLeStream for Prefixed<P, Vec<T>>
where
    P: From<usize> + ToLeStream,
    T: ToLeStream,
{
    type Iter = Chain<P::Iter, <Vec<T> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        P::from(self.data.len())
            .to_le_stream()
            .chain(self.data.to_le_stream())
    }
}
