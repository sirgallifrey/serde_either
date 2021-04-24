
use serde::{
    ser::{Serialize, Serializer},
};
use crate::enums::{StringOrStruct, StringOrStructOrVec};

impl<S, V> Serialize for StringOrStructOrVec<S, V>
where
    S: Serialize,
    V: Serialize,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        match self {
            StringOrStructOrVec::String(s) => s.serialize(serializer),
            StringOrStructOrVec::Struct(s) => s.serialize(serializer),
            StringOrStructOrVec::Vec(v) => v.serialize(serializer),
        }
    }
}

impl<S> Serialize for StringOrStruct<S>
where
    S: Serialize,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        match self {
            StringOrStruct::String(s) => s.serialize(serializer),
            StringOrStruct::Struct(s) => s.serialize(serializer),
        }
    }
}