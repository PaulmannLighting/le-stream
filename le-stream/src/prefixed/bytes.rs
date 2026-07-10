#![cfg(feature = "bytes")]

//! Length-prefixed raw `bytes` buffers.
//!
//! `Prefixed<P, Bytes>` and `Prefixed<P, BytesMut>` write a little-endian byte
//! count of type `P`, followed by exactly that many raw bytes.

use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use bytes::buf::IntoIter;
use bytes::{Bytes, BytesMut};

use crate::{FromLeStream, Prefixed, ToLeStream};

impl<P> FromLeStream for Prefixed<P, Bytes>
where
    P: Into<usize> + FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = P::from_le_stream(&mut bytes)?.into();
        let data: Bytes = bytes.by_ref().take(size).collect();

        if data.len() != size {
            return None;
        }

        Some(Self {
            data,
            prefix: PhantomData,
        })
    }
}

impl<P> FromLeStream for Prefixed<P, BytesMut>
where
    P: Into<usize> + FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = P::from_le_stream(&mut bytes)?.into();
        let data: BytesMut = bytes.by_ref().take(size).collect();

        if data.len() != size {
            return None;
        }

        Some(Self {
            data,
            prefix: PhantomData,
        })
    }
}

impl<P> ToLeStream for Prefixed<P, Bytes>
where
    P: TryFrom<usize, Error: Debug> + ToLeStream,
{
    type Iter = Chain<<P as ToLeStream>::Iter, IntoIter<Bytes>>;

    fn to_le_stream(self) -> Self::Iter {
        <P as TryFrom<usize>>::try_from(self.data.len())
            .expect("amount of bytes should fit into prefix")
            .to_le_stream()
            .chain(self.data)
    }
}

impl<P> ToLeStream for Prefixed<P, BytesMut>
where
    P: TryFrom<usize, Error: Debug> + ToLeStream,
{
    type Iter = Chain<<P as ToLeStream>::Iter, IntoIter<BytesMut>>;

    fn to_le_stream(self) -> Self::Iter {
        <P as TryFrom<usize>>::try_from(self.data.len())
            .expect("amount of bytes should fit into prefix")
            .to_le_stream()
            .chain(self.data)
    }
}

impl From<Bytes> for Prefixed<usize, Bytes> {
    fn from(data: Bytes) -> Self {
        Self {
            data,
            prefix: PhantomData,
        }
    }
}

impl From<BytesMut> for Prefixed<usize, BytesMut> {
    fn from(data: BytesMut) -> Self {
        Self {
            data,
            prefix: PhantomData,
        }
    }
}

impl TryFrom<Bytes> for Prefixed<u8, Bytes> {
    type Error = Bytes;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        if data.len() > usize::from(u8::MAX) {
            return Err(data);
        }

        Ok(Self {
            data,
            prefix: PhantomData,
        })
    }
}

impl TryFrom<BytesMut> for Prefixed<u8, BytesMut> {
    type Error = BytesMut;

    fn try_from(data: BytesMut) -> Result<Self, Self::Error> {
        if data.len() > usize::from(u8::MAX) {
            return Err(data);
        }

        Ok(Self {
            data,
            prefix: PhantomData,
        })
    }
}

impl TryFrom<Bytes> for Prefixed<u16, Bytes> {
    type Error = Bytes;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        if data.len() > usize::from(u16::MAX) {
            return Err(data);
        }

        Ok(Self {
            data,
            prefix: PhantomData,
        })
    }
}

impl TryFrom<BytesMut> for Prefixed<u16, BytesMut> {
    type Error = BytesMut;

    fn try_from(data: BytesMut) -> Result<Self, Self::Error> {
        if data.len() > usize::from(u16::MAX) {
            return Err(data);
        }

        Ok(Self {
            data,
            prefix: PhantomData,
        })
    }
}
