#![allow(unused_variables)]

use std::{
    fmt::Display,
    io::{Read, Write},
    num::TryFromIntError,
    string::FromUtf8Error,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{
    de::{self, DeserializeOwned, SeqAccess},
    ser::{self, Impossible},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Illegal string")]
    IllegalString(#[from] std::str::Utf8Error),
    #[error("Illegal string length")]
    IllegalStringLength,
    #[error("I/O error")]
    IOError(#[from] std::io::Error),
    #[error("Sequence length required")]
    SequenceLengthRequired,
    #[error("Sequence too long")]
    SequenceTooLong(#[from] TryFromIntError),
    #[error("Unsupported type")]
    UnsupportedType,
    #[error("{0}")]
    Custom(String),
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::from(value.utf8_error())
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Serializer<W: Write> {
    pub writer: W,
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_i8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(self.writer.write_i8(v)?)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(self.writer.write_i16::<LittleEndian>(v)?)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(self.writer.write_i32::<LittleEndian>(v)?)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(self.writer.write_i64::<LittleEndian>(v)?)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(self.writer.write_u8(v)?)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(self.writer.write_u16::<LittleEndian>(v)?)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(self.writer.write_u32::<LittleEndian>(v)?)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(self.writer.write_u64::<LittleEndian>(v)?)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(self.writer.write_f32::<LittleEndian>(v)?)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(self.writer.write_f64::<LittleEndian>(v)?)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let mut val = v.len();
        loop {
            let mut byte = (val & 127) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            self.writer.write_all(&[byte])?;
            if val == 0 {
                break;
            }
        }
        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self, name: &'static str, value: &T) -> Result<Self::Ok> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let len32 = match len {
            None => return Err(Error::SequenceLengthRequired),
            Some(n) => i32::try_from(n)?,
        };
        self.serialize_i32(len32 - 1)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedType)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        // Error::Custom(msg.to_string())
        panic!("{}", msg)
    }
}

pub fn read_bytes<R: Read>(mut r: R) -> Result<Vec<u8>> {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let byte = r.read_i8()?;
        result |= ((byte & 0x7F) as usize) << shift;
        if byte >= 0 {
            break;
        }
        shift += 7;
        if shift >= 32 {
            return Err(Error::IllegalStringLength);
        }
    }
    let mut buf: Vec<u8> = vec![0u8; result];
    r.read_exact(&mut buf[..])?;
    Ok(buf)
}

pub struct Deserializer<R: Read> {
    pub reader: R,
}

pub fn from_reader<R: Read, T: DeserializeOwned>(r: R) -> Result<T> {
    de::Deserialize::deserialize(&mut Deserializer { reader: r })
}

impl<'a, 'de, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.reader.read_u8()?;
        visitor.visit_bool(n > 0)
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(self.reader.read_i8()?)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(self.reader.read_i16::<LittleEndian>()?)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.reader.read_i32::<LittleEndian>()?)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.reader.read_i64::<LittleEndian>()?)
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.reader.read_u8()?)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.reader.read_u16::<LittleEndian>()?)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.reader.read_u32::<LittleEndian>()?)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(self.reader.read_u64::<LittleEndian>()?)
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.reader.read_f32::<LittleEndian>()?)
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.reader.read_f64::<LittleEndian>()?)
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let buf = read_bytes(&mut self.reader)?;
        visitor.visit_str(std::str::from_utf8(&buf[..]).unwrap())
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let buf = read_bytes(&mut self.reader)?;
        visitor.visit_string(String::from_utf8(buf).unwrap())
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let buf = read_bytes(&mut self.reader)?;
        visitor.visit_bytes(&buf[..])
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let buf = read_bytes(&mut self.reader)?;
        visitor.visit_byte_buf(buf)
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let last = self.reader.read_i32::<LittleEndian>()?;
        visitor.visit_seq(FixedSeq {
            de: self,
            n: usize::try_from(last + 1).unwrap(),
        })
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(FixedSeq { de: self, n: len })
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_struct<V: de::Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        visitor.visit_seq(FixedSeq { de: self, n: fields.len() })
    }

    fn deserialize_enum<V: de::Visitor<'de>>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

struct FixedSeq<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    n: usize,
}

impl<'a, 'de, R: Read> SeqAccess<'de> for FixedSeq<'a, R> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.n == 0 {
            Ok(None)
        } else {
            self.n -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

pub mod array_hack {
    use serde::{
        de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor},
        ser::{Error, Serialize, SerializeTuple, Serializer},
    };
    use std::{fmt, marker::PhantomData};

    pub(crate) fn serialize<S: Serializer, T: Serialize>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error> {
        struct HackTuple<'a, T>(&'a Vec<T>);
        impl<'a, T: Serialize> Serialize for HackTuple<'a, T> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut seq = serializer.serialize_tuple(self.0.len())?;
                for e in self.0 {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }

        let len32 = match i32::try_from(value.len()) {
            Ok(v) => v,
            Err(e) => return Err(S::Error::custom("array too large")),
        };
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&len32)?;
        seq.serialize_element(&HackTuple(value))?;
        seq.end()
    }

    // This horrible thing is needed to handle the case where an array is prefix
    // by its actual length instead of the usual length-1.
    pub(crate) fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<Vec<T>, D::Error> {
        struct HackVisitor<T>(PhantomData<T>);
        struct HackSeed<T> {
            len: usize,
            _marker: PhantomData<T>,
        }

        impl<'de, T: Deserialize<'de>> Visitor<'de> for HackSeed<T> {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Vec::<T>::with_capacity(self.len);

                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }

                Ok(values)
            }
        }

        impl<'de, T: Deserialize<'de>> DeserializeSeed<'de> for HackSeed<T> {
            type Value = Vec<T>;

            fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
                deserializer.deserialize_tuple(self.len, self)
            }
        }

        impl<'de, T: Deserialize<'de>> Visitor<'de> for HackVisitor<T> {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let num: i32 = seq.next_element()?.unwrap();
                seq.next_element_seed(HackSeed {
                    len: usize::try_from(num).unwrap_or(0),
                    _marker: PhantomData,
                })
                .map(Option::unwrap)
            }
        }

        deserializer.deserialize_tuple(2, HackVisitor(PhantomData))
    }
}
