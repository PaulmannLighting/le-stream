mod impls;

pub trait ToLeBytes
where
    Self::Iter: Iterator<Item = u8>,
{
    type Iter;

    fn to_le_bytes(self) -> Self::Iter;
}
