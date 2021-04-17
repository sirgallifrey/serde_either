# Serde Either

serde_either is a simple library that provides a few enums that enables you to deserialize and serialize values that can have string, struct or vec forms.

Let's say you want to deserialize a json for a Book, and that json has an "authors" field and this files can be either a string or a object like:

```json
{
  "authors": "John Smith"
}
```

Or like

```json
{
  "authors": {
    "first_name": "John",
    "last_name": "Smith"
  }
}
```

All you need to do is create a data structure like this:

```rust
use serde::{Serialize, Deserialize};
use serde_either::StringOrStruct;
use serde_json;

#[derive(Serialize, Deserialize)]
struct Authors {
  first_name: String,
  last_name: String
}

#[derive(Serialize, Deserialize)]
struct Book {
  pub authors: StringOrStruct<Authors>
}

// And StringOrStruct is just a normal enum

impl Book {
  fn get_author_name(&self) -> String {
    match &self.authors {
      StringOrStruct::String(s) => s.to_owned(),
      StringOrStruct::Struct(author) => format!("{} {}", &author.first_name, &author.last_name)
    }
  }
}

let books = r#"[
    {
        "authors": {
            "first_name": "John",
            "last_name": "Smith"
        }
    },
    {
        "authors": "Michael J. Smith"
    }
]"#;
let res: Vec<Book> = serde_json::from_str(books).unwrap();
assert_eq!(res[0].get_author_name(), "John Smith");
assert_eq!(res[1].get_author_name(), "Michael J. Smith");
```

And if you need to also capture a possible array of authors like this:

```json
{
  "authors": [
    {
      "first_name": "John",
      "last_name": "Smith"
    },
    {
      "first_name": "Michael",
      "last_name": "Smith"
    },
  ]
}
```

Then all you need to change is:

```rust
use serde_either::StringOrStructOrVec;

// ...

#[derive(Serialize, Deserialize)]
struct Book {
  pub authors: StringOrStructOrVec<Authors, Vec<Authors>>
}
```

# License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
