#![cfg(all(test, feature = "macaddr"))]

use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;

const MAC_ADDRESS: [u8; 8] = [0x90, 0x35, 0xEA, 0xFF, 0xFE, 0xAF, 0x2F, 0xD0];

#[test]
fn test_mac_address_from_le_stream() {
    let mac_address = MacAddr8::from_le_stream_exact(MAC_ADDRESS.into_iter().rev())
        .expect("Could not create MAC address from byte stream.");
    assert_eq!(mac_address, MAC_ADDRESS.into());
    assert_eq!(mac_address.to_string(), "90:35:EA:FF:FE:AF:2F:D0");
}

#[test]
fn test_mac_address_to_le_stream() {
    let mac_address = MacAddr8::from_le_stream_exact(MAC_ADDRESS.into_iter().rev())
        .expect("Could not create MAC address from byte stream.");
    let le_bytes: Vec<u8> = mac_address.to_le_stream().collect();
    let mut target = MAC_ADDRESS;
    target.reverse();
    assert_eq!(le_bytes, target);
    assert_eq!(mac_address.to_string(), "90:35:EA:FF:FE:AF:2F:D0");
}
