use serde::{Deserialize, Serialize};
use serde_either::{StringOrStruct, StringOrStructOrVec};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::{error::Error, ops::Deref};

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
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
pub struct PersonFromStrError {
    pub string: String,
}

impl Display for PersonFromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Could not parse Person string: {}",
            &self.string
        ))
    }
}

impl Error for PersonFromStrError {}

impl FromStr for Person {
    type Err = PersonFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() < 2 {
            return Err(PersonFromStrError { string: s.into() });
        }
        Ok(Person {
            first_name: parts[0].into(),
            last_name: parts.last().unwrap().deref().into(),
        })
    }
}
