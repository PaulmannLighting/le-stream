#![cfg(test)]
#![cfg(feature = "derive")]

use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::{Error, FromLeStream, ToLeStream};
use std::iter::empty;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct SubStruct {
    num: u16,
    array: [u8; 4],
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct MyStruct {
    flag: u8,
    num: u16,
    array: [u8; 4],
    tail: u8,
    array_u16: [u16; 2],
    array_sub_struct: [SubStruct; 3],
    is_working: bool,
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct Unit;

#[test]
fn deserialize_struct() {
    let bytes = [
        66, 55, 19, 18, 52, 86, 120, 255, 170, 187, 204, 221, 52, 18, 86, 120, 154, 188, 52, 18,
        86, 120, 154, 188, 52, 18, 86, 120, 154, 188, 1,
    ];
    let my_struct = MyStruct::from_le_stream(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
    assert!(my_struct.is_working);
}

#[test]
fn deserialize_unit_struct() {
    let bytes: [u8; 0] = [];
    let unit = Unit::from_le_stream(&mut bytes.into_iter())
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
        array_sub_struct: [
            SubStruct {
                num: 0x1234,
                array: [0x56, 0x78, 0x9A, 0xBC],
            },
            SubStruct {
                num: 0x1234,
                array: [0x56, 0x78, 0x9A, 0xBC],
            },
            SubStruct {
                num: 0x1234,
                array: [0x56, 0x78, 0x9A, 0xBC],
            },
        ],
        is_working: false,
    };
    let bytes = vec![
        66, 55, 19, 18, 52, 86, 120, 255, 170, 187, 204, 221, 52, 18, 86, 120, 154, 188, 52, 18,
        86, 120, 154, 188, 52, 18, 86, 120, 154, 188, 0,
    ];

    assert_eq!(my_struct.to_le_stream().collect::<Vec<_>>(), bytes);
}

#[test]
fn serialize_unit_struct() {
    let unit = Unit;
    let bytes: Vec<_> = unit.to_le_stream().collect();
    assert_eq!(bytes, vec![]);
}

#[test]
fn deserialize_struct_exact() {
    let bytes = [
        66, 55, 19, 18, 52, 86, 120, 255, 170, 187, 204, 221, 52, 18, 86, 120, 154, 188, 52, 18,
        86, 120, 154, 188, 52, 18, 86, 120, 154, 188, 0,
    ];
    let my_struct = MyStruct::from_le_stream_exact(&mut bytes.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
}

#[test]
fn deserialize_empty() {
    assert_eq!(
        MyStruct::from_le_stream(&mut empty()),
        Err(Error::UnexpectedEndOfStream)
    );
}

#[test]
fn deserialize_empty_exact() {
    assert_eq!(
        MyStruct::from_le_stream_exact(&mut empty()),
        Err(Error::UnexpectedEndOfStream)
    );
}

#[test]
fn deserialize_excess_exact() {
    const EXTRA_BYTE: u8 = 0xFE;
    const TAIL: u8 = 0xFF;
    let bytes = [
        66, 55, 19, 18, 52, 86, 120, 255, 170, 187, 204, 221, 52, 18, 86, 120, 154, 188, 52, 18,
        86, 120, 154, 188, 52, 18, 86, 120, 154, 188, 0, EXTRA_BYTE, TAIL,
    ];
    let mut iter = bytes.into_iter();
    assert_eq!(
        MyStruct::from_le_stream_exact(&mut iter),
        Err(Error::StreamNotExhausted(EXTRA_BYTE))
    );
    assert_eq!(iter.next(), Some(TAIL));
    assert_eq!(iter.next(), None);
}
