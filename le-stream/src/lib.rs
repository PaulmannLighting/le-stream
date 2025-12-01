//! A library for reading and writing data in little-endian byte order.

#![no_std]
extern crate alloc;

pub use consume::Consume;
pub use error::{Error, Result};
pub use from_le_stream::FromLeStream;
pub use from_le_stream_tagged::FromLeStreamTagged;
pub use prefixed::Prefixed;
pub use to_le_stream::ToLeStream;
pub use try_from_le_stream::TryFromLeStream;

mod consume;
mod error;
mod from_le_stream;
mod from_le_stream_tagged;
mod prefixed;
mod to_le_stream;
mod try_from_le_stream;

#[cfg(feature = "derive")]
pub use le_stream_derive::{FromLeStream, FromLeStreamTagged, ToLeStream};
