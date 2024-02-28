use chumsky::span::SimpleSpan;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod visitor;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::error::*;
    pub use crate::lexer::*;
    pub use crate::parser::parse;
    pub use crate::visitor::*;
}

pub type Span = SimpleSpan<usize>;

pub mod error {
    use miette::Diagnostic;
    use std::io;
    use thiserror::Error;

    use crate::{lexer::error::LexErrors, parser::error::ParseErrors};

    #[derive(Debug, Error, Diagnostic)]
    pub enum SyntaxError {
        #[diagnostic(transparent)]
        #[error(transparent)]
        Parse(#[from] ParseErrors),
        #[diagnostic(transparent)]
        #[error(transparent)]
        Lex(#[from] LexErrors),
        #[error(transparent)]
        #[diagnostic(code(type_down::TydError::Io))]
        Io(#[from] io::Error),
    }
}
