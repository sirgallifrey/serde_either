
#[derive(Debug, PartialEq)]
pub enum StringOrStruct<S> {
    String(String),
    Struct(S),
}

impl<S: Clone> Clone for StringOrStruct<S> {
    fn clone(&self) -> Self {
        match self {
            Self::String(as_string) => Self::String(as_string.clone()),
            Self::Struct(as_struct) => Self::Struct(as_struct.clone()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StringOrStructOrVec<S, V> {
    String(String),
    Struct(S),
    Vec(V),
}

impl<S: Clone, V: Clone> Clone for StringOrStructOrVec<S, V> {
    fn clone(&self) -> Self {
        match self {
            Self::String(as_string) => Self::String(as_string.clone()),
            Self::Struct(as_struct) => Self::Struct(as_struct.clone()),
            Self::Vec(as_vec) => Self::Vec(as_vec.clone()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SingleOrVec<S> {
    Single(S),
    Vec(Vec<S>)
}

impl<S: Clone> Clone for SingleOrVec<S> {
    fn clone(&self) -> Self {
        match self {
            Self::Single(as_single) => Self::Single(as_single.clone()),
            Self::Vec(as_vec) => Self::Vec(as_vec.clone()),
        }
    }
}
