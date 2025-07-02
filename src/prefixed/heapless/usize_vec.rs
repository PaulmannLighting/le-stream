use crate::{FromLeStream, Prefixed};

impl<const SIZE: usize, T> Prefixed<usize, heapless::Vec<T, SIZE>> {
    /// Create a new prefixed vec from a heapless Vec.
    pub fn new(data: heapless::Vec<T, SIZE>) -> Self {
        Self {
            prefix: data.len(),
            data,
        }
    }
}

impl<const SIZE: usize, T> From<heapless::Vec<T, SIZE>>
    for Prefixed<usize, heapless::Vec<T, SIZE>>
{
    fn from(vec: heapless::Vec<T, SIZE>) -> Self {
        Self::new(vec)
    }
}

impl<const SIZE: usize, T> FromLeStream for Prefixed<usize, heapless::Vec<T, SIZE>>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size = usize::from_le_stream(&mut bytes)?;

        let mut data = heapless::Vec::new();

        for _ in 0..size {
            data.push(T::from_le_stream(&mut bytes)?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(Self { prefix: size, data })
    }
}
