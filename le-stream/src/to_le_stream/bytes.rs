#![cfg(feature = "bytes")]

//! Serialization for unprefixed raw `bytes` buffers.
//!
//! `Bytes` and `BytesMut` serialize as their contained byte sequence without a
//! length prefix.

use bytes::buf::IntoIter;
use bytes::{Bytes, BytesMut};

use crate::ToLeStream;

impl ToLeStream for Bytes {
    type Iter = IntoIter<Self>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter()
    }
}

impl ToLeStream for BytesMut {
    type Iter = IntoIter<Self>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter()
    }
}
