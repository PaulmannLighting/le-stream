#![cfg(test)]
use le_stream::{FromLeBytes, ToLeBytes};
use le_stream_derive::{FromLeBytes, ToLeBytes};

#[derive(Debug, FromLeBytes, ToLeBytes)]
struct MyStruct {
    flag: u8,
    num: u16,
    tail: u8,
}

#[test]
fn serialize() {
    let bytes = [0x42, 0x37, 0x13, 0xFF];
    let my_struct = MyStruct::from_le_bytes(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.tail, 0xff);
}

#[test]
fn deserialize() {
    let my_struct = MyStruct {
        flag: 0x42,
        num: 0x1337,
        tail: 0xff,
    };
    let bytes = vec![0x42, 0x37, 0x13, 0xFF];

    assert_eq!(my_struct.to_le_bytes().collect::<Vec<_>>(), bytes);
}
