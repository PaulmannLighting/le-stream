#![cfg(feature = "intx")]

use intx::{
    I24, I40, I48, I56, I72, I80, I88, I96, I104, I112, I120, U24, U40, U48, U56, U72, U80, U88,
    U96, U104, U112, U120,
};

use crate::FromLeStream;

macro_rules! implement_for {
    ($($typ:ty,)+) => {
        $(
            impl FromLeStream for $typ {
                fn from_le_stream<T>(bytes: T) -> Option<Self>
                where
                    T: Iterator<Item = u8>,
                {
                    <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
                }
            }
        )+
    };
}

implement_for!(
    U24, U40, U48, U56, U72, U80, U88, U96, U104, U112, U120, I24, I40, I48, I56, I72, I80, I88,
    I96, I104, I112, I120,
);
