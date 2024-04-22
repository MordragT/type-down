use ecow::EcoString;
use std::{collections::BTreeMap, fmt::Debug};
use tyd_syntax::ast::Literal;

use crate::ty::Type;

pub trait Shape: Debug + Copy + Clone {
    type Inline: Debug + Clone + 'static + Cast<Self>;
    type Block: Debug + Clone + 'static + Cast<Self>;
}

pub trait Cast<S: Shape> {
    fn cast(value: Value<S>) -> Self;
}

impl<S: Shape> Cast<S> for BTreeMap<String, Value<S>> {
    fn cast(value: Value<S>) -> Self {
        value.into_map().unwrap()
    }
}

impl<S: Shape, T: Cast<S>> Cast<S> for Vec<T> {
    fn cast(value: Value<S>) -> Self {
        value
            .into_list()
            .unwrap()
            .into_iter()
            .map(T::cast)
            .collect()
    }
}

impl<S: Shape> Cast<S> for bool {
    fn cast(value: Value<S>) -> Self {
        value.into_bool().unwrap()
    }
}

impl<S: Shape> Cast<S> for String {
    fn cast(value: Value<S>) -> Self {
        value.into_string().unwrap()
    }
}

impl<S: Shape> Cast<S> for f64 {
    fn cast(value: Value<S>) -> Self {
        value.into_float().unwrap()
    }
}

impl<S: Shape> Cast<S> for i64 {
    fn cast(value: Value<S>) -> Self {
        value.into_int().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value<S: Shape> {
    Map(BTreeMap<String, Value<S>>),
    List(Vec<Value<S>>),
    Bool(bool),
    Str(String),
    Float(f64),
    Int(i64),
    Inline(S::Inline),
    Block(S::Block),
    None,
}

impl<S: Shape> Value<S> {
    pub fn ty(&self) -> Type {
        use Type::*;

        match self {
            Self::Map(map) => {
                let inner = map
                    .iter()
                    .map(|(name, val)| (name.clone(), val.ty()))
                    .collect();

                Map(inner)
            }
            Self::List(list) => {
                if list.is_empty() {
                    List(Box::new(Any))
                } else {
                    List(Box::new(list.first().unwrap().ty()))
                }
            }
            Self::Bool(_) => Bool,
            Self::Str(_) => Str,
            Self::Float(_) => Float,
            Self::Int(_) => Int,
            Self::Inline(_) => Inline,
            Self::Block(_) => Block,
            Self::None => None,
        }
    }

    pub fn into_map(self) -> Option<BTreeMap<String, Value<S>>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<Vec<Value<S>>> {
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

    pub fn into_inline(self) -> Option<S::Inline> {
        match self {
            Self::Inline(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_block(self) -> Option<S::Block> {
        match self {
            Self::Block(c) => Some(c),
            _ => None,
        }
    }
}

impl<S: Shape> From<EcoString> for Value<S> {
    fn from(value: EcoString) -> Self {
        Self::Str(String::from(value))
    }
}

impl<S: Shape> From<String> for Value<S> {
    fn from(value: String) -> Self {
        Self::Str(String::from(value))
    }
}

impl<'a, S: Shape> From<&'a str> for Value<S> {
    fn from(value: &'a str) -> Self {
        Self::Str(String::from(value))
    }
}

impl<S: Shape> From<bool> for Value<S> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<S: Shape> From<Vec<Value<S>>> for Value<S> {
    fn from(value: Vec<Value<S>>) -> Self {
        Self::List(value)
    }
}

impl<S: Shape> From<BTreeMap<String, Value<S>>> for Value<S> {
    fn from(value: BTreeMap<String, Value<S>>) -> Self {
        Self::Map(value)
    }
}

impl<S: Shape> From<i64> for Value<S> {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl<S: Shape> From<f64> for Value<S> {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

// impl<S: ContentShape> From<S::Inline> for Value<S> {
//     fn from(value: S::Inline) -> Self {
//         Self::Inline(value)
//     }
// }

// impl<S: ContentShape> From<S::Block> for Value<S> {
//     fn from(value: S::Block) -> Self {
//         Self::Block(value)
//     }
// }

impl<'a, S: Shape> From<Literal> for Value<S> {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Boolean(b) => Self::Bool(b),
            Literal::Int(i) => Self::Int(i),
            Literal::Str(s) => Self::Str(s.to_string()),
        }
    }
}
