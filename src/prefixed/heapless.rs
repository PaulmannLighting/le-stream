use crate::Prefixed;

pub use byte_sized_vec::ByteSizedVec;
pub use word_sized_vec::WordSizedVec;

mod byte_sized_vec;
mod usize_vec;
mod word_sized_vec;

impl<const SIZE: usize, P, T> Prefixed<P, heapless::Vec<T, SIZE>> {
    /// Return a slice of the data contained in the vector.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}
