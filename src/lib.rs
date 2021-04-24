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

mod enums;
mod de;
mod se;

pub use enums::*;