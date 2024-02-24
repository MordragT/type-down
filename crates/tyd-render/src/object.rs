use crate::Map;
use tyd_syntax::ast::{Blocks, Elements};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Map(Map<String, Object>),
    List(Vec<Object>),
    Bool(bool),
    Str(String),
    Blocks(Blocks),
    Elements(Elements),
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl<'a> From<&'a str> for Object {
    fn from(value: &'a str) -> Self {
        Self::Str(value.to_owned())
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<Object>> for Object {
    fn from(value: Vec<Object>) -> Self {
        Self::List(value)
    }
}

impl From<Map<String, Object>> for Object {
    fn from(value: Map<String, Object>) -> Self {
        Self::Map(value)
    }
}

impl From<Blocks> for Object {
    fn from(value: Blocks) -> Self {
        Self::Blocks(value)
    }
}

impl From<Elements> for Object {
    fn from(value: Elements) -> Self {
        Self::Elements(value)
    }
}
