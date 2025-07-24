use core::fmt::Debug;
use core::iter::Chain;
use core::ops::{Deref, DerefMut};

use crate::ToLeStream;

#[cfg(feature = "heapless")]
mod heapless;
#[cfg(feature = "heapless")]
pub use heapless::{ByteSizedVec, WordSizedVec};

mod vec;

/// A wrapper type that adds a size prefix to the data it contains.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Prefixed<P, D> {
    prefix: P,
    data: D,
}

impl<P, D> Prefixed<P, D> {
    /// Extract the data.
    pub fn into_data(self) -> D {
        self.data
    }
}

impl<P, D> ToLeStream for Prefixed<P, D>
where
    P: ToLeStream,
    D: ToLeStream,
{
    type Iter = Chain<P::Iter, D::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        self.prefix.to_le_stream().chain(self.data.to_le_stream())
    }
}

impl<P, D, T> AsRef<T> for Prefixed<P, D>
where
    T: ?Sized,
    D: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.data.as_ref()
    }
}

impl<P, D, T> AsMut<T> for Prefixed<P, D>
where
    T: ?Sized,
    D: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.data.as_mut()
    }
}

impl<P, D> Deref for Prefixed<P, D>
where
    D: Deref<Target: ?Sized>,
{
    type Target = D::Target;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<P, D> DerefMut for Prefixed<P, D>
where
    D: DerefMut,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
