use derive_more::From;
use ecow::EcoString;
use std::fmt::Debug;

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
        match self {
            Self::Map(map) => {
                let inner = map
                    .iter()
                    .map(|(name, val)| (name.clone(), val.ty()))
                    .collect();

                Type::Map(inner)
            }
            Self::List(list) => {
                if list.is_empty() {
                    Type::list(Type::Any)
                } else {
                    Type::list(list.first().unwrap().ty())
                }
            }
            Self::Content(_) => Type::Content,
            Self::Bool(_) => Type::Bool,
            Self::Str(_) => Type::Str,
            Self::Float(_) => Type::Float,
            Self::Int(_) => Type::Int,
            Self::Inline(_) => Type::Inline,
            Self::Block(_) => Type::Block,
            Self::Func(_) => Type::Func,
            Self::None => Type::None,
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(value.into())
    }
}

pub trait TypeCast: Sized + Clone {
    fn upcast(self) -> Value;
    fn try_downcast(v: Value) -> Result<Self, Type>;
    fn try_downcast_ref(v: &Value) -> Result<&Self, Type>;
    fn try_downcast_mut(v: &mut Value) -> Result<&mut Self, Type>;

    fn try_downcast_cloned(v: &Value) -> Result<Self, Type> {
        Self::try_downcast_ref(v).cloned()
    }
}

macro_rules! impl_type_cast {
    ($($variant:ident($variant_type:ty)),*) => {
        $(
            impl TypeCast for $variant_type {
                fn upcast(self) -> Value {
                    Value::$variant(self)
                }

                fn try_downcast(v: Value) -> Result<Self, Type> {
                    match v {
                        Value::$variant(val) => Ok(val),
                        _ => Err(v.ty()),
                    }
                }

                fn try_downcast_ref(v: &Value) -> Result<&Self, Type> {
                    match v {
                        Value::$variant(val) => Ok(val),
                        _ => Err(v.ty()),
                    }
                }

                fn try_downcast_mut(v: &mut Value) -> Result<&mut Self, Type> {
                    match v {
                        Value::$variant(val) => Ok(val),
                        _ => Err(v.ty()),
                    }
                }
            }
        )*
    };
}

impl_type_cast!(
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

impl TypeCast for Value {
    fn upcast(self) -> Value {
        self
    }

    fn try_downcast(v: Value) -> Result<Self, Type> {
        Ok(v)
    }

    fn try_downcast_ref(v: &Value) -> Result<&Self, Type> {
        Ok(v)
    }

    fn try_downcast_mut(v: &mut Value) -> Result<&mut Self, Type> {
        Ok(v)
    }
}
