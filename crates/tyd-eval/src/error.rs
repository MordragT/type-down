use ecow::EcoString;
use thiserror::Error;

use crate::value::Type;

/// Errors that can occur during engine operations.
#[derive(Debug, Clone, Error)]
pub enum EngineError {
    /// Error when an inline element was expected but not found.
    #[error("Expected element of type 'Inline'")]
    ExpectedInline,
}

/// Errors related to symbol resolution.
#[derive(Debug, Clone, Error)]
pub enum SymbolError {
    /// Error when a referenced symbol cannot be found.
    ///
    /// # Arguments
    ///
    /// * `0` - The name of the symbol that wasn't found.
    #[error("Symbol '{0}' not found")]
    NotFound(EcoString),
}

/// Errors related to function or method arguments.
#[derive(Debug, Clone, Error)]
pub enum ArgumentError {
    /// Error when a required named argument is missing.
    ///
    /// # Fields
    ///
    /// * `name` - The name of the missing argument.
    /// * `ty` - The expected type of the missing argument.
    #[error("Missing Argument {name}: {ty}")]
    MissingRequired { name: EcoString, ty: Type },

    /// Error when a required positional argument is missing.
    ///
    /// # Fields
    ///
    /// * `pos` - The position of the missing argument.
    /// * `ty` - The expected type of the missing argument.
    #[error("Missing Argument at {pos} of {ty}")]
    MissingPositional { pos: usize, ty: Type },

    /// Error when an unknown named argument is provided.
    ///
    /// # Fields
    ///
    /// * `name` - The name of the unknown argument.
    #[error("Unknown Argument {name}")]
    UnknownNamed { name: EcoString },

    /// Error when an unknown positional argument is provided.
    ///
    /// # Fields
    ///
    /// * `pos` - The position of the unknown argument.
    #[error("Unknown Argument at {pos}")]
    UnknownPositional { pos: usize },
}

/// Errors related to type mismatches.
#[derive(Debug, Clone, Error)]
pub enum TypeError {
    /// Error when a value has the wrong type.
    ///
    /// # Fields
    ///
    /// * `got` - The actual type that was provided.
    /// * `expected` - The type that was expected.
    #[error("Wrong type of {got}, expected: {expected}")]
    WrongType { got: Type, expected: Type },
}
