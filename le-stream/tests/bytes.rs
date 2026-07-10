//! Tests for optional `bytes` buffer implementations.

#![cfg(all(test, feature = "bytes"))]

use bytes::{Bytes, BytesMut};
use le_stream::{Error, FromLeStream, Prefixed, ToLeStream};

const BYTES: [u8; 4] = [0x01, 0x02, 0xA5, 0xFF];
const PREFIXED_BYTES_U8: [u8; 5] = [0x04, 0x01, 0x02, 0xA5, 0xFF];
const PREFIXED_BYTES_U16: [u8; 6] = [0x04, 0x00, 0x01, 0x02, 0xA5, 0xFF];
const EXTRA_BYTE: u8 = 0x42;

#[test]
fn serialize_and_deserialize_bytes() {
    let value = Bytes::from_static(&BYTES);

    assert_eq!(value.clone().to_le_stream().collect::<Vec<_>>(), BYTES);
    assert_eq!(Bytes::from_le_stream_exact(BYTES.into_iter()), Ok(value));
}

#[test]
fn serialize_and_deserialize_bytes_mut() {
    let value = BytesMut::from(BYTES.as_slice());

    assert_eq!(value.clone().to_le_stream().collect::<Vec<_>>(), BYTES);
    assert_eq!(BytesMut::from_le_stream_exact(BYTES.into_iter()), Ok(value));
}

#[test]
fn deserialize_bytes_consumes_remaining_stream() {
    let mut stream = BYTES.into_iter();
    let value = Bytes::from_le_stream(&mut stream).expect("Could not create bytes from stream.");

    assert_eq!(value, Bytes::from_static(&BYTES));
    assert_eq!(stream.next(), None);
}

#[test]
fn deserialize_empty_bytes() {
    assert_eq!(
        Bytes::from_le_stream(core::iter::empty()),
        Some(Bytes::new())
    );
    assert_eq!(
        BytesMut::from_le_stream(core::iter::empty()),
        Some(BytesMut::new())
    );
}

#[test]
fn serialize_and_deserialize_prefixed_bytes() {
    let value = Prefixed::<u8, Bytes>::try_from(Bytes::from_static(&BYTES))
        .expect("Could not create prefixed bytes.");

    assert_eq!(
        value.clone().to_le_stream().collect::<Vec<_>>(),
        PREFIXED_BYTES_U8
    );
    assert_eq!(
        Prefixed::<u8, Bytes>::from_le_stream_exact(PREFIXED_BYTES_U8.into_iter()),
        Ok(value)
    );
}

#[test]
fn serialize_and_deserialize_prefixed_bytes_mut() {
    let value = Prefixed::<u16, BytesMut>::try_from(BytesMut::from(BYTES.as_slice()))
        .expect("Could not create prefixed bytes.");

    assert_eq!(
        value.clone().to_le_stream().collect::<Vec<_>>(),
        PREFIXED_BYTES_U16
    );
    assert_eq!(
        Prefixed::<u16, BytesMut>::from_le_stream_exact(PREFIXED_BYTES_U16.into_iter()),
        Ok(value)
    );
}

#[test]
fn prefixed_bytes_leave_trailing_stream() {
    let mut stream = PREFIXED_BYTES_U8.into_iter().chain([EXTRA_BYTE]);
    let value = Prefixed::<u8, Bytes>::from_le_stream(&mut stream)
        .expect("Could not create prefixed bytes.");

    assert_eq!(value.into_data(), Bytes::from_static(&BYTES));
    assert_eq!(stream.next(), Some(EXTRA_BYTE));
    assert_eq!(stream.next(), None);
}

#[test]
fn prefixed_bytes_reject_short_stream() {
    let bytes = [0x04, 0x01, 0x02, 0xA5];

    assert_eq!(
        Prefixed::<u8, Bytes>::from_le_stream(bytes.into_iter()),
        None
    );
    assert_eq!(
        Prefixed::<u8, BytesMut>::from_le_stream_exact(bytes.into_iter()),
        Err(Error::UnexpectedEndOfStream)
    );
}

#[test]
fn prefixed_bytes_reject_too_small_prefix_type() {
    let too_long_len = usize::from(u8::MAX) + 1;
    let data = Bytes::from(vec![0; too_long_len]);

    assert_eq!(Prefixed::<u8, Bytes>::try_from(data.clone()), Err(data));
}
