use derive_more::From;
use ecow::EcoString;
use std::fmt::Debug;
use tyd_util::{impl_try_as_ref, impl_try_from, TryAsRef};

use crate::{ir, ty::Type};

#[derive(Debug, Clone, From)]
pub enum Value {
    Map(ir::Map),
    List(ir::List),
    Bool(bool),
    Str(EcoString),
    Float(f64),
    Int(i64),
    Inline(ir::Inline),
    Block(ir::Block),
    Content(ir::Content),
    Func(ir::Func),
    None,
}

impl Value {
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
                    Type::list(Any)
                } else {
                    Type::list(list.first().unwrap().ty())
                }
            }
            Self::Content(_) => Type::Content,
            Self::Bool(_) => Bool,
            Self::Str(_) => Str,
            Self::Float(_) => Float,
            Self::Int(_) => Int,
            Self::Inline(_) => Inline,
            Self::Block(_) => Block,
            Self::Func(_) => Func,
            Self::None => None,
        }
    }

    pub fn into_map(self) -> Option<ir::Map> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<ir::List> {
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

    pub fn into_inline(self) -> Option<ir::Inline> {
        match self {
            Self::Inline(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_block(self) -> Option<ir::Block> {
        match self {
            Self::Block(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_content(self) -> Option<ir::Content> {
        match self {
            Self::Content(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_func(self) -> Option<ir::Func> {
        match self {
            Self::Func(f) => Some(f),
            _ => None,
        }
    }
}

impl TryAsRef<Value> for Value {
    fn try_as_ref(&self) -> Option<&Value> {
        Some(self)
    }
}

impl_try_as_ref!(
    Value,
    Map(ir::Map),
    List(ir::List),
    Bool(bool),
    Str(EcoString),
    Float(f64),
    Int(i64),
    Inline(ir::Inline),
    Block(ir::Block),
    Content(ir::Content),
    Func(ir::Func)
);

impl_try_from!(
    Value,
    Map(ir::Map),
    List(ir::List),
    Bool(bool),
    Str(EcoString),
    Float(f64),
    Int(i64),
    Inline(ir::Inline),
    Block(ir::Block),
    Content(ir::Content),
    Func(ir::Func)
);

// impl<T: TryFrom<Value, Error = ()>> TryFrom<Value> for Vec<T> {
//     type Error = ();

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         let list = value.into_list().ok_or(())?;
//         list.into_iter()
//             .map(T::try_from)
//             .collect::<Result<Vec<_>, _>>()
//     }
// }

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Str(value.into())
    }
}

// use ecow::EcoString;

// use crate::ir;

// use super::{Map, Value};

// pub trait Downcast {
//     fn downcast(value: Value) -> Self;
// }

// impl Downcast for Map {
//     fn downcast(value: Value) -> Self {
//         value.into_map().unwrap()
//     }
// }

// impl<T: Downcast> Downcast for Vec<T> {
//     fn downcast(value: Value) -> Self {
//         let list = value.into_list().unwrap();
//         list.iter().cloned().map(T::downcast).collect()
//     }
// }

// impl Downcast for bool {
//     fn downcast(value: Value) -> Self {
//         value.into_bool().unwrap()
//     }
// }

// impl Downcast for EcoString {
//     fn downcast(value: Value) -> Self {
//         value.into_string().unwrap()
//     }
// }

// impl Downcast for f64 {
//     fn downcast(value: Value) -> Self {
//         value.into_float().unwrap()
//     }
// }

// impl Downcast for i64 {
//     fn downcast(value: Value) -> Self {
//         value.into_int().unwrap()
//     }
// }

// impl Downcast for ir::Func {
//     fn downcast(value: Value) -> Self {
//         value.into_func().unwrap()
//     }
// }

// impl Downcast for ir::Block {
//     fn downcast(value: Value) -> Self {
//         value.into_block().unwrap()
//     }
// }

// impl Downcast for ir::Inline {
//     fn downcast(value: Value) -> Self {
//         value.into_inline().unwrap()
//     }
// }
