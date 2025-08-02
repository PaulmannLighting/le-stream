#![cfg(feature = "intx")]

use core::array::IntoIter;

use intx::{
    I24, I40, I48, I56, I72, I80, I88, I96, I104, I112, I120, U24, U40, U48, U56, U72, U80, U88,
    U96, U104, U112, U120,
};

use crate::ToLeStream;

impl ToLeStream for U24 {
    type Iter = IntoIter<u8, 3>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U40 {
    type Iter = IntoIter<u8, 5>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U48 {
    type Iter = IntoIter<u8, 6>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U56 {
    type Iter = IntoIter<u8, 7>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U72 {
    type Iter = IntoIter<u8, 9>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U80 {
    type Iter = IntoIter<u8, 10>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U88 {
    type Iter = IntoIter<u8, 11>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U96 {
    type Iter = IntoIter<u8, 12>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}
impl ToLeStream for U104 {
    type Iter = IntoIter<u8, 13>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}
impl ToLeStream for U112 {
    type Iter = IntoIter<u8, 14>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for U120 {
    type Iter = IntoIter<u8, 15>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I24 {
    type Iter = IntoIter<u8, 3>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I40 {
    type Iter = IntoIter<u8, 5>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I48 {
    type Iter = IntoIter<u8, 6>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I56 {
    type Iter = IntoIter<u8, 7>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I72 {
    type Iter = IntoIter<u8, 9>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I80 {
    type Iter = IntoIter<u8, 10>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I88 {
    type Iter = IntoIter<u8, 11>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I96 {
    type Iter = IntoIter<u8, 12>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}
impl ToLeStream for I104 {
    type Iter = IntoIter<u8, 13>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}
impl ToLeStream for I112 {
    type Iter = IntoIter<u8, 14>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}

impl ToLeStream for I120 {
    type Iter = IntoIter<u8, 15>;

    fn to_le_stream(self) -> Self::Iter {
        self.to_le_bytes().into_iter()
    }
}
