//! Little-endian byte stream encoding and decoding.
//!
//! `le-stream` defines compact [`ToLeStream`] and [`FromLeStream`] traits for
//! types that are represented as a sequence of little-endian bytes. The traits
//! work directly on iterators, which keeps the crate usable in `no_std`
//! environments and avoids requiring an intermediate buffer.
//!
//! Primitive integer and floating-point types are encoded with their native
//! `to_le_bytes` representation. Arrays, ranges, options, and feature-gated
//! collection types are encoded by concatenating the representation of their
//! contents. `Option<T>` is intentionally compact: `None` emits no bytes and
//! `Some(value)` emits only `value`.
//!
//! # Collection Semantics
//!
//! Standard-library collection types behind the `alloc` feature are not
//! length-prefixed by default. `Vec<T>` and `Box<[T]>` deserialize by consuming
//! as many complete `T` values as the byte stream can provide. Use
//! [`Prefixed<P, D>`] when an allocated collection needs an explicit length in
//! the stream.
//!
//! `heapless::Vec<T, N, LenT>` and `heapless::String<N, LenT>` are different:
//! they always encode a length prefix of type `LenT`, followed by exactly that
//! many serialized elements or UTF-8 bytes. During deserialization the prefix is
//! read first, and the implementation then attempts to consume exactly the
//! number of elements or bytes described by that prefix. This works naturally
//! for `heapless` collections because their type carries a fixed capacity limit.
//!
//! The `derive` feature re-exports derive macros for structs and explicitly
//! represented enums. Derived enums require `#[repr(T)]` and explicit
//! discriminants on every variant; the discriminant is encoded as `T`, followed
//! by any associated data.
//!
//! # Examples
//!
//! ```
//! use le_stream::{FromLeStream, ToLeStream};
//!
//! let value = 0x1234_u16;
//! assert_eq!(value.to_le_stream().collect::<Vec<_>>(), [0x34, 0x12]);
//! assert_eq!(u16::from_le_stream([0x34, 0x12].into_iter()), Some(value));
//! ```
//!
//! With the `derive` feature enabled:
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! use le_stream::{FromLeStream, ToLeStream};
//!
//! #[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
//! struct Header {
//!     command: u8,
//!     sequence: u16,
//! }
//!
//! let header = Header {
//!     command: 0xA5,
//!     sequence: 0x1234,
//! };
//!
//! let bytes: Vec<_> = header.clone().to_le_stream().collect();
//! assert_eq!(bytes, [0xA5, 0x34, 0x12]);
//! assert_eq!(Header::from_le_stream_exact(bytes.into_iter()).unwrap(), header);
//! # }
//! ```

#![no_std]

pub use consume::Consume;
pub use error::{Error, Result};
pub use from_le_stream::FromLeStream;
pub use prefixed::Prefixed;
pub use stream::{LeStream, LeStreamIterator};
pub use to_le_stream::ToLeStream;
pub use try_from_le_stream::TryFromLeStream;

mod consume;
mod error;
mod from_le_stream;
mod prefixed;
mod stream;
mod to_le_stream;
mod try_from_le_stream;

#[cfg(feature = "derive")]
pub use le_stream_derive::{FromLeStream, ToLeStream};
