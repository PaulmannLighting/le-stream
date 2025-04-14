//! Test d-/serialization of `Option<T>`.

#![cfg(test)]

use le_stream::{FromLeStream, ToLeStream};

#[test]
fn test_serialize_none() {
    let value: Option<u8> = None;
    let bytes: Vec<_> = value.to_le_stream().collect();
    assert_eq!(bytes, vec![]);
}

#[test]
fn test_serialize_some() {
    let value: Option<u8> = Some(0xAB);
    let bytes: Vec<_> = value.to_le_stream().collect();
    assert_eq!(bytes, vec![0xAB]);
}

#[test]
fn test_deserialize_none() {
    let bytes = [];
    let result: Option<Option<u8>> = Option::from_le_stream(bytes.into_iter());
    assert_eq!(result, Some(None));
}

#[test]
fn test_deserialize_some() {
    let bytes = [0xAB];
    let result: Option<Option<u8>> = Option::from_le_stream(bytes.into_iter());
    assert_eq!(result, Some(Some(0xAB)));
}

#[test]
fn test_deserialize_err() {
    let bytes = [0xAB];
    let result: Option<Option<u16>> = Option::from_le_stream(bytes.into_iter());
    assert_eq!(result, None);
}
