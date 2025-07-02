use crate::FromLeStream;
use crate::WithSizePrefix;

impl<P, T> WithSizePrefix<P, Vec<T>> {
    /// Return a slice of the data contained in the vector.
    pub const fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<P, T> FromLeStream for WithSizePrefix<P, Vec<T>>
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

impl<P, T> TryFrom<Vec<T>> for WithSizePrefix<P, Vec<T>>
where
    P: TryFrom<usize>,
{
    type Error = P::Error;

    fn try_from(data: Vec<T>) -> Result<Self, Self::Error> {
        P::try_from(data.len()).map(|prefix| Self { prefix, data })
    }
}
