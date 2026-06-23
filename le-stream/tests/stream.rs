//! Test the `LeStream` trait.

#![cfg(test)]

use le_stream::LeStream;

const BYTES: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

#[test]
fn test_stream_u8() {
    let bytes: Vec<u8> = BYTES.into_iter().le_stream().collect();
    assert_eq!(bytes, BYTES);
}

#[test]
fn test_stream_u16() {
    let bytes: Vec<u16> = BYTES.into_iter().le_stream().collect();
    assert_eq!(bytes, [0x0201, 0x0403, 0x0605, 0x0807]);
}
