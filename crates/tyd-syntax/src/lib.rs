use chumsky::span::SimpleSpan;

pub mod ast;
pub mod parser;
pub mod visitor;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::error::*;
    pub use crate::parser::*;
    pub use crate::visitor::*;
}

pub type Span = SimpleSpan<usize>;

pub mod error {
    use miette::Diagnostic;
    use std::io;
    use thiserror::Error;

    use crate::parser::error::SyntaxErrors;

    #[derive(Debug, Error, Diagnostic)]
    pub enum SyntaxError {
        #[diagnostic(transparent)]
        #[error(transparent)]
        Parse(#[from] SyntaxErrors),
        #[error(transparent)]
        #[diagnostic(code(type_down::TydError::Io))]
        Io(#[from] io::Error),
    }
}
