//! Tests for primitive `FromLeStream` and `ToLeStream` implementations.

#![cfg(test)]

use le_stream::{Error, FromLeStream, ToLeStream};

fn assert_round_trip<T>(value: T, expected: &[u8])
where
    T: Clone + core::fmt::Debug + PartialEq + FromLeStream + ToLeStream,
{
    assert_eq!(value.clone().to_le_stream().collect::<Vec<_>>(), expected);
    assert_eq!(T::from_le_stream_exact(expected.iter().copied()), Ok(value));
}

#[test]
fn unsigned_integers_are_little_endian() {
    assert_round_trip(0x12_u8, &[0x12]);
    assert_round_trip(0x1234_u16, &[0x34, 0x12]);
    assert_round_trip(0x1234_5678_u32, &[0x78, 0x56, 0x34, 0x12]);
    assert_round_trip(
        0x1234_5678_9ABC_DEF0_u64,
        &[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12],
    );
    assert_round_trip(
        0x1234_5678_9ABC_DEF0_0123_4567_89AB_CDEF_u128,
        &[
            0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56,
            0x34, 0x12,
        ],
    );
}

#[test]
fn signed_integers_are_little_endian() {
    assert_round_trip(-0x12_i8, &[0xEE]);
    assert_round_trip(-0x1234_i16, &[0xCC, 0xED]);
    assert_round_trip(-0x1234_5678_i32, &[0x88, 0xA9, 0xCB, 0xED]);
    assert_round_trip(
        -0x1234_5678_9ABC_DEF_i64,
        &[0x11, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE],
    );
}

#[test]
fn pointer_sized_integers_round_trip() {
    let usize_value = 0x1234_usize;
    let isize_value = -0x1234_isize;

    assert_round_trip(usize_value, &usize_value.to_le_bytes());
    assert_round_trip(isize_value, &isize_value.to_le_bytes());
}

#[test]
fn floats_are_little_endian() {
    let f32_value = 123.25_f32;
    let f64_value = -123.25_f64;

    assert_eq!(
        f32_value.to_le_stream().collect::<Vec<_>>(),
        f32_value.to_le_bytes()
    );
    assert_eq!(
        f32::from_le_stream_exact(f32_value.to_le_bytes().into_iter()).map(f32::to_bits),
        Ok(f32_value.to_bits())
    );
    assert_eq!(
        f64_value.to_le_stream().collect::<Vec<_>>(),
        f64_value.to_le_bytes()
    );
    assert_eq!(
        f64::from_le_stream_exact(f64_value.to_le_bytes().into_iter()).map(f64::to_bits),
        Ok(f64_value.to_bits())
    );
}

#[test]
fn bool_unit_array_option_and_ranges_round_trip() {
    assert_round_trip(false, &[0x00]);
    assert_round_trip(true, &[0x01]);
    assert_eq!(bool::from_le_stream_exact([0xFF].into_iter()), Ok(true));
    assert_eq!(().to_le_stream().collect::<Vec<_>>(), []);
    assert_eq!(<()>::from_le_stream_exact([].into_iter()), Ok(()));

    assert_round_trip([0x1234_u16, 0x5678], &[0x34, 0x12, 0x78, 0x56]);
    assert_round_trip(Some(0x1234_u16), &[0x34, 0x12]);
    assert_eq!(Option::<u16>::None.to_le_stream().collect::<Vec<_>>(), []);
    assert_eq!(Option::<u16>::from_le_stream([].into_iter()), Some(None));
    assert_round_trip(0x12_u8..0x34, &[0x12, 0x34]);
    assert_round_trip(0x1234_u16..=0x5678, &[0x34, 0x12, 0x78, 0x56]);
}

#[test]
fn exact_deserialization_reports_short_and_trailing_streams() {
    assert_eq!(
        u16::from_le_stream_exact([0x34].into_iter()),
        Err(Error::UnexpectedEndOfStream)
    );
    assert_eq!(
        u8::from_le_stream_exact([0x12, 0x34].into_iter()),
        Err(Error::StreamNotExhausted {
            instance: 0x12,
            next_byte: 0x34,
        })
    );
}
