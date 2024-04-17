#![cfg(test)]
#![allow(clippy::unwrap_used)]
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};

#[derive(Debug, FromLeBytes, ToLeBytes)]
struct MyStruct {
    flag: u8,
    num: u16,
    array: [u8; 4],
    tail: u8,
    array_u16: [u16; 2],
    is_working: bool,
    heapless_vec: heapless::Vec<u8, { u8::MAX as usize }>,
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
struct Unit;

#[test]
fn deserialize_struct() {
    let bytes = [
        0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD, 0x01, 0x03, 0x01,
        0x02, 0x03,
    ];
    let my_struct = MyStruct::from_le_bytes(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
    assert!(my_struct.is_working);
    let heapless_vec: heapless::Vec<u8, { u8::MAX as usize }> =
        [0x01, 0x02, 0x03].as_slice().try_into().unwrap();
    assert_eq!(my_struct.heapless_vec, heapless_vec);
}

#[test]
fn deserialize_unit_struct() {
    let bytes: [u8; 0] = [];
    let unit = Unit::from_le_bytes(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");
    assert_eq!(unit, Unit);
}

#[test]
fn serialize_struct() {
    let my_struct = MyStruct {
        flag: 0x42,
        num: 0x1337,
        array: [0x12, 0x34, 0x56, 0x78],
        tail: 0xff,
        array_u16: [0xBBAA, 0xDDCC],
        is_working: false,
        heapless_vec: [0x01, 0x02, 0x03].as_slice().try_into().unwrap(),
    };
    let bytes = vec![
        0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD, 0x00, 0x03, 0x01,
        0x02, 0x03,
    ];

    assert_eq!(my_struct.to_le_bytes().collect::<Vec<_>>(), bytes);
}

#[test]
fn serialize_unit_struct() {
    let unit = Unit;
    let bytes: Vec<_> = unit.to_le_bytes().collect();
    assert_eq!(bytes, vec![]);
}

#[test]
fn deserialize_struct_exact() {
    let bytes = [
        0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD, 0x01, 0x03, 0x01,
        0x02, 0x03,
    ];
    let my_struct = MyStruct::from_le_bytes_exact(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
    assert!(my_struct.is_working);
    let heapless_vec: heapless::Vec<u8, { u8::MAX as usize }> =
        [0x01, 0x02, 0x03].as_slice().try_into().unwrap();
    assert_eq!(my_struct.heapless_vec, heapless_vec);
}
