//! Test for tagged stream deserialization using `FromLeStreamTagged`.

#![cfg(all(test, feature = "derive"))]

use le_stream::FromLeStream;

#[derive(Debug, PartialEq, FromLeStream)]
#[repr(u16)]
enum Tagged {
    A(u8) = 0x0001,
    B(u16) = 0x0002,
}

#[test]
fn test_from_le_stream_tagged_a() {
    let bytes = vec![0x01, 0x00, 0x02];
    let a = Tagged::from_le_stream_exact(bytes.into_iter());
    assert_eq!(a, Ok(Tagged::A(2)));
}

#[test]
fn test_from_le_stream_tagged_b() {
    let bytes = vec![0x02, 0x00, 0x03, 0x02];
    let a = Tagged::from_le_stream_exact(bytes.into_iter());
    assert_eq!(a, Ok(Tagged::B(0x0203)));
}
