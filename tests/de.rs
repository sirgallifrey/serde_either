mod common;

use crate::common::{MyType, SimpleStruct};
use eyre::eyre;
use serde_either::{StringOrStruct, StringOrStructOrVec};
use serde_json;

mod string_or_struct {
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

mod string_or_struct_or_vec {
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
