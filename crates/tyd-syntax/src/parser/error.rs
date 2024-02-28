use std::io;

use chumsky::error::Rich;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::lexer::{error::LexErrors, node::Node};

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

#[derive(Error, Debug, Diagnostic)]
#[error("Parsing failed with the following errors:")]
#[diagnostic()]
pub struct ParseErrors {
    #[source_code]
    pub src: NamedSource<String>,
    #[related]
    pub related: Vec<ParseError>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(tyd_syntax::parser), url(docsrs), help("Please read the Book"))]
pub struct ParseError {
    #[label("This bit here")]
    pub span: SourceSpan,
    pub msg: String,
}

impl<'src> From<Rich<'src, char>> for ParseError {
    fn from(e: Rich<'src, char>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();

        Self { span, msg }
    }
}

impl<'tokens, 'src: 'tokens> From<Rich<'tokens, Node<'src>>> for ParseError {
    fn from(e: Rich<'tokens, Node<'src>>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();

        Self { span, msg }
    }
}
