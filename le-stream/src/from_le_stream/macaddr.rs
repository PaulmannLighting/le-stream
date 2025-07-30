#![cfg(feature = "macaddr")]

use macaddr::{MacAddr6, MacAddr8};

use crate::FromLeStream;

impl FromLeStream for MacAddr6 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; 6]>::from_le_stream(bytes).map(|[f, e, d, c, b, a]| Self::new(a, b, c, d, e, f))
    }
}

impl FromLeStream for MacAddr8 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; 8]>::from_le_stream(bytes)
            .map(|[h, g, f, e, d, c, b, a]| Self::new(a, b, c, d, e, f, g, h))
    }
}
