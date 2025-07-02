use std::fmt::Debug;
use std::iter::Chain;
use std::ops::{Deref, DerefMut};

use crate::ToLeStream;

#[cfg(feature = "heapless")]
pub use heapless::SizedHeaplessVec;

#[cfg(feature = "heapless")]
mod heapless;
mod vec;

/// A wrapper type that adds a size prefix to the data it contains.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithSizePrefix<P, D> {
    prefix: P,
    data: D,
}

impl<P, D> WithSizePrefix<P, D> {
    /// Extract the data.
    pub fn into_data(self) -> D {
        self.data
    }
}

impl<P, D> ToLeStream for WithSizePrefix<P, D>
where
    P: ToLeStream,
    D: ToLeStream,
{
    type Iter = Chain<P::Iter, D::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.prefix.to_le_stream().chain(self.data.to_le_stream())
    }
}

impl<P, D, T> AsRef<T> for WithSizePrefix<P, D>
where
    D: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.data.as_ref()
    }
}

impl<P, D, T> AsMut<T> for WithSizePrefix<P, D>
where
    D: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.data.as_mut()
    }
}

impl<P, D> Deref for WithSizePrefix<P, D>
where
    D: Deref,
{
    type Target = D::Target;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<P, D> DerefMut for WithSizePrefix<P, D>
where
    D: DerefMut,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
