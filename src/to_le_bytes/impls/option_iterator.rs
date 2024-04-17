use crate::ToLeBytes;

pub struct OptionIterator<T>(Option<<T as ToLeBytes>::Iter>)
where
    T: ToLeBytes;

impl<T> OptionIterator<T>
where
    T: ToLeBytes,
{
    pub fn new(option: Option<T>) -> Self {
        Self(option.map(ToLeBytes::to_le_bytes))
    }
}

impl<T> Iterator for OptionIterator<T>
where
    T: ToLeBytes,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(Iterator::next)
    }
}
