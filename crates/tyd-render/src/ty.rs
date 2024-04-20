use core::fmt;
use std::{any::type_name, collections::BTreeMap, fmt::Debug, marker::PhantomData};

use crate::Value;

pub trait Shape: Debug + Copy + Clone {
    type Inline: Debug + Clone + 'static;
    type Block: Debug + Clone + 'static;
}

pub struct TypeChecker<S: Shape> {
    s: PhantomData<S>,
}

impl<S: Shape> TypeChecker<S> {
    pub fn check<T: ?Sized>(ty: Type) -> bool {
        use Type::*;

        let t = type_name::<T>();

        match ty {
            Map => t == type_name::<BTreeMap<String, Value<S>>>(),
            List => t == type_name::<Vec<Value<S>>>(),
            Bool => t == type_name::<bool>(),
            Str => t == type_name::<String>(),
            Float => t == type_name::<f64>(),
            Int => t == type_name::<i64>(),
            Inline => t == type_name::<S::Inline>(),
            Block => t == type_name::<S::Block>(),
            // TODO maybe match against all the valid ones before
            Any => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
pub enum Type {
    Map,
    List,
    Bool,
    Str,
    Float,
    Int,
    Inline,
    Block,
    Any,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Map => write!(f, "Map"),
            Type::List => write!(f, "List"),
            Type::Bool => write!(f, "Bool"),
            Type::Str => write!(f, "Str"),
            Type::Float => write!(f, "Float"),
            Type::Int => write!(f, "Int"),
            Type::Inline => write!(f, "Inline"),
            Type::Block => write!(f, "Block"),
            Type::Any => write!(f, "Any"),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;

        match (self, other) {
            (Map, Map) => true,
            (List, List) => true,
            (Bool, Bool) => true,
            (Str, Str) => true,
            (Float, Float) => true,
            (Int, Int) => true,
            (Inline, Inline) => true,
            (Block, Block) => true,
            (Any, _) => true,
            (_, Any) => true,
            _ => false,
        }
    }
}

impl Eq for Type {}
