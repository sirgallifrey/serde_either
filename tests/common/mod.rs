use serde_either::{StringOrStruct, StringOrStructOrVec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SimpleStruct {
    pub number: i32,
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MyType {
    pub string_or_struct: Option<StringOrStruct<SimpleStruct>>,
    pub string_or_struct_with_vec: Option<StringOrStruct<Vec<SimpleStruct>>>,
    pub string_or_struct_with_vec_of_u8: Option<StringOrStruct<Vec<u8>>>,
    pub string_or_struct_or_vec: Option<StringOrStructOrVec<SimpleStruct, Vec<SimpleStruct>>>,
}