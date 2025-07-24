use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use crate::{FromLeStream, Prefixed, ToLeStream};

impl<P, T> FromLeStream for Prefixed<P, Box<[T]>>
where
    P: Into<usize> + FromLeStream,
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = P::from_le_stream(&mut bytes)?;

        let mut data = Vec::new();

        for _ in 0..Into::<usize>::into(size) {
            data.push(T::from_le_stream(&mut bytes)?);
        }

        Some(Self {
            data: data.into_boxed_slice(),
            prefix: PhantomData,
        })
    }
}

impl<P, T> ToLeStream for Prefixed<P, Box<[T]>>
where
    P: TryFrom<usize, Error: Debug> + ToLeStream,
    T: ToLeStream,
{
    type Iter = Chain<<P as ToLeStream>::Iter, <Vec<T> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        <P as TryFrom<usize>>::try_from(self.data.len())
            .expect("amount of items should fit into prefix")
            .to_le_stream()
            .chain(self.data.to_le_stream())
    }
}

impl<T> From<Box<[T]>> for Prefixed<usize, Box<[T]>> {
    fn from(data: Box<[T]>) -> Self {
        Self {
            data,
            prefix: PhantomData,
        }
    }
}

impl<T> TryFrom<Box<[T]>> for Prefixed<u8, Box<[T]>> {
    type Error = Box<[T]>;

    fn try_from(data: Box<[T]>) -> Result<Self, Self::Error> {
        if data.len() > usize::from(u8::MAX) {
            return Err(data);
        }

        Ok(Self {
            data,
            prefix: PhantomData,
        })
    }
}

impl<T> TryFrom<Box<[T]>> for Prefixed<u16, Box<[T]>> {
    type Error = Box<[T]>;

    fn try_from(data: Box<[T]>) -> Result<Self, Self::Error> {
        if data.len() > usize::from(u16::MAX) {
            return Err(data);
        }

        Ok(Self {
            data,
            prefix: PhantomData,
        })
    }
}
