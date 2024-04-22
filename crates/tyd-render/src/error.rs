use std::{path::PathBuf, sync::Arc};

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use tyd_syntax::Span;

use crate::ty::Type;

pub type EngineResult<T> = Result<T, Vec<EngineError>>;

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
    pub source_span: SourceSpan,
    pub msg: EngineMessage,
    pub span: Span,
}

impl EngineError {
    pub fn new(span: Span, msg: EngineMessage) -> Self {
        Self {
            msg,
            source_span: SourceSpan::from(span.into_range()),
            span,
        }
    }

    pub fn arg(span: Span, msg: ArgumentError) -> Self {
        Self {
            msg: EngineMessage::Argument(msg),
            source_span: SourceSpan::from(span.into_range()),
            span,
        }
    }
}

#[derive(Clone, Error, Debug, Diagnostic)]
pub enum EngineMessage {
    #[error(transparent)]
    Argument(#[from] ArgumentError),
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),
    #[error("Symbol '{0}' not found")]
    SymbolNotFound(String),
    #[error("File '{0}' not found")]
    FileNotFound(PathBuf),
    #[error("Expected element of type 'Inline'")]
    ExpectedInline,
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
