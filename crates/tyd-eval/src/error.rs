use ecow::EcoString;
use thiserror::Error;

use crate::ty::Type;

#[derive(Debug, Clone, Error)]
pub enum EngineError {
    #[error("Expected element of type 'Inline'")]
    ExpectedInline,
}

#[derive(Debug, Clone, Error)]
pub enum SymbolError {
    #[error("Symbol '{0}' not found")]
    NotFound(EcoString),
}

#[derive(Debug, Clone, Error)]
pub enum ArgumentError {
    #[error("Missing Argument {name}: {ty}")]
    MissingRequired { name: EcoString, ty: Type },
    #[error("Missing Argument at {pos} of {ty}")]
    MissingPositional { pos: usize, ty: Type },
    #[error("Unknown Argument {name}")]
    UnknownNamed { name: EcoString },
    #[error("Unknown Argument at {pos}")]
    UnknownPositional { pos: usize },
}

#[derive(Debug, Clone, Error)]
pub enum TypeError {
    #[error("Wrong type of {got}, expected: {expected}")]
    WrongType { got: Type, expected: Type },
}
