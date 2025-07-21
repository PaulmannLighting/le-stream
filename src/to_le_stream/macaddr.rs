#![cfg(feature = "macaddr")]

use core::array::IntoIter;
use core::iter::Rev;

use macaddr::{MacAddr6, MacAddr8};

use crate::ToLeStream;

impl ToLeStream for MacAddr6 {
    type Iter = Rev<IntoIter<u8, 6>>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_array().into_iter().rev()
    }
}

impl ToLeStream for MacAddr8 {
    type Iter = Rev<IntoIter<u8, 8>>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_array().into_iter().rev()
    }
}
