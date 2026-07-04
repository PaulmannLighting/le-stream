# le-stream-derive

Derive macros for `le-stream`.

The crate provides `FromLeStream` and `ToLeStream` derives for structs and
represented enums.

## Structs

Struct fields are serialized in declaration order.

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

## Enums

Enums must specify `#[repr(T)]`, and every variant must have an explicit
discriminant. The discriminant is serialized as `T`, followed by any associated
data.

```rust
use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
#[repr(u16)]
enum Message {
    Ping = 0x0001,
    Data { sequence: u16, value: u8 } = 0x1002,
}

let message = Message::Data {
    sequence: 0x1234,
    value: 0xA5,
};

let bytes: Vec<_> = message.clone().to_le_stream().collect();
assert_eq!(bytes, [0x02, 0x10, 0x34, 0x12, 0xA5]);
assert_eq!(Message::from_le_stream_exact(bytes.into_iter()), Ok(message));
```
