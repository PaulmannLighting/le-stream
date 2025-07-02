use std::num::TryFromIntError;

use crate::FromLeStream;
use crate::WithSizePrefix;

use size::Size;

mod size;

/// A `heapless::Vec` with a size prefix.
pub type SizedHeaplessVec<T, const SIZE: usize> = WithSizePrefix<Size, heapless::Vec<T, SIZE>>;

impl<const SIZE: usize, T> WithSizePrefix<Size, heapless::Vec<T, SIZE>> {
    /// Return a slice of the data contained in the vector.
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<const SIZE: usize, T> FromLeStream for SizedHeaplessVec<T, SIZE>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let prefix = Size::parse::<SIZE>(&mut bytes)?;
        let Ok(size) = prefix.try_into() else {
            return None; // Ensure we can convert the prefix into usize
        };

        if size > SIZE {
            return None; // Ensure we do not exceed the fixed size
        }

        let mut data = heapless::Vec::<T, SIZE>::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(Self { prefix, data })
    }
}

impl<const SIZE: usize, T> TryFrom<&[T]> for SizedHeaplessVec<T, SIZE>
where
    T: Clone + FromLeStream,
{
    type Error = Option<TryFromIntError>;

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        heapless::Vec::<T, SIZE>::from_slice(slice)
            .map_err(|()| None)
            .and_then(|data| {
                Size::try_from(data.len())
                    .map_err(Some)
                    .map(|prefix| Self { prefix, data })
            })
    }
}
