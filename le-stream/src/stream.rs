use core::marker::PhantomData;

use crate::FromLeStream;

/// Extension trait for reading typed values from an iterator of bytes.
pub trait LeStream: Sized {
    /// Converts an iterator of little-endian bytes into an iterator of `T`.
    ///
    /// Iteration stops when `T::from_le_stream` returns [`None`], usually
    /// because the underlying byte iterator is exhausted or ends with an
    /// incomplete value.
    fn le_stream<T>(self) -> LeStreamIterator<T, Self>;
}

impl<I> LeStream for I
where
    I: Iterator<Item = u8>,
{
    fn le_stream<T>(self) -> LeStreamIterator<T, Self> {
        LeStreamIterator {
            iterator: self,
            _type: PhantomData,
        }
    }
}

/// Iterator adapter returned by [`LeStream::le_stream`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeStreamIterator<T, I> {
    iterator: I,
    _type: PhantomData<T>,
}

impl<T, I> LeStreamIterator<T, I> {
    /// Consumes the adapter and returns the underlying byte iterator.
    #[must_use]
    pub fn into_inner(self) -> I {
        self.iterator
    }
}

impl<T, I> Iterator for LeStreamIterator<T, I>
where
    T: FromLeStream,
    I: Iterator<Item = u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::from_le_stream(&mut self.iterator)
    }
}
