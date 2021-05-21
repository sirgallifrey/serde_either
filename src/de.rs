
use serde::{
    de::{Deserialize, Deserializer, Error, Expected, Unexpected},
};
use serde_value::{Value, ValueDeserializer};
use crate::enums::{StringOrStruct, StringOrStructOrVec, SingleOrVec};

// the unexpected function was copied from https://github.com/arcnmx/serde-value/blob/master/src/lib.rs
// note that serde-value is licensed under MIT https://github.com/arcnmx/serde-value/blob/master/COPYING
// credit goes to arcnmx
fn unexpected(value: &Value) -> Unexpected {
    match *value {
        Value::Bool(b) => serde::de::Unexpected::Bool(b),
        Value::U8(n) => serde::de::Unexpected::Unsigned(n as u64),
        Value::U16(n) => serde::de::Unexpected::Unsigned(n as u64),
        Value::U32(n) => serde::de::Unexpected::Unsigned(n as u64),
        Value::U64(n) => serde::de::Unexpected::Unsigned(n),
        Value::I8(n) => serde::de::Unexpected::Signed(n as i64),
        Value::I16(n) => serde::de::Unexpected::Signed(n as i64),
        Value::I32(n) => serde::de::Unexpected::Signed(n as i64),
        Value::I64(n) => serde::de::Unexpected::Signed(n),
        Value::F32(n) => serde::de::Unexpected::Float(n as f64),
        Value::F64(n) => serde::de::Unexpected::Float(n),
        Value::Char(c) => serde::de::Unexpected::Char(c),
        Value::String(ref s) => serde::de::Unexpected::Str(s),
        Value::Unit => serde::de::Unexpected::Unit,
        Value::Option(_) => serde::de::Unexpected::Option,
        Value::Newtype(_) => serde::de::Unexpected::NewtypeStruct,
        Value::Seq(_) => serde::de::Unexpected::Seq,
        Value::Map(_) => serde::de::Unexpected::Map,
        Value::Bytes(ref b) => serde::de::Unexpected::Bytes(b),
    }
}

impl<'de, S, V> StringOrStructOrVec<S, V>
where
    S: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize_with_expected<D>(
        deserializer: D,
        expected: &dyn Expected,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        return match value {
            Value::String(_) | Value::Bytes(_) => Ok(Self::String(String::deserialize(
                ValueDeserializer::new(value),
            )?)),
            Value::Seq(_) => Ok(Self::Vec(V::deserialize(ValueDeserializer::new(value))?)),
            Value::Map(_) => Ok(Self::Struct(S::deserialize(ValueDeserializer::new(value))?)),
            _ => Err(Error::invalid_type(unexpected(&value), expected)),
        };
    }
}

impl<'de, S, V> Deserialize<'de> for StringOrStructOrVec<S, V>
where
    S: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        StringOrStructOrVec::<S, V>::deserialize_with_expected(
            deserializer,
            &"String, Struct or Vec",
        )
    }
}

impl<'de, S> Deserialize<'de> for StringOrStruct<S>
where
    S: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = StringOrStructOrVec::<S, S>::deserialize_with_expected(
            deserializer,
            &"String or Struct",
        )?;
        return match value {
            StringOrStructOrVec::String(s) => Ok(StringOrStruct::String(s)),
            StringOrStructOrVec::Struct(v) | StringOrStructOrVec::Vec(v) => {
                Ok(StringOrStruct::Struct(v))
            }
        };
    }
}

impl<'de, S> Deserialize<'de> for SingleOrVec<S>
where
    S: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        return match value {
            Value::Seq(_) => Ok(Self::Vec(Vec::<S>::deserialize(ValueDeserializer::new(value))?)),
            _ => Ok(Self::Single(S::deserialize(ValueDeserializer::new(value))?)),
        };
    }
}