//! A library for reading and writing data in little-endian byte order.
pub use error::{Error, Result};
pub use from_le_bytes::FromLeBytes;
pub use to_le_bytes::ToLeBytes;

mod error;
mod from_le_bytes;
mod to_le_bytes;

#[cfg(feature = "derive")]
pub mod derive {
    pub use le_stream_derive::{FromLeBytes, ToLeBytes};
}
