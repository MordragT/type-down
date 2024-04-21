use std::sync::Arc;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use tyd_syntax::Span;

use crate::Type;

pub type EngineResult<T> = Result<T, Vec<EngineError>>;

pub trait EngineErrorHandler {
    fn named_source(&self) -> NamedSource<Arc<str>>;
}

#[derive(Clone, Error, Debug, Diagnostic)]
#[error("Evaluation failed with the following errors:")]
#[diagnostic()]
pub struct EngineErrors {
    #[source_code]
    pub src: NamedSource<Arc<str>>,
    #[related]
    pub related: Vec<EngineError>,
}

#[derive(Clone, Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(tyd_render::engine), url(docsrs), help("Please read the Book"))]
pub struct EngineError {
    #[label("This bit here")]
    pub span: SourceSpan,
    pub msg: EngineErrorMessage,
}

impl EngineError {
    pub fn new(span: Span, msg: EngineErrorMessage) -> Self {
        Self {
            msg,
            span: SourceSpan::from(span.into_range()),
        }
    }

    pub fn arg(span: Span, msg: ArgumentError) -> Self {
        Self {
            msg: EngineErrorMessage::Argument(msg),
            span: SourceSpan::from(span.into_range()),
        }
    }
}

#[derive(Clone, Error, Debug, Diagnostic)]
pub enum EngineErrorMessage {
    #[error(transparent)]
    Argument(#[from] ArgumentError),
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),
    #[error("Symbol '{0}' not found")]
    SymbolNotFound(String),
    #[error("{0}")]
    Message(String),
}

#[derive(Clone, Error, Debug, Diagnostic)]
pub enum ArgumentError {
    #[error("Missing Argument {name}: {ty}")]
    MissingRequired { name: String, ty: Type },
    #[error("Missing Argument at {pos} of {ty}")]
    MissingPositional { pos: usize, ty: Type },
    #[error("Unknown Argument {name}")]
    UnknownNamed { name: String },
    #[error("Unknown Argument at {pos}")]
    UnknownPositional { pos: usize },
    #[error("Wrong Argument type of {got}, expected: {expected}")]
    WrongType { got: Type, expected: Type },
}
