
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

#[derive(Debug, PartialEq)]
pub enum SingleOrVec<S> {
    Single(S),
    Vec(Vec<S>)
}