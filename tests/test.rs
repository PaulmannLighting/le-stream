#![cfg(test)]
use le_stream::{FromLeBytes, ToLeBytes};
use le_stream_derive::{FromLeBytes, ToLeBytes};

#[derive(Debug, FromLeBytes, ToLeBytes)]
struct MyStruct {
    flag: u8,
    num: u16,
    array: [u8; 4],
    tail: u8,
    array_u16: [u16; 2],
}

#[test]
fn serialize() {
    let bytes = [
        0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD,
    ];
    let my_struct = MyStruct::from_le_bytes(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
}

#[test]
fn deserialize() {
    let my_struct = MyStruct {
        flag: 0x42,
        num: 0x1337,
        array: [0x12, 0x34, 0x56, 0x78],
        tail: 0xff,
        array_u16: [0xBBAA, 0xDDCC],
    };
    let bytes = vec![
        0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD,
    ];

    assert_eq!(my_struct.to_le_bytes().collect::<Vec<_>>(), bytes);
}
