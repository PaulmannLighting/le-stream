#![cfg(feature = "intx")]

use core::array::IntoIter;

use intx::{
    I24, I40, I48, I56, I72, I80, I88, I96, I104, I112, I120, U24, U40, U48, U56, U72, U80, U88,
    U96, U104, U112, U120,
};

use crate::ToLeStream;

macro_rules! implement_for {
    ($($typ:ty,)+) => {
        $(
            impl ToLeStream for $typ {
                type Iter = IntoIter<u8, { Self::BITS as usize / 8 }>;

                fn to_le_stream(self) -> Self::Iter {
                    self.to_le_bytes().into_iter()
                }
            }
        )+
    };
}

implement_for!(
    U24, U40, U48, U56, U72, U80, U88, U96, U104, U112, U120, I24, I40, I48, I56, I72, I80, I88,
    I96, I104, I112, I120,
);
