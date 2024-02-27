use std::{collections::BTreeMap, fmt};

use tyd_syntax::code::Literal;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value<C> {
    Map(BTreeMap<String, Value<C>>),
    List(Vec<Value<C>>),
    Bool(bool),
    Str(String),
    Float(f64),
    Int(i64),
    Content(C),
}

impl<C> Value<C> {
    pub fn kind(&self) -> ValueKind {
        use ValueKind::*;

        match self {
            Self::Map(_) => Map,
            Self::List(_) => List,
            Self::Bool(_) => Bool,
            Self::Str(_) => Str,
            Self::Float(_) => Float,
            Self::Int(_) => Int,
            Self::Content(_) => Content,
        }
    }

    pub fn into_map(self) -> Option<BTreeMap<String, Value<C>>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<Vec<Value<C>>> {
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

    pub fn into_int(self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn into_float(self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn into_content(self) -> Option<C> {
        match self {
            Self::Content(c) => Some(c),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueKind {
    Map,
    List,
    Bool,
    Str,
    Float,
    Int,
    Content,
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueKind::Map => write!(f, "Map"),
            ValueKind::List => write!(f, "List"),
            ValueKind::Bool => write!(f, "Bool"),
            ValueKind::Str => write!(f, "Str"),
            ValueKind::Float => write!(f, "Float"),
            ValueKind::Int => write!(f, "Int"),
            ValueKind::Content => write!(f, "Content"),
        }
    }
}

impl<C> From<String> for Value<C> {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl<'a, C> From<&'a str> for Value<C> {
    fn from(value: &'a str) -> Self {
        Self::Str(value.to_owned())
    }
}

impl<C> From<bool> for Value<C> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<C> From<Vec<Value<C>>> for Value<C> {
    fn from(value: Vec<Value<C>>) -> Self {
        Self::List(value)
    }
}

impl<C> From<BTreeMap<String, Value<C>>> for Value<C> {
    fn from(value: BTreeMap<String, Value<C>>) -> Self {
        Self::Map(value)
    }
}

impl<C> From<i64> for Value<C> {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl<C> From<f64> for Value<C> {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

// impl<C> From<C> for Value<C> {
//     fn from(value: C) -> Self {
//         Self::Content(value)
//     }
// }

impl<'a, C> From<Literal<'a>> for Value<C> {
    fn from(value: Literal<'a>) -> Self {
        match value {
            Literal::Boolean(b) => Self::Bool(b),
            Literal::Int(i) => Self::Int(i),
            Literal::Str(s) => Self::Str(s.to_owned()),
        }
    }
}
