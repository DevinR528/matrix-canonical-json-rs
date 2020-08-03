use std::{fmt, io};

use serde::{
    ser::{self, Error as _},
    serde_if_integer128, Serialize,
};
use serde_json::{
    ser::{CharEscape, Formatter, State},
    Error,
};

use crate::{CanonicalJsonFmt, Result};

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr) => {
        match $e {
            Result::Ok(val) => val,
            Result::Err(err) => return Result::Err(err),
        }
    };
    ($e:expr,) => {
        tri!($e)
    };
}

/// A structure for serializing Rust values into JSON.
pub struct Serializer<W> {
    pub(crate) writer: W,
    pub(crate) formatter: CanonicalJsonFmt,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new JSON visitor whose output will be written to the writer
    /// specified.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer {
            writer,
            formatter: CanonicalJsonFmt,
        }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        tri!(self
            .formatter
            .write_bool(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        tri!(self
            .formatter
            .write_i8(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        tri!(self
            .formatter
            .write_i16(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        tri!(self
            .formatter
            .write_i32(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        tri!(self
            .formatter
            .write_i64(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .write_number_str(&mut self.writer, &value.to_string())
                .map_err(Error::io)
        }
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        tri!(self
            .formatter
            .write_u8(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        tri!(self
            .formatter
            .write_u16(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        tri!(self
            .formatter
            .write_u32(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        tri!(self
            .formatter
            .write_u64(&mut self.writer, value)
            .map_err(Error::io));
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_u128(self, value: u128) -> Result<()> {
            self.formatter
                .write_number_str(&mut self.writer, &value.to_string())
                .map_err(Error::io)
        }
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        match value.classify() {
            std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                tri!(self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io));
            }
            _ => {
                tri!(self
                    .formatter
                    .write_f32(&mut self.writer, value)
                    .map_err(Error::io));
            }
        }
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        match value.classify() {
            std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                tri!(self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io));
            }
            _ => {
                tri!(self
                    .formatter
                    .write_f64(&mut self.writer, value)
                    .map_err(Error::io));
            }
        }
        Ok(())
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        tri!(format_escaped_str(
            &mut self.writer,
            &mut self.formatter,
            value
        ));
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = tri!(self.serialize_seq(Some(value.len())));
        for byte in value {
            tri!(seq.serialize_element(byte));
        }
        seq.end()
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        tri!(self
            .formatter
            .write_null(&mut self.writer)
            .map_err(Error::io));
        Ok(())
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
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        tri!(self
            .formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .begin_object_key(&mut self.writer, true)
            .map_err(Error::io));
        tri!(self.serialize_str(variant));
        tri!(self
            .formatter
            .end_object_key(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .begin_object_value(&mut self.writer)
            .map_err(Error::io));
        tri!(value.serialize(&mut *self));
        tri!(self
            .formatter
            .end_object_value(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .end_object(&mut self.writer)
            .map_err(Error::io));
        Ok(())
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
        if len == Some(0) {
            tri!(self
                .formatter
                .begin_array(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .end_array(&mut self.writer)
                .map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            tri!(self
                .formatter
                .begin_array(&mut self.writer)
                .map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
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
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        tri!(self
            .formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .begin_object_key(&mut self.writer, true)
            .map_err(Error::io));
        tri!(self.serialize_str(variant));
        tri!(self
            .formatter
            .end_object_key(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .begin_object_value(&mut self.writer)
            .map_err(Error::io));
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if len == Some(0) {
            tri!(self
                .formatter
                .begin_object(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .end_object(&mut self.writer)
                .map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            tri!(self
                .formatter
                .begin_object(&mut self.writer)
                .map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        tri!(self
            .formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .begin_object_key(&mut self.writer, true)
            .map_err(Error::io));
        tri!(self.serialize_str(variant));
        tri!(self
            .formatter
            .end_object_key(&mut self.writer)
            .map_err(Error::io));
        tri!(self
            .formatter
            .begin_object_value(&mut self.writer)
            .map_err(Error::io));
        self.serialize_map(Some(len))
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + fmt::Display,
    {
        use self::fmt::Write;

        struct Adapter<'ser, W: 'ser> {
            writer: &'ser mut W,
            formatter: &'ser mut CanonicalJsonFmt,
            error: Option<io::Error>,
        }

        impl<'ser, W> Write for Adapter<'ser, W>
        where
            W: io::Write,
        {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                debug_assert!(self.error.is_none());
                match format_escaped_str_contents(self.writer, self.formatter, s) {
                    Ok(()) => Ok(()),
                    Err(_) => {
                        self.error = Some(io::Error::new(io::ErrorKind::Other, "write failed"));
                        Err(fmt::Error)
                    }
                }
            }
        }

        tri!(self
            .formatter
            .begin_string(&mut self.writer)
            .map_err(Error::io));
        {
            let mut adapter = Adapter {
                writer: &mut self.writer,
                formatter: &mut self.formatter,
                error: None,
            };
            match write!(adapter, "{}", value) {
                Ok(()) => debug_assert!(adapter.error.is_none()),
                Err(fmt::Error) => {
                    return Err(Error::io(adapter.error.expect("there should be an error")));
                }
            }
        }
        tri!(self
            .formatter
            .end_string(&mut self.writer)
            .map_err(Error::io));
        Ok(())
    }
}

#[doc(hidden)]
pub enum Compound<'a, W: 'a> {
    Map {
        ser: &'a mut Serializer<W>,
        state: State,
    },
    #[cfg(feature = "arbitrary_precision")]
    Number { ser: &'a mut Serializer<W> },
    #[cfg(feature = "raw_value")]
    RawValue { ser: &'a mut Serializer<W> },
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                tri!(ser
                    .formatter
                    .begin_array_value(&mut ser.writer, *state == State::First)
                    .map_err(Error::io));
                *state = State::Rest;
                tri!(value.serialize(&mut **ser));
                tri!(ser
                    .formatter
                    .end_array_value(&mut ser.writer)
                    .map_err(Error::io));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => tri!(ser.formatter.end_array(&mut ser.writer).map_err(Error::io)),
                }
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => tri!(ser.formatter.end_array(&mut ser.writer).map_err(Error::io)),
                }
                tri!(ser
                    .formatter
                    .end_object_value(&mut ser.writer)
                    .map_err(Error::io));
                tri!(ser.formatter.end_object(&mut ser.writer).map_err(Error::io));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                tri!(ser
                    .formatter
                    .begin_object_key(&mut ser.writer, *state == State::First)
                    .map_err(Error::io));
                *state = State::Rest;

                tri!(key.serialize(crate::MapKeySerializer { ser: *ser }));

                tri!(ser
                    .formatter
                    .end_object_key(&mut ser.writer)
                    .map_err(Error::io));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            Compound::Map { ref mut ser, .. } => {
                tri!(ser
                    .formatter
                    .begin_object_value(&mut ser.writer)
                    .map_err(Error::io));
                tri!(value.serialize(&mut **ser));
                tri!(ser
                    .formatter
                    .end_object_value(&mut ser.writer)
                    .map_err(Error::io));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => tri!(ser.formatter.end_object(&mut ser.writer).map_err(Error::io)),
                }
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            Compound::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { ref mut ser, .. } => {
                if key == crate::number::TOKEN {
                    tri!(value.serialize(NumberStrEmitter(&mut *ser)));
                    Ok(())
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            Compound::RawValue { ref mut ser, .. } => {
                if key == crate::raw::TOKEN {
                    tri!(value.serialize(RawValueStrEmitter(&mut *ser)));
                    Ok(())
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { .. } => ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => Ok(()),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => Ok(()),
        }
    }
}

impl<'a, W> ser::SerializeStructVariant for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            Compound::Map { .. } => ser::SerializeStruct::serialize_field(self, key, value),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => tri!(ser.formatter.end_object(&mut ser.writer).map_err(Error::io)),
                }
                tri!(ser
                    .formatter
                    .end_object_value(&mut ser.writer)
                    .map_err(Error::io));
                tri!(ser.formatter.end_object(&mut ser.writer).map_err(Error::io));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}

fn format_escaped_str<W>(
    writer: &mut W,
    formatter: &mut CanonicalJsonFmt,
    value: &str,
) -> serde_json::Result<()>
where
    W: ?Sized + io::Write,
{
    tri!(formatter.begin_string(writer).map_err(Error::custom));
    tri!(format_escaped_str_contents(writer, formatter, value));
    tri!(formatter.end_string(writer).map_err(Error::custom));
    Ok(())
}

fn format_escaped_str_contents<W>(
    writer: &mut W,
    formatter: &mut CanonicalJsonFmt,
    value: &str,
) -> serde_json::Result<()>
where
    W: ?Sized + io::Write,
{
    let bytes = value.as_bytes();

    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            tri!(formatter
                .write_string_fragment(writer, &value[start..i])
                .map_err(Error::custom));
        }

        let char_escape = from_escape_table(escape, byte);
        tri!(formatter
            .write_char_escape(writer, char_escape)
            .map_err(Error::custom));

        start = i + 1;
    }

    if start != bytes.len() {
        tri!(formatter
            .write_string_fragment(writer, &value[start..])
            .map_err(Error::custom));
    }

    Ok(())
}

#[inline]
fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
    match escape {
        self::BB => CharEscape::Backspace,
        self::TT => CharEscape::Tab,
        self::NN => CharEscape::LineFeed,
        self::FF => CharEscape::FormFeed,
        self::RR => CharEscape::CarriageReturn,
        self::QU => CharEscape::Quote,
        self::BS => CharEscape::ReverseSolidus,
        self::UU => CharEscape::AsciiControl(byte),
        _ => unreachable!(),
    }
}

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const UU: u8 = b'u'; // \x00...\x1F except the ones above
const __: u8 = 0;

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU, // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];
