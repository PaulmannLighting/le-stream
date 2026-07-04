#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::iter::Peekable;

use serde::de::{
    self, DeserializeOwned, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use crate::{SerdeError, SerdeResult};

const MAC_ADDR8_NAME: &str = "MacAddr8";
const MAC_ADDR6_NAME: &str = "MacAddr6";
const MAC_ADDR8_SIZE: usize = 8;
const MAC_ADDR6_SIZE: usize = 6;

/// Serde deserializer for little-endian byte streams.
#[derive(Clone, Debug)]
pub struct Deserializer<I>
where
    I: Iterator<Item = u8>,
{
    bytes: Peekable<I>,
}

/// Deserialize a value from a little-endian byte stream.
///
/// # Errors
///
/// Returns an error if the stream ends prematurely or contains an unsupported Serde shape.
pub fn from_iter<T, I>(bytes: I) -> SerdeResult<T>
where
    T: DeserializeOwned,
    I: Iterator<Item = u8>,
{
    let mut deserializer = Deserializer::new(bytes);
    T::deserialize(&mut deserializer)
}

/// Deserialize a value from a little-endian byte stream and require that no bytes remain.
///
/// # Errors
///
/// Returns an error if the stream ends prematurely, contains an unsupported Serde shape, or has
/// trailing bytes.
pub fn from_iter_exact<T, I>(bytes: I) -> SerdeResult<T>
where
    T: DeserializeOwned,
    I: Iterator<Item = u8>,
{
    let mut deserializer = Deserializer::new(bytes);
    let value = T::deserialize(&mut deserializer)?;

    deserializer.bytes.next().map_or_else(
        || Ok(value),
        |next_byte| Err(SerdeError::StreamNotExhausted { next_byte }),
    )
}

/// Deserialize a value from a little-endian byte slice.
///
/// # Errors
///
/// Returns an error if the slice ends prematurely or contains an unsupported Serde shape.
pub fn from_slice<T>(bytes: &[u8]) -> SerdeResult<T>
where
    T: DeserializeOwned,
{
    from_iter(bytes.iter().copied())
}

/// Deserialize a value from a little-endian byte slice and require that no bytes remain.
///
/// # Errors
///
/// Returns an error if the slice ends prematurely, contains an unsupported Serde shape, or has
/// trailing bytes.
pub fn from_slice_exact<T>(bytes: &[u8]) -> SerdeResult<T>
where
    T: DeserializeOwned,
{
    from_iter_exact(bytes.iter().copied())
}

impl<I> Deserializer<I>
where
    I: Iterator<Item = u8>,
{
    /// Creates a deserializer that reads from `bytes`.
    pub fn new(bytes: I) -> Self {
        Self {
            bytes: bytes.peekable(),
        }
    }

    fn has_more(&mut self) -> bool {
        self.bytes.peek().is_some()
    }

    fn read_byte(&mut self) -> SerdeResult<u8> {
        self.bytes.next().ok_or(SerdeError::UnexpectedEndOfStream)
    }

    fn read_array<const SIZE: usize>(&mut self) -> SerdeResult<[u8; SIZE]> {
        let mut bytes = [0; SIZE];

        for byte in &mut bytes {
            *byte = self.read_byte()?;
        }

        Ok(bytes)
    }

    fn deserialize_mac_addr6<'de, V>(&mut self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_reversed_newtype_bytes::<MAC_ADDR6_SIZE, _>(visitor)
    }

    fn deserialize_mac_addr8<'de, V>(&mut self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_reversed_newtype_bytes::<MAC_ADDR8_SIZE, _>(visitor)
    }

    fn deserialize_reversed_newtype_bytes<'de, const SIZE: usize, V>(
        &mut self,
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut bytes = self.read_array::<SIZE>()?;
        bytes.reverse();

        let mut deserializer = Deserializer::new(bytes.into_iter());
        let value = visitor.visit_newtype_struct(&mut deserializer)?;

        if deserializer.has_more() {
            return Err(SerdeError::Unsupported);
        }

        Ok(value)
    }
}

impl<'de, I> de::Deserializer<'de> for &mut Deserializer<I>
where
    I: Iterator<Item = u8>,
{
    type Error = SerdeError;

    fn deserialize_any<V>(self, _visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::Unsupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.read_byte()? != 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(i8::from_le_bytes(self.read_array()?))
    }

    fn deserialize_i16<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(i16::from_le_bytes(self.read_array()?))
    }

    fn deserialize_i32<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(i32::from_le_bytes(self.read_array()?))
    }

    fn deserialize_i64<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(i64::from_le_bytes(self.read_array()?))
    }

    fn deserialize_i128<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(i128::from_le_bytes(self.read_array()?))
    }

    fn deserialize_u8<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(u8::from_le_bytes(self.read_array()?))
    }

    fn deserialize_u16<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(u16::from_le_bytes(self.read_array()?))
    }

    fn deserialize_u32<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(u32::from_le_bytes(self.read_array()?))
    }

    fn deserialize_u64<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(u64::from_le_bytes(self.read_array()?))
    }

    fn deserialize_u128<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(u128::from_le_bytes(self.read_array()?))
    }

    fn deserialize_f32<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(f32::from_le_bytes(self.read_array()?))
    }

    fn deserialize_f64<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(f64::from_le_bytes(self.read_array()?))
    }

    fn deserialize_char<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = u32::from_le_bytes(self.read_array()?);
        let character = char::from_u32(value).ok_or(SerdeError::InvalidChar)?;
        visitor.visit_char(character)
    }

    fn deserialize_str<V>(self, _visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::Unsupported)
    }

    fn deserialize_string<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "alloc")]
        {
            let mut bytes = Vec::new();

            for byte in self.bytes.by_ref() {
                bytes.push(byte);
            }

            let value = alloc::string::String::from_utf8(bytes).map_err(|_| SerdeError::Custom)?;
            visitor.visit_string(value)
        }

        #[cfg(not(feature = "alloc"))]
        {
            let _ = visitor;
            Err(SerdeError::Unsupported)
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::Unsupported)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "alloc")]
        {
            let mut bytes = Vec::new();

            for byte in self.bytes.by_ref() {
                bytes.push(byte);
            }

            visitor.visit_byte_buf(bytes)
        }

        #[cfg(not(feature = "alloc"))]
        {
            let _ = visitor;
            Err(SerdeError::Unsupported)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.has_more() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        if name == MAC_ADDR6_NAME {
            return self.deserialize_mac_addr6(visitor);
        }

        if name == MAC_ADDR8_NAME {
            return self.deserialize_mac_addr8(visitor);
        }

        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(CompoundAccess {
            deserializer: self,
            remaining: None,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(CompoundAccess {
            deserializer: self,
            remaining: Some(len),
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(CompoundAccess {
            deserializer: self,
            remaining: None,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(CompoundAccess {
            deserializer: self,
            remaining: Some(fields.len()),
        })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let variant_index = u32::from_le_bytes(self.read_array()?);
        visitor.visit_enum(Variant {
            deserializer: self,
            variant_index,
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

/// Compound Serde deserializer for sequences, tuples, structs, and maps.
pub struct CompoundAccess<'a, I>
where
    I: Iterator<Item = u8>,
{
    deserializer: &'a mut Deserializer<I>,
    remaining: Option<usize>,
}

impl<'de, I> SeqAccess<'de> for CompoundAccess<'_, I>
where
    I: Iterator<Item = u8>,
{
    type Error = SerdeError;

    fn next_element_seed<T>(&mut self, seed: T) -> SerdeResult<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.remaining {
            Some(0) => Ok(None),
            Some(ref mut remaining) => {
                *remaining -= 1;
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
            None if self.deserializer.has_more() => {
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

impl<'de, I> MapAccess<'de> for CompoundAccess<'_, I>
where
    I: Iterator<Item = u8>,
{
    type Error = SerdeError;

    fn next_key_seed<K>(&mut self, seed: K) -> SerdeResult<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.remaining {
            Some(0) => Ok(None),
            Some(ref mut remaining) => {
                *remaining -= 1;
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
            None if self.deserializer.has_more() => {
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> SerdeResult<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

/// Serde enum variant access.
pub struct Variant<'a, I>
where
    I: Iterator<Item = u8>,
{
    deserializer: &'a mut Deserializer<I>,
    variant_index: u32,
}

impl<'de, I> EnumAccess<'de> for Variant<'_, I>
where
    I: Iterator<Item = u8>,
{
    type Error = SerdeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> SerdeResult<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self.variant_index.into_deserializer())?;
        Ok((value, self))
    }
}

impl<'de, I> VariantAccess<'de> for Variant<'_, I>
where
    I: Iterator<Item = u8>,
{
    type Error = SerdeError;

    fn unit_variant(self) -> SerdeResult<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> SerdeResult<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self.deserializer, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> SerdeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.deserializer, "", fields, visitor)
    }
}
