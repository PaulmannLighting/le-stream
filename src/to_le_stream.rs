mod core;
mod heapless;
mod intx;
mod macaddr;
mod std;

/// Convert an object to a stream of bytes with little endianness.
pub trait ToLeStream
where
    Self::Iter: Iterator<Item = u8>,
{
    /// The byte iterator type.
    type Iter;

    /// Return an iterator of bytes with little endianness.
    fn to_le_stream(self) -> Self::Iter;
}
