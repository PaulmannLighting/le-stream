use crate::ToLeStream;

/// An iterator for an option of a serializable type.
pub struct OptionIterator<T>(Option<T>);

impl<T> OptionIterator<T> {
    /// Create a new `OptionIterator`.
    #[must_use]
    pub const fn new(option: Option<T>) -> Self {
        Self(option)
    }
}

impl<T> From<Option<T>> for OptionIterator<T::Iter>
where
    T: ToLeStream,
{
    fn from(option: Option<T>) -> Self {
        Self::new(option.map(ToLeStream::to_le_stream))
    }
}

impl<T> Iterator for OptionIterator<T>
where
    T: Iterator<Item = u8>,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(Iterator::next)
    }
}
