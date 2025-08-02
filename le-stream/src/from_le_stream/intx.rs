#![cfg(feature = "intx")]

use intx::{
    I24, I40, I48, I56, I72, I80, I88, I96, I104, I112, I120, U24, U40, U48, U56, U72, U80, U88,
    U96, U104, U112, U120,
};

use crate::FromLeStream;

impl FromLeStream for U24 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U40 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U48 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U56 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U72 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U80 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U88 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U96 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U104 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U112 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for U120 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I24 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I40 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I48 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I56 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I72 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I80 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I88 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I96 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I104 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I112 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for I120 {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}
