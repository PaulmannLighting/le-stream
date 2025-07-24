//! Tests for the `FromLeStream` and `ToLeStream` traits on structs.

#![cfg(all(test, feature = "derive", feature = "std"))]

use std::iter::empty;

use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::{Error, FromLeStream, Prefixed, ToLeStream};

const MY_STRUCT_BYTES: [u8; 49] = [
    0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xff, 0xaa, 0xbb, 0xcc, 0xdd, 0x34, 0x12, 0x56, 0x78,
    0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x00, 0x42,
    0x37, 0x13, 0x12, 0x42, 0x37, 0x13, 0x12, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xab,
    0xcd,
];

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
    size: usize,
    vec: Prefixed<usize, Box<[u8]>>,
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct Unit;

#[test]
fn deserialize_struct() {
    let my_struct = MyStruct::from_le_stream(MY_STRUCT_BYTES.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
    assert!(!my_struct.is_working);
    assert_eq!(my_struct.size, 0x1213_3742_1213_3742);
    assert_eq!(
        my_struct.vec.into_data(),
        vec![0xAB, 0xCD].into_boxed_slice()
    );
}

#[test]
fn deserialize_unit_struct() {
    let bytes: [u8; 0] = [];
    let unit =
        Unit::from_le_stream(bytes.into_iter()).expect("Could not create struct from byte stream.");
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
        size: 0x1213_3742_1213_3742,
        vec: vec![0xab, 0xcd].into_boxed_slice().into(),
    };

    assert_eq!(
        my_struct.to_le_stream().collect::<Vec<_>>(),
        MY_STRUCT_BYTES
    );
}

#[test]
fn serialize_unit_struct() {
    let unit = Unit;
    let bytes: Vec<_> = unit.to_le_stream().collect();
    assert_eq!(bytes, vec![]);
}

#[test]
fn deserialize_struct_exact() {
    let my_struct = MyStruct::from_le_stream_exact(MY_STRUCT_BYTES.into_iter())
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
    assert!(!my_struct.is_working);
    assert_eq!(my_struct.size, 0x1213_3742_1213_3742);
    assert_eq!(
        my_struct.vec.into_data(),
        vec![0xAB, 0xCD].into_boxed_slice()
    );
}

#[test]
fn deserialize_empty() {
    assert_eq!(MyStruct::from_le_stream(empty()), None);
}

#[test]
fn deserialize_empty_exact() {
    assert_eq!(
        MyStruct::from_le_stream_exact(empty()),
        Err(Error::UnexpectedEndOfStream)
    );
}

#[test]
fn deserialize_excess_exact() {
    const EXTRA_BYTE: u8 = 0xFE;
    const TAIL: u8 = 0xFF;
    let bytes = [
        0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xff, 0xaa, 0xbb, 0xcc, 0xdd, 0x34, 0x12, 0x56,
        0x78, 0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc,
        0x00, 0x42, 0x37, 0x13, 0x12, 0x42, 0x37, 0x13, 0x12, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xab, 0xcd, EXTRA_BYTE, TAIL,
    ];
    let mut iter = bytes.into_iter();
    assert_eq!(
        MyStruct::from_le_stream_exact(&mut iter),
        Err(Error::StreamNotExhausted(EXTRA_BYTE))
    );
    assert_eq!(iter.next(), Some(TAIL));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_serialization_primitive() {
    let i: i32 = 1234;
    let bytes: [u8; 4] = [0xd2, 0x04, 0x00, 0x00];

    for (byte, target) in i.to_le_stream().zip(bytes) {
        assert_eq!(byte, target);
    }
}

#[test]
fn test_deserialization_primitive() {
    let bytes: [u8; 4] = [0xd2, 0x04, 0x00, 0x00];
    let target: i32 = 1234;
    assert_eq!(i32::from_le_stream(&mut bytes.into_iter()), Some(target));
}

#[test]
fn deserialize_two_structs() {
    let mut stream = MY_STRUCT_BYTES.into_iter().chain(MY_STRUCT_BYTES);
    let one =
        MyStruct::from_le_stream(&mut stream).expect("Could not create struct from byte stream.");
    let other =
        MyStruct::from_le_stream_exact(stream).expect("Could not create struct from byte stream.");
    assert_eq!(one, other);
}

#[test]
fn deserialize_two_structs_array() {
    let stream = MY_STRUCT_BYTES.into_iter().chain(MY_STRUCT_BYTES);
    let structs =
        <[MyStruct; 2]>::from_le_stream(stream).expect("Could not create struct from byte stream.");
    assert_eq!(structs[0], structs[1]);
}

#[test]
fn deserialize_from_slice() {
    let my_struct = MyStruct::from_le_slice(&MY_STRUCT_BYTES)
        .expect("Could not create struct from byte stream.");

    assert_eq!(my_struct.flag, 0x42);
    assert_eq!(my_struct.num, 0x1337);
    assert_eq!(my_struct.array, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(my_struct.tail, 0xff);
    assert_eq!(my_struct.array_u16, [0xBBAA, 0xDDCC]);
    assert!(!my_struct.is_working);
    assert_eq!(my_struct.size, 0x1213_3742_1213_3742);
    assert_eq!(
        my_struct.vec.into_data(),
        vec![0xAB, 0xCD].into_boxed_slice()
    );
}
