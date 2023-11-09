mod error;
mod from_le_bytes;
mod to_le_bytes;

pub use error::{Error, Result};
pub use from_le_bytes::FromLeBytes;
pub use to_le_bytes::ToLeBytes;

#[cfg(test)]
mod tests {
    use le_stream::{FromLeBytes, ToLeBytes};
    use le_stream_derive::{FromLeBytes, ToLeBytes};

    #[derive(Debug, FromLeBytes, ToLeBytes)]
    struct MyStruct {
        flag: u8,
        num: u16,
        tail: u8,
    }

    fn main() {
        let bytes = [0x42, 0x37, 0x13, 0xFF];
        let my_struct = MyStruct::from_le_bytes(&mut bytes.into_iter())
            .expect("Could not create struct from byte stream.");
        assert_eq!(my_struct.flag, 0x42);
        assert_eq!(my_struct.num, 0x1337);
        assert_eq!(my_struct.tail, 0xff);
        println!("{my_struct:#04X?}");

        for byte in my_struct.to_le_bytes() {
            println!("{byte:#04X?}");
        }
    }
}
