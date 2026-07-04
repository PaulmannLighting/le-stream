#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use serde::Serialize;
use serde::ser::{
    self, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};

use crate::{ByteWriter, SerdeError, SerdeResult};

const MAC_ADDR8_NAME: &str = "MacAddr8";
const MAC_ADDR6_NAME: &str = "MacAddr6";
const MAC_ADDR8_SIZE: usize = 8;
const MAC_ADDR6_SIZE: usize = 6;

/// Serde serializer for little-endian byte streams.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Serializer<W> {
    writer: W,
}

/// Serialize a value into a little-endian byte stream.
///
/// # Errors
///
/// Returns an error if the value contains an unsupported Serde shape or the writer fails.
pub fn to_writer<T, W>(value: &T, writer: W) -> SerdeResult<W>
where
    T: ?Sized + Serialize,
    W: ByteWriter,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

/// Serialize a value into a freshly allocated little-endian byte stream.
///
/// # Errors
///
/// Returns an error if the value contains an unsupported Serde shape.
#[cfg(feature = "alloc")]
pub fn to_vec<T>(value: &T) -> SerdeResult<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    to_writer(value, Vec::new())
}

/// Serialize a value into an existing byte buffer, returning the number of bytes written.
///
/// # Errors
///
/// Returns an error if the buffer is too small or the value contains an unsupported Serde shape.
pub fn to_slice<T>(value: &T, bytes: &mut [u8]) -> SerdeResult<usize>
where
    T: ?Sized + Serialize,
{
    let capacity = bytes.len();
    let remaining = to_writer(value, bytes)?.len();
    Ok(capacity - remaining)
}

impl<W> Serializer<W> {
    /// Creates a serializer that writes into `writer`.
    pub const fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Extracts the wrapped writer.
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W> Serializer<W>
where
    W: ByteWriter,
{
    fn write_all(&mut self, bytes: &[u8]) -> SerdeResult<()> {
        self.writer.write_all(bytes)
    }

    fn serialize_mac_addr6<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_reversed_newtype_bytes::<_, MAC_ADDR6_SIZE>(value)
    }

    fn serialize_mac_addr8<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_reversed_newtype_bytes::<_, MAC_ADDR8_SIZE>(value)
    }

    fn serialize_reversed_newtype_bytes<T, const SIZE: usize>(
        &mut self,
        value: &T,
    ) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        let mut bytes = [0; SIZE];
        let written = to_slice(value, &mut bytes)?;

        if written != SIZE {
            return Err(SerdeError::Unsupported);
        }

        bytes.reverse();
        self.write_all(&bytes)
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();
    type SerializeMap = Compound<'a, W>;
    type SerializeSeq = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;

    fn serialize_bool(self, value: bool) -> SerdeResult<Self::Ok> {
        self.serialize_u8(u8::from(value))
    }

    fn serialize_i8(self, value: i8) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_i16(self, value: i16) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_i32(self, value: i32) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_i64(self, value: i64) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_i128(self, value: i128) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_u8(self, value: u8) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_u16(self, value: u16) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_u32(self, value: u32) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_u64(self, value: u64) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_u128(self, value: u128) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_f32(self, value: f32) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_f64(self, value: f64) -> SerdeResult<Self::Ok> {
        self.write_all(&value.to_le_bytes())
    }

    fn serialize_char(self, value: char) -> SerdeResult<Self::Ok> {
        self.serialize_u32(u32::from(value))
    }

    fn serialize_str(self, value: &str) -> SerdeResult<Self::Ok> {
        self.write_all(value.as_bytes())
    }

    fn serialize_bytes(self, value: &[u8]) -> SerdeResult<Self::Ok> {
        self.write_all(value)
    }

    fn serialize_none(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> SerdeResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> SerdeResult<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> SerdeResult<Self::Ok> {
        self.serialize_u32(variant_index)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> SerdeResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        if name == MAC_ADDR6_NAME {
            return self.serialize_mac_addr6(value);
        }

        if name == MAC_ADDR8_NAME {
            return self.serialize_mac_addr8(value);
        }

        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> SerdeResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> SerdeResult<Self::SerializeSeq> {
        Ok(Compound { serializer: self })
    }

    fn serialize_tuple(self, _len: usize) -> SerdeResult<Self::SerializeTuple> {
        Ok(Compound { serializer: self })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleStruct> {
        Ok(Compound { serializer: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleVariant> {
        self.serialize_u32(variant_index)?;
        Ok(Compound { serializer: self })
    }

    fn serialize_map(self, _len: Option<usize>) -> SerdeResult<Self::SerializeMap> {
        Ok(Compound { serializer: self })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStruct> {
        Ok(Compound { serializer: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStructVariant> {
        self.serialize_u32(variant_index)?;
        Ok(Compound { serializer: self })
    }

    fn collect_str<T>(self, _value: &T) -> SerdeResult<Self::Ok>
    where
        T: ?Sized + core::fmt::Display,
    {
        Err(SerdeError::Unsupported)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

/// Compound Serde serializer for sequences, tuples, structs, and maps.
pub struct Compound<'a, W> {
    serializer: &'a mut Serializer<W>,
}

impl<W> SerializeSeq for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_element<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}

impl<W> SerializeTuple for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_element<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}

impl<W> SerializeTupleStruct for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_field<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}

impl<W> SerializeTupleVariant for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_field<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}

impl<W> SerializeMap for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_key<T>(&mut self, key: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<T>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}

impl<W> SerializeStruct for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}

impl<W> SerializeStructVariant for Compound<'_, W>
where
    W: ByteWriter,
{
    type Error = SerdeError;
    type Ok = ();

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(())
    }
}
