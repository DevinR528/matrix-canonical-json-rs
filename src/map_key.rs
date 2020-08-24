use std::{fmt, io};

use serde::{
    ser::{self, Error as _, Impossible},
    serde_if_integer128, Serialize,
};

use crate::{serializer::Serializer, Error, Result};

pub struct MapKeySerializer<'a, W: 'a> {
    pub ser: &'a mut Serializer<W>,
}

#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
    Error::custom("invalid number")
}

#[cfg(feature = "raw_value")]
fn invalid_raw_value() -> Error {
    Error::custom("invalid raw value")
}

fn key_must_be_a_string() -> Error {
    Error::custom("key must be a string")
}

impl<'a, W> ser::Serializer for MapKeySerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.ser.serialize_str(value)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.ser.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _value: bool) -> Result<()> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.ser.serialize_i8(value)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.ser.serialize_i16(value)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.ser.serialize_i32(value)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.ser.serialize_i64(value)
    }

    serde_if_integer128! {
        fn serialize_i128(self, value: i128) -> Result<()> {
            self.ser.serialize_i128(value)
        }

    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        self.ser.serialize_u8(value)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        self.ser.serialize_u16(value)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        self.ser.serialize_u32(value)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        self.ser.serialize_u64(value)
    }

    serde_if_integer128! {
        fn serialize_u128(self, value: u128) -> Result<()> {
            self.ser.serialize_u128(value)
        }
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.ser.serialize_str(&value.to_string())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + fmt::Display,
    {
        self.ser.collect_str(value)
    }
}
