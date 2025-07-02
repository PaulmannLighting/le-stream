//! A library for reading and writing data in little-endian byte order.

pub use consume::Consume;
pub use error::{Error, Result};
pub use from_le_stream::FromLeStream;
pub use to_le_stream::ToLeStream;
pub use with_size_prefix::WithSizePrefix;

#[cfg(feature = "heapless")]
pub use with_size_prefix::SizedHeaplessVec;

mod consume;
mod error;
mod from_le_stream;
mod to_le_stream;
mod with_size_prefix;

/// Re-export the `FromLeBytes` and `ToLeBytes` derive macros.
#[cfg(feature = "derive")]
pub mod derive {
    pub use le_stream_derive::{FromLeStream, ToLeStream};
}
