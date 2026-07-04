//! Tests for Serde-based little-endian serialization.

#![cfg(test)]

#[cfg(feature = "alloc")]
use le_stream::to_vec;
use le_stream::{SerdeError, from_slice, from_slice_exact, to_slice};
#[cfg(feature = "macaddr")]
use macaddr::{MacAddr6, MacAddr8};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(not(feature = "macaddr"))]
const MY_STRUCT_BYTES: [u8; 39] = [
    0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xff, 0xaa, 0xbb, 0xcc, 0xdd, 0x34, 0x12, 0x56, 0x78,
    0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x00, 0x42,
    0x37, 0x13, 0x12, 0x42, 0x37, 0x13, 0x12,
];

#[cfg(feature = "macaddr")]
const MY_STRUCT_BYTES: [u8; 53] = [
    0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xff, 0xaa, 0xbb, 0xcc, 0xdd, 0x34, 0x12, 0x56, 0x78,
    0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x12, 0x34,
    0x56, 0x78, 0x9a, 0xbc, 0xd0, 0x2f, 0xaf, 0xfe, 0xff, 0xea, 0x35, 0x90, 0x00, 0x42, 0x37, 0x13,
    0x12, 0x42, 0x37, 0x13, 0x12,
];

#[cfg(feature = "macaddr")]
const MAC_ADDRESS6: MacAddr6 = MacAddr6::new(0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12);

#[cfg(feature = "macaddr")]
const MAC_ADDRESS: MacAddr8 = MacAddr8::new(0x90, 0x35, 0xEA, 0xFF, 0xFE, 0xAF, 0x2F, 0xD0);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct SubStruct {
    num: u16,
    array: [u8; 4],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct MyStruct {
    flag: u8,
    num: u16,
    array: [u8; 4],
    tail: u8,
    array_u16: [u16; 2],
    array_sub_struct: [SubStruct; 3],
    #[cfg(feature = "macaddr")]
    mac_address6: MacAddr6,
    #[cfg(feature = "macaddr")]
    mac_address8: MacAddr8,
    is_working: bool,
    size: usize,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
enum Tagged {
    A(u8),
    B(u16),
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u16)]
enum ReprTagged {
    A(u8) = 0x1001,
    B(u16) = 0x1002,
}

#[derive(Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u16)]
enum ReprPrimitive {
    A = 0x1001,
    B = 0x1002,
}

fn my_struct() -> MyStruct {
    MyStruct {
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
        #[cfg(feature = "macaddr")]
        mac_address6: MAC_ADDRESS6,
        #[cfg(feature = "macaddr")]
        mac_address8: MAC_ADDRESS,
        is_working: false,
        size: 0x1213_3742_1213_3742,
    }
}

#[test]
#[cfg(feature = "alloc")]
fn serialize_primitive() {
    assert_eq!(to_vec(&1234_i32), Ok(vec![0xd2, 0x04, 0x00, 0x00]));
}

#[test]
fn deserialize_primitive() {
    assert_eq!(from_slice_exact::<i32>(&[0xd2, 0x04, 0x00, 0x00]), Ok(1234));
}

#[test]
#[cfg(feature = "alloc")]
fn serialize_struct() {
    assert_eq!(to_vec(&my_struct()), Ok(MY_STRUCT_BYTES.to_vec()));
}

#[test]
fn deserialize_struct() {
    assert_eq!(
        from_slice_exact::<MyStruct>(&MY_STRUCT_BYTES),
        Ok(my_struct())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn serialize_compact_option() {
    assert_eq!(to_vec(&Option::<u8>::None), Ok(vec![]));
    assert_eq!(to_vec(&Some(0xAB_u8)), Ok(vec![0xAB]));
}

#[test]
fn deserialize_compact_option() {
    assert_eq!(from_slice_exact::<Option<u8>>(&[]), Ok(None));
    assert_eq!(from_slice_exact::<Option<u8>>(&[0xAB]), Ok(Some(0xAB)));
}

#[test]
#[cfg(feature = "alloc")]
fn serialize_enum_variant_index() {
    assert_eq!(
        to_vec(&Tagged::A(2)),
        Ok(vec![0x00, 0x00, 0x00, 0x00, 0x02])
    );
    assert_eq!(
        to_vec(&Tagged::B(0x0203)),
        Ok(vec![0x01, 0x00, 0x00, 0x00, 0x03, 0x02])
    );
}

#[test]
fn deserialize_enum_variant_index() {
    assert_eq!(
        from_slice_exact::<Tagged>(&[0x00, 0x00, 0x00, 0x00, 0x02]),
        Ok(Tagged::A(2))
    );
    assert_eq!(
        from_slice_exact::<Tagged>(&[0x01, 0x00, 0x00, 0x00, 0x03, 0x02]),
        Ok(Tagged::B(0x0203))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn serialize_repr_u16_enum_variant_index() {
    assert_eq!(
        to_vec(&ReprTagged::A(2)),
        Ok(vec![0x00, 0x00, 0x00, 0x00, 0x02])
    );
    assert_eq!(
        to_vec(&ReprTagged::B(0x0203)),
        Ok(vec![0x01, 0x00, 0x00, 0x00, 0x03, 0x02])
    );
}

#[test]
fn deserialize_repr_u16_enum_variant_index() {
    assert_eq!(
        from_slice_exact::<ReprTagged>(&[0x00, 0x00, 0x00, 0x00, 0x02]),
        Ok(ReprTagged::A(2))
    );
    assert_eq!(
        from_slice_exact::<ReprTagged>(&[0x01, 0x00, 0x00, 0x00, 0x03, 0x02]),
        Ok(ReprTagged::B(0x0203))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn serialize_repr_u16_primitive_enum() {
    assert_eq!(to_vec(&ReprPrimitive::B), Ok(vec![0x02, 0x10]));
}

#[test]
fn deserialize_repr_u16_primitive_enum() {
    assert_eq!(
        from_slice_exact::<ReprPrimitive>(&[0x02, 0x10]),
        Ok(ReprPrimitive::B)
    );
}

#[test]
fn deserialize_excess_exact() {
    assert_eq!(
        from_slice_exact::<u8>(&[0xAB, 0xCD]),
        Err(SerdeError::StreamNotExhausted { next_byte: 0xCD })
    );
}

#[test]
fn serialize_to_slice() {
    let mut bytes = [0; 4];
    assert_eq!(to_slice(&1234_i32, &mut bytes), Ok(4));
    assert_eq!(bytes, [0xd2, 0x04, 0x00, 0x00]);
}

#[test]
fn deserialize_partial_slice() {
    assert_eq!(from_slice::<u8>(&[0xAB, 0xCD]), Ok(0xAB));
}
