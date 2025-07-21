use crate::ToLeStream;

pub struct OptionIterator<T>(Option<T::Iter>)
where
    T: ToLeStream;

impl<T> OptionIterator<T>
where
    T: ToLeStream,
{
    pub fn new(option: Option<T>) -> Self {
        Self(option.map(ToLeStream::to_le_stream))
    }
}

impl<T> Iterator for OptionIterator<T>
where
    T: ToLeStream,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(Iterator::next)
    }
}
