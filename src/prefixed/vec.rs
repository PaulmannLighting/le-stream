use std::fmt::Debug;

use crate::{FromLeStream, Prefixed};

impl<P, T> Prefixed<P, Vec<T>> {
    /// Return a slice of the data contained in the vector.
    pub const fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<P, T> Prefixed<P, Vec<T>>
where
    P: TryFrom<usize>,
{
    /// Create a new `WithSizePrefix` from a `Vec` if the size is valid.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of the vector cannot be converted to `P`.
    pub fn try_new(data: Vec<T>) -> Result<Self, P::Error> {
        P::try_from(data.len()).map(|prefix| Self { prefix, data })
    }
}

impl<P, T> Prefixed<P, Vec<T>>
where
    P: TryFrom<usize>,
    P::Error: Debug,
{
    /// Create a new `WithSizePrefix` with the given data.
    ///
    /// # Panics
    ///
    /// Panics if the length of the vector cannot be converted to `P`.
    #[must_use]
    pub fn new(data: Vec<T>) -> Self {
        Self::try_new(data).expect("Size too lage. This is a bug.")
    }
}

impl<P, T> FromLeStream for Prefixed<P, Vec<T>>
where
    P: Copy + FromLeStream + Into<usize>,
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let prefix = P::from_le_stream(&mut bytes)?;
        let size: usize = prefix.into();

        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?);
        }

        Some(Self { prefix, data })
    }
}

impl<P, T> TryFrom<Vec<T>> for Prefixed<P, Vec<T>>
where
    P: TryFrom<usize>,
{
    type Error = P::Error;

    fn try_from(data: Vec<T>) -> Result<Self, Self::Error> {
        P::try_from(data.len()).map(|prefix| Self { prefix, data })
    }
}
