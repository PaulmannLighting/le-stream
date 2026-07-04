//! Tests for optional `intx` integer implementations.

#![cfg(all(test, feature = "intx"))]

use intx::{I24, U24};
use le_stream::{FromLeStream, ToLeStream};

#[test]
fn serialize_and_deserialize_u24() {
    let value = U24::from_le_bytes([0x34, 0x12, 0x00]);
    let bytes = [0x34, 0x12, 0x00];

    assert_eq!(value.to_le_stream().collect::<Vec<_>>(), bytes);
    assert_eq!(U24::from_le_stream_exact(bytes.into_iter()), Ok(value));
}

#[test]
fn serialize_and_deserialize_i24() {
    let value = I24::from_le_bytes([0xCC, 0xED, 0xFF]);
    let bytes = [0xCC, 0xED, 0xFF];

    assert_eq!(value.to_le_stream().collect::<Vec<_>>(), bytes);
    assert_eq!(I24::from_le_stream_exact(bytes.into_iter()), Ok(value));
}
