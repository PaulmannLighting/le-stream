//! Test for tagged stream deserialization using `FromLeStreamTagged`.
//!
#![cfg(test)]

use le_stream::{FromLeStream, FromLeStreamTagged};

#[derive(Debug, PartialEq)]
#[repr(u16)]
enum Tagged {
    A(u8) = 0x0001,
    B(u16) = 0x0002,
}

impl FromLeStreamTagged for Tagged {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, mut bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0x0001 => Ok(u8::from_le_stream(&mut bytes).map(Self::A)),
            0x0002 => Ok(u16::from_le_stream(&mut bytes).map(Self::B)),
            other => Err(other),
        }
    }
}

#[test]
fn test_from_le_stream_tagged_a() {
    let bytes = vec![0x01, 0x00, 0x02];
    let a = Tagged::from_le_stream_exact(bytes.into_iter());
    assert_eq!(a, Ok(Tagged::A(2)));
}

#[test]
fn test_from_le_stream_tagged_b() {
    let bytes = vec![0x02, 0x00, 0x03, 0x02];
    let a = Tagged::from_le_stream_exact(bytes.into_iter());
    assert_eq!(a, Ok(Tagged::B(0x0203)));
}
