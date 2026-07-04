mod alloc;
mod core;
mod heapless;
mod intx;
mod macaddr;

/// Converts a value into a stream of little-endian bytes.
///
/// The returned iterator yields the complete byte representation of `self`.
/// Primitive numeric implementations use their standard little-endian byte
/// order; compound implementations concatenate the byte stream of each field or
/// element in declaration order.
pub trait ToLeStream
where
    Self::Iter: Iterator<Item = u8>,
{
    /// Iterator returned by [`to_le_stream`](Self::to_le_stream).
    type Iter;

    /// Returns an iterator over the little-endian byte representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use le_stream::ToLeStream;
    ///
    /// assert_eq!(0x1234_u16.to_le_stream().collect::<Vec<_>>(), [0x34, 0x12]);
    /// ```
    fn to_le_stream(self) -> Self::Iter;
}
