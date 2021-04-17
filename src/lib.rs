#![warn(clippy::all)]

//! This crate provides a set of Enums that help you to define data structures
//! that accept either strings, structs or arrays.
//!
//! # Example
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_either::StringOrStruct;
//! use serde_json;
//! 
//! #[derive(Serialize, Deserialize)]
//! struct Authors {
//!   first_name: String,
//!   last_name: String
//! }
//! 
//! #[derive(Serialize, Deserialize)]
//! struct Book {
//!   pub authors: StringOrStruct<Authors>
//! }
//! 
//! // And StringOrStruct is just a normal enum
//! 
//! impl Book {
//!   fn get_author_name(&self) -> String {
//!     match &self.authors {
//!       StringOrStruct::String(s) => s.to_owned(),
//!       StringOrStruct::Struct(author) => format!("{} {}", &author.first_name, &author.last_name)
//!     }
//!   }
//! }
//! 
//! let books = r#"[
//!     {
//!         "authors": {
//!             "first_name": "John",
//!             "last_name": "Smith"
//!         }
//!     },
//!     {
//!         "authors": "Michael J. Smith"
//!     }
//! ]"#;
//!
//! let res: Vec<Book> = serde_json::from_str(books).unwrap();
//! assert_eq!(res[0].get_author_name(), "John Smith");
//! assert_eq!(res[1].get_author_name(), "Michael J. Smith");
//!
//! ```
//!

use serde::{
    de::{Deserialize, Deserializer, Error, Expected, Unexpected},
    ser::{Serialize, Serializer},
};
use serde_value::{Value, ValueDeserializer};

#[derive(Debug, PartialEq)]
pub enum StringOrStruct<S> {
    String(String),
    Struct(S),
}

#[derive(Debug, PartialEq)]
pub enum StringOrStructOrVec<S, V> {
    String(String),
    Struct(S),
    Vec(V),
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::eyre;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct SimpleStruct {
        pub number: i32,
        pub text: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct MyType {
        pub string_or_struct: Option<StringOrStruct<SimpleStruct>>,
        pub string_or_struct_with_vec: Option<StringOrStruct<Vec<SimpleStruct>>>,
        pub string_or_struct_with_vec_of_u8: Option<StringOrStruct<Vec<u8>>>,
        pub string_or_struct_or_vec: Option<StringOrStructOrVec<SimpleStruct, Vec<SimpleStruct>>>,
    }

    mod string_or_struct {
        use super::*;

        mod deserialize {
            use super::*;

            #[test]
            fn string_value() {
                let string_value_json = r#"{
                    "string_or_struct": "some string"
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();

                let value = match res.string_or_struct.unwrap() {
                    StringOrStruct::String(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(value.unwrap(), "some string");
            }
            #[test]
            fn struct_value() {
                let string_value_json = r#"{
                    "string_or_struct": {
                        "number": 42,
                        "text": "some text"
                    }
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();
                let value = match res.string_or_struct.unwrap() {
                    StringOrStruct::Struct(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(
                    value.unwrap(),
                    SimpleStruct {
                        number: 42,
                        text: String::from("some text")
                    }
                );
            }
            #[test]
            fn vec_of_structs() {
                let string_value_json = r#"{
                    "string_or_struct_with_vec": [{
                        "number": 42,
                        "text": "some text"
                    },
                    {
                        "number": 3,
                        "text": "some other text"
                    }]
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();
                let value = match res.string_or_struct_with_vec.unwrap() {
                    StringOrStruct::Struct(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(
                    value.unwrap(),
                    vec![
                        SimpleStruct {
                            number: 42,
                            text: String::from("some text")
                        },
                        SimpleStruct {
                            number: 3,
                            text: String::from("some other text")
                        }
                    ]
                );
            }

            #[test]
            fn vec_of_u8s() {
                let string_value_json = r#"{
                    "string_or_struct_with_vec_of_u8": [1,5,8,12,32]
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();
                let value = match res.string_or_struct_with_vec_of_u8.unwrap() {
                    StringOrStruct::Struct(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(value.unwrap(), vec![1, 5, 8, 12, 32]);
            }

            #[test]
            fn vec_of_u8s_containing_string() {
                let string_value_json = r#"{
                    "string_or_struct_with_vec_of_u8": "[1,5,8,12,32]"
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();
                let value = match res.string_or_struct_with_vec_of_u8.unwrap() {
                    StringOrStruct::String(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(value.unwrap(), "[1,5,8,12,32]");
            }

            mod errors {
                use super::*;

                #[test]
                fn on_number() {
                    let string_value_json = r#"{
                        "string_or_struct": 18
                    }"#;

                    let res: serde_json::Result<MyType> = serde_json::from_str(string_value_json);

                    assert!(res.is_err());
                }
                #[test]
                fn on_vec_when_struct_is_not_defined_as_vec() {
                    let string_value_json = r#"{
                        "string_or_struct": [{
                            "number": 42,
                            "text": "some text"
                        },
                        {
                            "number": 3,
                            "text": "some other text"
                        }]
                    }"#;

                    let res: serde_json::Result<MyType> = serde_json::from_str(string_value_json);

                    assert!(res.is_err());
                }
            }
        }

        mod serialize {
            use super::*;

            #[test]
            fn string_value() {
                let value = StringOrStruct::<SimpleStruct>::String(String::from("Some string"));

                let res = serde_json::to_string(&value);

                assert_eq!(res.unwrap(), "\"Some string\"");
            }

            #[test]
            fn struct_value() {
                let value = StringOrStruct::<SimpleStruct>::Struct(SimpleStruct {
                    number: 912,
                    text: String::from("some text"),
                });

                let res = serde_json::to_string(&value);

                assert_eq!(res.unwrap(), "{\"number\":912,\"text\":\"some text\"}");
            }

            #[test]
            fn struct_as_vec_value() {
                let value = StringOrStruct::<Vec<SimpleStruct>>::Struct(vec![
                    SimpleStruct {
                        number: 912,
                        text: String::from("some text"),
                    },
                    SimpleStruct {
                        number: 100,
                        text: String::from(""),
                    },
                ]);

                let res = serde_json::to_string(&value);

                assert_eq!(
                    res.unwrap(),
                    "[{\"number\":912,\"text\":\"some text\"},{\"number\":100,\"text\":\"\"}]"
                );
            }
        }
    }

    mod string_or_struct_or_vec {
        use super::*;

        mod deserialize {
            use super::*;

            #[test]
            fn string_value() {
                let string_value_json = r#"{
                    "string_or_struct_or_vec": "some string"
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();

                let value = match res.string_or_struct_or_vec.unwrap() {
                    StringOrStructOrVec::String(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(value.unwrap(), "some string");
            }

            #[test]
            fn struct_value() {
                let string_value_json = r#"{
                    "string_or_struct_or_vec": {
                        "number": 0,
                        "text": "abc text"
                    }
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();
                let value = match res.string_or_struct_or_vec.unwrap() {
                    StringOrStructOrVec::Struct(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(
                    value.unwrap(),
                    SimpleStruct {
                        number: 0,
                        text: String::from("abc text")
                    }
                );
            }

            #[test]
            fn vec_value() {
                let string_value_json = r#"{
                    "string_or_struct_or_vec": [
                        {
                            "number": 999,
                            "text": "text1"
                        },
                        {
                            "number": -50,
                            "text": "text2"
                        }
                    ]
                }"#;

                let res: MyType = serde_json::from_str(string_value_json).unwrap();
                let value = match res.string_or_struct_or_vec.unwrap() {
                    StringOrStructOrVec::Vec(v) => Ok(v),
                    _ => Err(eyre!("Wrong deserialize type")),
                };

                assert!(value.is_ok());
                assert_eq!(
                    value.unwrap(),
                    vec![
                        SimpleStruct {
                            number: 999,
                            text: String::from("text1")
                        },
                        SimpleStruct {
                            number: -50,
                            text: String::from("text2")
                        }
                    ]
                );
            }

            mod errors {
                use super::*;

                #[test]
                fn on_number() {
                    let string_value_json = r#"{
                        "string_or_struct_or_vec": 18
                    }"#;

                    let res: serde_json::Result<MyType> = serde_json::from_str(string_value_json);

                    assert!(res.is_err());
                }
                #[test]
                fn on_boolean() {
                    let string_value_json = r#"{
                        "string_or_struct_or_vec": false
                    }"#;

                    let res: serde_json::Result<MyType> = serde_json::from_str(string_value_json);

                    assert!(res.is_err());
                }
            }
        }

        mod serialize {
            use super::*;

            #[test]
            fn string_value() {
                let value = StringOrStructOrVec::<SimpleStruct, Vec<SimpleStruct>>::String(
                    String::from("Some string"),
                );

                let res = serde_json::to_string(&value);

                assert_eq!(res.unwrap(), "\"Some string\"");
            }

            #[test]
            fn struct_value() {
                let value =
                    StringOrStructOrVec::<SimpleStruct, Vec<SimpleStruct>>::Struct(SimpleStruct {
                        number: 912,
                        text: String::from("some text"),
                    });

                let res = serde_json::to_string(&value);

                assert_eq!(res.unwrap(), "{\"number\":912,\"text\":\"some text\"}");
            }

            #[test]
            fn vec_value() {
                let value = StringOrStructOrVec::<SimpleStruct, Vec<SimpleStruct>>::Vec(vec![
                    SimpleStruct {
                        number: 912,
                        text: String::from("some text"),
                    },
                    SimpleStruct {
                        number: 100,
                        text: String::from(""),
                    },
                ]);

                let res = serde_json::to_string(&value);

                assert_eq!(
                    res.unwrap(),
                    "[{\"number\":912,\"text\":\"some text\"},{\"number\":100,\"text\":\"\"}]"
                );
            }
        }
    }
}
