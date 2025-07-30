//! Tests for the `FromLeStream` and `ToLeStream` traits on `MacAddr8`.

#![cfg(all(test, feature = "macaddr"))]

use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;

const LE_BYTES: [u8; 8] = [0xD0, 0x2F, 0xAF, 0xFE, 0xFF, 0xEA, 0x35, 0x90];
const MAC_ADDRESS: MacAddr8 = MacAddr8::new(0x90, 0x35, 0xEA, 0xFF, 0xFE, 0xAF, 0x2F, 0xD0);

#[test]
fn test_mac_address_from_le_stream() {
    let mac_address = MacAddr8::from_le_stream_exact(LE_BYTES.into_iter())
        .expect("Could not create MAC address from byte stream.");
    assert_eq!(mac_address, MAC_ADDRESS);
}

#[test]
fn test_mac_address_to_le_stream() {
    let le_bytes: Vec<u8> = MAC_ADDRESS.to_le_stream().collect();
    assert_eq!(le_bytes, LE_BYTES);
}
