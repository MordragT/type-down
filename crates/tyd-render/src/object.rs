use std::{collections::BTreeMap, fmt};

use tyd_syntax::ast::{Block, Element};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Map(BTreeMap<String, Object>),
    List(Vec<Object>),
    Bool(bool),
    Str(String),
    Block(Block),
    Element(Element),
}

impl Object {
    pub fn kind(&self) -> ObjectKind {
        use ObjectKind::*;

        match self {
            Self::Map(_) => Map,
            Self::List(_) => List,
            Self::Bool(_) => Bool,
            Self::Str(_) => Str,
            Self::Block(_) => Block,
            Self::Element(_) => Element,
        }
    }

    pub fn into_map(self) -> Option<BTreeMap<String, Object>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<Vec<Object>> {
        match self {
            Self::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_block(self) -> Option<Block> {
        match self {
            Self::Block(b) => Some(b),
            _ => None,
        }
    }

    pub fn into_element(self) -> Option<Element> {
        match self {
            Self::Element(el) => Some(el),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectKind {
    Map,
    List,
    Bool,
    Str,
    Block,
    Element,
}

impl fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectKind::Map => write!(f, "Map"),
            ObjectKind::List => write!(f, "List"),
            ObjectKind::Bool => write!(f, "Bool"),
            ObjectKind::Str => write!(f, "Str"),
            ObjectKind::Block => write!(f, "Block"),
            ObjectKind::Element => write!(f, "Element"),
        }
    }
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

impl From<BTreeMap<String, Object>> for Object {
    fn from(value: BTreeMap<String, Object>) -> Self {
        Self::Map(value)
    }
}

impl From<Block> for Object {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}

impl From<Element> for Object {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}
