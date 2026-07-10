#![cfg(feature = "bytes")]

//! Deserialization for unprefixed raw `bytes` buffers.
//!
//! `Bytes` and `BytesMut` read every remaining byte from the stream. They are
//! byte buffers rather than typed collections, so their representation is the
//! raw byte sequence itself and does not include a length prefix.

use bytes::{Bytes, BytesMut};

use crate::FromLeStream;

impl FromLeStream for Bytes {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Some(bytes.collect())
    }
}

impl FromLeStream for BytesMut {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Some(bytes.collect())
    }
}
