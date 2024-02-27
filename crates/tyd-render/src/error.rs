use miette::Diagnostic;
use thiserror::Error;

use crate::ValueKind;

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(tyd_render::Context::call))]
pub enum ContextError {
    #[error("{0}")]
    Message(String),
    #[error("Missing Argument {0}")]
    MissingArgument(String),
    #[error("Wrong Type for argument {arg}. Expected {expected}")]
    WrongArgType { arg: String, expected: ValueKind },
    #[error("Wrong Arguments")]
    WrongArguments,
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),
    #[error("Symbol '{0}' not found")]
    SymbolNotFound(String),
}
