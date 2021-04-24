mod common;

use crate::common::SimpleStruct;
use serde_either::{StringOrStruct, StringOrStructOrVec};
use serde_json;

mod string_or_struct {
    use super::*;

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
