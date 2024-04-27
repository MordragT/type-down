use ecow::EcoString;
use std::{collections::BTreeMap, fmt::Debug, sync::Arc};

use crate::{
    eval::Engine,
    foundations::{Arg, Func, List, Map},
    ty::Type,
};

mod cast;

pub use cast::*;

#[derive(Debug, Clone)]
pub enum Value<E: Engine> {
    Map(Map<E>),
    List(List<E>),
    Bool(bool),
    Str(EcoString),
    Float(f64),
    Int(i64),
    Inline(E::Inline),
    Block(E::Block),
    None,
    Arg(Arc<Arg<E>>),
    Func(Arc<dyn Func<E>>),
}

impl<E: Engine> Value<E> {
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
            Self::Arg(_) => Arg,
            Self::Func(_) => Func,
            Self::None => None,
        }
    }

    pub fn into_map(self) -> Option<Map<E>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<List<E>> {
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

    pub fn into_string(self) -> Option<EcoString> {
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

    pub fn into_inline(self) -> Option<E::Inline> {
        match self {
            Self::Inline(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_block(self) -> Option<E::Block> {
        match self {
            Self::Block(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_func(self) -> Option<Arc<dyn Func<E>>> {
        match self {
            Self::Func(f) => Some(f),
            _ => None,
        }
    }

    pub fn into_arg(self) -> Option<Arc<Arg<E>>> {
        match self {
            Self::Arg(a) => Some(a),
            _ => None,
        }
    }
}

impl<E: Engine, T: Into<Value<E>>> From<Option<T>> for Value<E> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Value::None,
        }
    }
}

impl<E: Engine> From<Arg<E>> for Value<E> {
    fn from(value: Arg<E>) -> Self {
        Self::Arg(Arc::new(value))
    }
}

// impl<E: Engine, F: Func<E>> From<F> for Value<E> {
//     fn from(value: F) -> Self {
//         Self::Func(Box::new(value))
//     }
// }

impl<E: Engine> From<String> for Value<E> {
    fn from(value: String) -> Self {
        Self::Str(EcoString::from(value))
    }
}

impl<'a, E: Engine> From<&'a str> for Value<E> {
    fn from(value: &'a str) -> Self {
        Self::Str(EcoString::from(value))
    }
}

impl<E: Engine> From<bool> for Value<E> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<E: Engine, T: Into<Value<E>>> From<Vec<T>> for Value<E> {
    fn from(value: Vec<T>) -> Self {
        Self::List(Arc::new(value.into_iter().map(Into::into).collect()))
    }
}

impl<E: Engine> From<BTreeMap<EcoString, Value<E>>> for Value<E> {
    fn from(value: BTreeMap<EcoString, Value<E>>) -> Self {
        Self::Map(Arc::new(value))
    }
}

impl<E: Engine> From<i64> for Value<E> {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl<E: Engine> From<f64> for Value<E> {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
