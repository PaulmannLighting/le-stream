#![cfg(test)]

use le_stream::{Consume, Error};

#[test]
fn test_consume() {
    let bytes = [0xAB, 0xCD];
    let int: Result<u16, Error> = bytes.into_iter().consume();
    assert_eq!(int, Ok(0xCDAB));
}
