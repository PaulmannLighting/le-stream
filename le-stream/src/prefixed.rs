use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "heapless")]
pub use heapless::ByteSizedVec;

mod heapless;
mod std;

/// A wrapper type that adds a size prefix to the data it contains.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Prefixed<P, D> {
    data: D,
    prefix: PhantomData<P>,
}

impl<P, D> Prefixed<P, D> {
    /// Extract the data.
    pub fn into_data(self) -> D {
        self.data
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
    D: Deref,
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
