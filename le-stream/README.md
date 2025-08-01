# le-stream

Serialize and deserialize object to/from little-endian byte streams.

## Example

```rust
use le_stream::{FromLeStream, ToLeStream};

fn main() {
    let i: i32 = 1234;
    let bytes: [u8; 4] = [0xd2, 0x04, 0x00, 0x00];

    for (byte, target) in i.to_le_stream().zip(bytes) {
        assert_eq!(byte, target);
    }

    let bytes: [u8; 4] = [0xd2, 0x04, 0x00, 0x00];
    let target: i32 = 1234;
    assert_eq!(i32::from_le_stream(&mut bytes.into_iter()), Some(target));
}
```

## Contribution guidelines

* Use `cargo fmt`
* Check code with `cargo clippy`