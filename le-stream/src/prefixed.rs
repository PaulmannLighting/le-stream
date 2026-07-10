use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

mod alloc;
mod bytes;

/// A wrapper that serializes a collection with a little-endian length prefix.
///
/// `P` is the integer type used for the prefix and `D` is the wrapped data.
/// Implementations are provided for selected collection types behind feature
/// flags. For example, `Prefixed<usize, Box<[u8]>>` serializes as a `usize`
/// element count followed by the bytes in the boxed slice when the `alloc`
/// feature is enabled. `Prefixed<P, bytes::Bytes>` and
/// `Prefixed<P, bytes::BytesMut>` serialize a byte count followed by exactly
/// that many raw bytes when the `bytes` feature is enabled.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Prefixed<P, D> {
    data: D,
    prefix: PhantomData<P>,
}

impl<P, D> Prefixed<P, D> {
    /// Returns the wrapped data without its prefix marker.
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

impl<P, D> IntoIterator for Prefixed<P, D>
where
    D: IntoIterator,
{
    type Item = D::Item;
    type IntoIter = D::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
