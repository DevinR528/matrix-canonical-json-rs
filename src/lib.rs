use std::{fmt, io};

use serde::{
    ser::{self, Error},
    serde_if_integer128, Serialize,
};
use serde_json::ser::Formatter;

mod map_key;
mod serializer;

pub use map_key::MapKeySerializer;
pub use serializer::{Compound, Serializer};

pub type Result<T> = std::result::Result<T, serde_json::Error>;

#[inline]
fn to_canonical_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = CanonicalJson::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}

#[inline]
fn to_canonical_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_canonical_writer(&mut writer, value)?;
    if writer.len() > 65_535 {
        return Err(serde_json::Error::custom(
            "canonical JSON larger than 65,535 bytes is not allowed",
        ));
    }
    Ok(writer)
}

pub fn to_canonical_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_canonical_vec(value)?;
    Ok(
        // No invalid utf8 is emitted from serde_json
        // or the CanonicalJson formatter.
        unsafe { String::from_utf8_unchecked(vec) },
    )
}

pub struct CanonicalJson<W> {
    ser: Serializer<W>,
}

impl<W: io::Write> CanonicalJson<W> {
    pub fn new(writer: W) -> Self {
        Self {
            ser: Serializer::new(writer),
        }
    }
}

impl<'a, W> ser::Serializer for &'a mut CanonicalJson<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = serde_json::Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = MapKeySorted<'a, W>;
    type SerializeStruct = MapKeySorted<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.ser.serialize_bool(value)
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

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(Error::custom("floats are not allowed"))
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(Error::custom("floats are not allowed"))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.ser.serialize_str(value)
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(value.len()))?;
        for byte in value {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        self.ser.serialize_unit()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    /// Serialize newtypes without an object wrapper.
    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.ser.serialize_seq(len)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.ser
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapKeySorted {
            ser: self,
            pairs: vec![],
        })
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.ser
            .serialize_struct_variant(name, variant_index, variant, len)
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + fmt::Display,
    {
        self.ser.collect_str(value)
    }
}

pub enum State {
    Empty,
    First,
    Rest,
}

pub struct MapKeySorted<'a, W> {
    ser: &'a mut CanonicalJson<W>,
    pairs: Vec<String>,
}

impl<'a, W> ser::SerializeMap for MapKeySorted<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: Serialize,
        V: Serialize,
    {
        let mut buf = vec![];
        let mut ser = Serializer::new(&mut buf);

        key.serialize(MapKeySerializer { ser: &mut ser })?;
        buf.push(b':');
        value.serialize(&mut Serializer::new(&mut buf))?;

        let pair = unsafe { String::from_utf8_unchecked(buf) };
        self.pairs.push(pair);

        Ok(())
    }

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.pairs.sort();
        let count = self.pairs.len();
        self.ser
            .ser
            .writer
            .write_all(&[b'{'])
            .map_err(Error::custom)?;
        for (idx, pair) in self.pairs.drain(..).enumerate() {
            self.ser
                .ser
                .writer
                .write_all(pair.as_bytes())
                .map_err(Error::custom)?;

            if count != idx + 1 {
                self.ser
                    .ser
                    .writer
                    .write_all(&[b','])
                    .map_err(Error::custom)?;
            }
        }
        self.ser
            .ser
            .writer
            .write_all(&[b'}'])
            .map_err(Error::custom)?;

        self.pairs.clear();

        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for MapKeySorted<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = serde_json::Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeMap::end(self)
    }
}

pub struct CanonicalJsonFmt;

impl Formatter for CanonicalJsonFmt {}

#[test]
fn check_canonical_empty() {
    let json = serde_json::json!({});
    assert_eq!(to_canonical_string(&json).unwrap(), r#"{}"#)
}

#[test]
fn check_canonical_num() {
    let json = serde_json::json!({
        "b": "2",
        "a": "1"
    });
    assert_eq!(to_canonical_string(&json).unwrap(), r#"{"a":"1","b":"2"}"#)
}

#[test]
fn check_canonical_obj() {
    let json = serde_json::json!({ "one": 1, "two": "Two" });
    assert_eq!(
        to_canonical_string(&json).unwrap(),
        r#"{"one":1,"two":"Two"}"#
    )
}

#[test]
fn check_canonical_sorts_keys() {
    let json = serde_json::json!({
        "auth": {
            "success": true,
            "mxid": "@john.doe:example.com",
            "profile": {
                "display_name": "John Doe",
                "three_pids": [
                    {
                        "medium": "email",
                        "address": "john.doe@example.org"
                    },
                    {
                        "medium": "msisdn",
                        "address": "123456789"
                    }
                ]
            }
        }
    });

    assert_eq!(
        to_canonical_string(&json).unwrap(),
        r#"{"auth":{"mxid":"@john.doe:example.com","profile":{"display_name":"John Doe","three_pids":[{"address":"john.doe@example.org","medium":"email"},{"address":"123456789","medium":"msisdn"}]},"success":true}}"#
    )
}

#[test]
fn check_canonical_utf8_keys() {
    let json = serde_json::json!({
        "本": 2,
        "日": 1
    });
    assert_eq!(to_canonical_string(&json).unwrap(), r#"{"日":1,"本":2}"#)
}

#[test]
fn check_canonical_utf8_value() {
    let json = serde_json::json!({ "a": "日本語" });
    assert_eq!(to_canonical_string(&json).unwrap(), r#"{"a":"日本語"}"#)
}

#[test]
fn check_canonical_utf8_display() {
    let json = serde_json::json!({ "a": "\u{65E5}" });
    assert_eq!(to_canonical_string(&json).unwrap(), r#"{"a":"日"}"#)
}

#[test]
fn check_canonical_null() {
    let json = serde_json::json!({ "a": null });
    assert_eq!(to_canonical_string(&json).unwrap(), r#"{"a":null}"#)
}

#[test]
fn check_canonical_float_value() {
    let json = serde_json::json!({ "a": 1.01_f32 });
    assert!(to_canonical_string(&json).is_err())
}

#[test]
fn sorts_keys_of_structs() {
    #[derive(Debug, serde_derive::Serialize)]
    struct Test {
        z: u8,
        y: u64,
        x: usize,
    }

    let t = Test { x: 1, y: 23, z: 10 };

    assert_eq!(to_canonical_string(&t).unwrap(), r#"{"x":1,"y":23,"z":10}"#)
}
