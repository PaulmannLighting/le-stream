# le-stream

`le-stream` converts Rust values to and from little-endian byte streams.

The crate is `no_std` by default and works with iterators instead of requiring a
buffer. It provides implementations for primitive types, arrays, options,
ranges, and additional feature-gated types.

## Byte format

- Numeric primitives use their standard little-endian byte representation.
- `bool` is encoded as one byte: `0` for `false`, `1` for `true`.
- Arrays and derived structs concatenate their elements or fields in order.
- `Option<T>` has no tag: `None` writes no bytes, `Some(value)` writes `value`.
- Derived enums require `#[repr(T)]` and explicit discriminants. The
  discriminant is written as `T`, followed by associated data.

## Collections

The `alloc` collection implementations do not add a size prefix by default.
`Vec<T>` and `Box<[T]>` deserialize by consuming as many complete `T` values as
the byte stream can provide. Use `Prefixed<P, D>` when a standard allocated
collection needs an explicit length in the stream.

The `bytes` implementations treat `bytes::Bytes` and `bytes::BytesMut` as raw
byte buffers. They serialize exactly the contained bytes and deserialize by
consuming the remaining stream. No length prefix is added unless they are
wrapped in `Prefixed<P, D>`, which serializes a byte count of type `P` followed
by exactly that many raw bytes.

The `heapless` collection implementations are length-prefixed. A
`heapless::Vec<T, N, LenT>` serializes as a little-endian `LenT` element count
followed by exactly that many serialized `T` values. A
`heapless::String<N, LenT>` serializes as a `LenT` byte count followed by that
many UTF-8 bytes. During deserialization the prefix is read first, and the
implementation then attempts to consume exactly the number of elements or bytes
specified by that prefix. This is possible because the heapless type carries a
fixed capacity limit.

## Examples

```rust
use le_stream::{FromLeStream, ToLeStream};

let bytes: Vec<_> = 0x1234_u16.to_le_stream().collect();

assert_eq!(bytes, [0x34, 0x12]);
assert_eq!(u16::from_le_stream_exact(bytes.into_iter()), Ok(0x1234));
```

With `features = ["derive"]`:

```rust
use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct Header {
    command: u8,
    sequence: u16,
}

let header = Header {
    command: 0xA5,
    sequence: 0x1234,
};

let bytes: Vec<_> = header.clone().to_le_stream().collect();
assert_eq!(bytes, [0xA5, 0x34, 0x12]);
assert_eq!(Header::from_le_stream_exact(bytes.into_iter()), Ok(header));
```

## Feature flags

- `derive`: re-export the derive macros.
- `alloc`: support unprefixed `Vec`, `Box<T>`, `Box<[T]>`, and explicitly
  prefixed `Prefixed<P, Box<[T]>>`.
- `heapless`: support length-prefixed `heapless::Vec<T, N, LenT>` and
  `heapless::String<N, LenT>`.
- `bytes`: support unprefixed and explicitly prefixed raw `bytes::Bytes` and
  `bytes::BytesMut` buffers.
- `macaddr`: support `MacAddr6` and `MacAddr8` in little-endian byte order.
- `intx`: support selected `intx` integer types.
