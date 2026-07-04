//! A library for reading and writing data in little-endian byte order.

#![no_std]

mod consume;
mod de;
mod error;
mod from_le_stream;
mod prefixed;
mod ser;
mod serde_error;
mod stream;
mod to_le_stream;
mod try_from_le_stream;
mod writer;

pub use consume::Consume;
pub use de::{Deserializer, from_iter, from_iter_exact, from_slice, from_slice_exact};
pub use error::{Error, Result};
pub use from_le_stream::FromLeStream;
#[cfg(feature = "derive")]
pub use le_stream_derive::{FromLeStream, ToLeStream};
pub use prefixed::Prefixed;
#[cfg(feature = "alloc")]
pub use ser::to_vec;
pub use ser::{Serializer, to_slice, to_writer};
pub use serde_error::{SerdeError, SerdeResult};
pub use stream::{LeStream, LeStreamIterator};
pub use to_le_stream::ToLeStream;
pub use try_from_le_stream::TryFromLeStream;
pub use writer::ByteWriter;
