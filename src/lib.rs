//! A library for reading and writing data in little-endian byte order.
#![cfg_attr(not(feature = "std"), no_std)]

pub use consume::Consume;
pub use error::{Error, Result};
pub use from_le_stream::FromLeStream;
pub use from_le_stream_tagged::FromLeStreamTagged;
pub use prefixed::Prefixed;
#[cfg(feature = "heapless")]
pub use prefixed::{ByteSizedVec, WordSizedVec};
pub use to_le_stream::ToLeStream;

mod consume;
mod error;
mod from_le_stream;
mod from_le_stream_tagged;
mod prefixed;
mod to_le_stream;

/// Re-export the `FromLeBytes` and `ToLeBytes` derive macros.
#[cfg(feature = "derive")]
pub mod derive {
    pub use le_stream_derive::{FromLeStream, FromLeStreamTagged, ToLeStream};
}
