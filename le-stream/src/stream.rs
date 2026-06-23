use core::marker::PhantomData;

use crate::FromLeStream;

/// Extension trait on iterators to convert them into streams of deserializable types.
pub trait LeStream: Sized {
    /// Convert an iterator of little endian bytes into a stream of deserializable types.
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

/// An iterator over a little endian byte stream.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeStreamIterator<T, I> {
    iterator: I,
    _type: PhantomData<T>,
}

impl<T, I> LeStreamIterator<T, I> {
    /// Consumes the iterator, returning the underlying byte stream.
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
