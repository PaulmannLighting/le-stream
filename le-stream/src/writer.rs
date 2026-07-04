#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{SerdeError, SerdeResult};

/// Output sink for Serde-based serialization.
pub trait ByteWriter {
    /// Writes all bytes into the sink.
    ///
    /// # Errors
    ///
    /// Returns an error if the sink cannot accept all bytes.
    fn write_all(&mut self, bytes: &[u8]) -> SerdeResult<()>;
}

impl ByteWriter for &mut [u8] {
    fn write_all(&mut self, bytes: &[u8]) -> SerdeResult<()> {
        if self.len() < bytes.len() {
            return Err(SerdeError::BufferTooSmall);
        }

        let current = core::mem::take(self);
        let (head, tail) = current.split_at_mut(bytes.len());
        head.copy_from_slice(bytes);
        *self = tail;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl ByteWriter for Vec<u8> {
    fn write_all(&mut self, bytes: &[u8]) -> SerdeResult<()> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl ByteWriter for &mut Vec<u8> {
    fn write_all(&mut self, bytes: &[u8]) -> SerdeResult<()> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}
