# le-stream

`le-stream` is a small Rust workspace for encoding and decoding values as
little-endian byte streams.

The workspace contains two crates:

- `le-stream`: the `no_std` runtime traits and implementations.
- `le-stream-derive`: derive macros for structs and represented enums.

## Format

Primitive integer and floating-point values use their standard little-endian byte
representation. Compound values are encoded by concatenating their fields or
elements in declaration order.

`Option<T>` is compact: `None` writes no bytes, and `Some(value)` writes only
`value`. This is useful for optional trailing data, but it also means `Option<T>`
does not carry an explicit presence tag.

Derived enums require `#[repr(T)]` and an explicit discriminant on every
variant. The discriminant is serialized as `T`, followed by the variant's
associated data.

## Collections

Allocated collection types do not carry a size prefix by default. `Vec<T>` and
`Box<[T]>` deserialize by consuming as many complete `T` values as possible from
the stream. If a standard allocated collection needs an explicit length, wrap it
in `Prefixed<P, D>`.

The `bytes` feature adds support for `bytes::Bytes` and `bytes::BytesMut` as
raw byte buffers. They serialize exactly their contained bytes and deserialize
by consuming the remaining stream. No length prefix is added unless they are
wrapped in `Prefixed<P, D>`, which serializes a byte count of type `P` followed
by exactly that many raw bytes.

Heapless collection types are length-prefixed. `heapless::Vec<T, N, LenT>` uses
a little-endian `LenT` element count, and `heapless::String<N, LenT>` uses a
little-endian `LenT` byte count. Deserialization reads that prefix first and
then attempts to consume exactly the number of elements or bytes it describes.
This format is suitable for heapless collections because their types include a
fixed capacity limit.

## Example

```rust
use le_stream::{FromLeStream, ToLeStream};

let value = 0x1234_u16;
let bytes: Vec<_> = value.to_le_stream().collect();

assert_eq!(bytes, [0x34, 0x12]);
assert_eq!(u16::from_le_stream_exact(bytes.into_iter()), Ok(value));
```

With the `derive` feature:

```rust
use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct Packet {
    command: u8,
    sequence: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
#[repr(u8)]
enum Message {
    Ping = 1,
    Data(Packet) = 2,
}

let message = Message::Data(Packet {
    command: 0xA5,
    sequence: 0x1234,
});

let bytes: Vec<_> = message.clone().to_le_stream().collect();
assert_eq!(bytes, [2, 0xA5, 0x34, 0x12]);
assert_eq!(Message::from_le_stream_exact(bytes.into_iter()), Ok(message));
```

## Features

- `derive`: re-export `FromLeStream` and `ToLeStream` derive macros from
  `le-stream-derive`.
- `alloc`: implementations for unprefixed `Vec`, `Box<T>`, `Box<[T]>`, and
  explicitly prefixed boxed slices.
- `heapless`: implementations for length-prefixed `heapless::Vec<T, N, LenT>`
  and `heapless::String<N, LenT>`.
- `bytes`: implementations for unprefixed and explicitly prefixed raw
  `bytes::Bytes` and `bytes::BytesMut` buffers.
- `macaddr`: implementations for `macaddr::MacAddr6` and `macaddr::MacAddr8`.
- `intx`: implementations for selected `intx` integer types.

## Development

Use the same checks that CI expects:

```sh
cargo +nightly fmt --check
cargo clippy --all-features
cargo test --all-features
cargo vet
```
