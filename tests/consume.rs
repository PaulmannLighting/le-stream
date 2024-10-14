#![cfg(test)]

use le_stream::Consume;

#[test]
fn test_consume() {
    let bytes = [0xAB, 0xCD];
    let int: u16 = bytes
        .into_iter()
        .consume()
        .expect("Could not consume bytes.");
    assert_eq!(int, 0xCDAB);
}
