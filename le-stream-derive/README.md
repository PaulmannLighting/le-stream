# le-stream-derive

Derive macros for `le-stream`.

## Example

```rust
use le_stream::{FromLeStream, ToLeStream};
use le_stream_derive::{FromLeStream, ToLeStream};

const MY_STRUCT_BYTES: [u8; 39] = [
    0x42, 0x37, 0x13, 0x12, 0x34, 0x56, 0x78, 0xff, 0xaa, 0xbb, 0xcc, 0xdd, 0x34, 0x12, 0x56, 0x78,
    0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc, 0x00, 0x42,
    0x37, 0x13, 0x12, 0x42, 0x37, 0x13, 0x12,
];

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
}
```

## Serialize

```rust
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
    };

    assert_eq!(
        my_struct.to_le_stream().collect::<Vec<_>>(),
        MY_STRUCT_BYTES
    );
}
```

# Deserialize

```rust
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
}
```

## Contribution guidelines

* Use `cargo fmt`
* Check code with `cargo clippy`