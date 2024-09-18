mod impls;

/// Convert an object to a stream of bytes with little endianness.
pub trait ToLeBytes
where
    Self::Iter: Iterator<Item = u8>,
{
    /// The byte iterator type.
    type Iter;

    /// Return an iterator of bytes with little endianness.
    fn to_le_bytes(self) -> Self::Iter;
}
