use core::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub enum Type {
    Map(Vec<(String, Self)>),
    List(Box<Self>),
    Bool,
    Str,
    Float,
    Int,
    Inline,
    Block,
    Any,
    None,
}

impl Type {
    pub fn list(ty: Self) -> Self {
        Self::List(Box::new(ty))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Map(map) => {
                write!(f, "Map {{")?;
                for (name, ty) in map {
                    write!(f, "{name}: {ty}")?;
                }
                write!(f, "}}")
            }
            Type::List(ty) => write!(f, "List {ty}"),
            Type::Bool => write!(f, "Bool"),
            Type::Str => write!(f, "Str"),
            Type::Float => write!(f, "Float"),
            Type::Int => write!(f, "Int"),
            Type::Inline => write!(f, "Inline"),
            Type::Block => write!(f, "Block"),
            Type::Any => write!(f, "Any"),
            Type::None => write!(f, "None"),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;

        match (self, other) {
            (Map(x), Map(y)) => x == y,
            (List(x), List(y)) => x == y,
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
