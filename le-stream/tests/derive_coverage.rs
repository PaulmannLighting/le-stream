//! Coverage for derived structs and represented enums.

#![cfg(all(test, feature = "derive"))]
#![allow(missing_docs)]

use le_stream::{FromLeStream, ToLeStream};

const COMPLEX_BYTES: [u8; 17] = [
    0xCD, 0xAB, 0x01, 0x01, 0x02, 0x03, 0xFE, 0xFF, 0x34, 0x12, 0x03, 0x00, 0x10, 0x02, 0x10, 0xFE,
    0xCA,
];

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct Header {
    magic: u16,
    enabled: bool,
    modes: [u8; 3],
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct Payload {
    sample_count: u8,
    window: core::ops::RangeInclusive<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct ComplexPacket {
    header: Header,
    samples: [i16; 2],
    payload: Payload,
    checksum: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
struct OptionalTail {
    code: u8,
    tail: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
#[repr(u8)]
enum UnitMessage {
    Ping = 0x01,
    Pong = 0x02,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
#[repr(u16)]
enum TupleMessage {
    Small(u8, u16) = 0x1001,
    Large(u32) = 0x1002,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
#[repr(u32)]
enum NamedMessage {
    Data { enabled: bool, sequence: u16 } = 0x0102_0304,
    Done = 0x0506_0708,
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
#[repr(i16)]
enum SignedMessage {
    Negative(u8) = -2,
    Positive = 0x0102,
}

fn complex_packet() -> ComplexPacket {
    ComplexPacket {
        header: Header {
            magic: 0xABCD,
            enabled: true,
            modes: [0x01, 0x02, 0x03],
        },
        samples: [-2, 0x1234],
        payload: Payload {
            sample_count: 3,
            window: 0x1000..=0x1002,
        },
        checksum: Some(0xCAFE),
    }
}

#[test]
fn derived_complex_struct_serializes_in_field_order() {
    assert_eq!(
        complex_packet().to_le_stream().collect::<Vec<_>>(),
        COMPLEX_BYTES
    );
}

#[test]
fn derived_complex_struct_deserializes_in_field_order() {
    assert_eq!(
        ComplexPacket::from_le_stream_exact(COMPLEX_BYTES.into_iter()),
        Ok(complex_packet())
    );
}

#[test]
fn derived_struct_supports_absent_trailing_option() {
    let bytes = [0xA5];
    let value = OptionalTail {
        code: 0xA5,
        tail: None,
    };

    assert_eq!(
        OptionalTail::from_le_stream_exact(bytes.into_iter()),
        Ok(value)
    );
}

#[test]
fn repr_u8_unit_enum_round_trips() {
    assert_eq!(UnitMessage::Pong.to_le_stream().collect::<Vec<_>>(), [0x02]);
    assert_eq!(
        UnitMessage::from_le_stream_exact([0x01].into_iter()),
        Ok(UnitMessage::Ping)
    );
}

#[test]
fn repr_u16_tuple_enum_round_trips_with_associated_data() {
    let value = TupleMessage::Small(0xA5, 0x1234);
    let bytes = [0x01, 0x10, 0xA5, 0x34, 0x12];

    assert_eq!(value.clone().to_le_stream().collect::<Vec<_>>(), bytes);
    assert_eq!(
        TupleMessage::from_le_stream_exact(bytes.into_iter()),
        Ok(value)
    );
}

#[test]
fn repr_u32_named_enum_round_trips_with_associated_data() {
    let value = NamedMessage::Data {
        enabled: true,
        sequence: 0x1234,
    };
    let bytes = [0x04, 0x03, 0x02, 0x01, 0x01, 0x34, 0x12];

    assert_eq!(value.clone().to_le_stream().collect::<Vec<_>>(), bytes);
    assert_eq!(
        NamedMessage::from_le_stream_exact(bytes.into_iter()),
        Ok(value)
    );
}

#[test]
fn repr_i16_enum_round_trips_signed_discriminant() {
    let value = SignedMessage::Negative(0xA5);
    let bytes = [0xFE, 0xFF, 0xA5];

    assert_eq!(value.clone().to_le_stream().collect::<Vec<_>>(), bytes);
    assert_eq!(
        SignedMessage::from_le_stream_exact(bytes.into_iter()),
        Ok(value)
    );
}

#[test]
fn enum_deserialization_rejects_unknown_discriminants() {
    assert_eq!(UnitMessage::from_le_stream([0xFF].into_iter()), None);
}
