use miette::{Diagnostic, NamedSource, SourceSpan};
use parasite::chumsky::error::Simple;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum TydError {
    #[diagnostic(transparent)]
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    #[diagnostic(code(type_down::TydError::Io))]
    Io(#[from] io::Error),
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parsing failed with the following errors:")]
#[diagnostic()]
pub struct ParseError {
    #[source_code]
    pub src: NamedSource<String>,
    #[related]
    pub related: Vec<SimpleError>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(type_down::parse), url(docsrs), help("Please read the Book"))]
pub struct SimpleError {
    #[label("This bit here")]
    pub span: SourceSpan,
    pub msg: String,
}

impl From<Simple<char>> for SimpleError {
    fn from(e: Simple<char>) -> Self {
        let span = SourceSpan::from(e.span());
        let msg = e.to_string();

        Self { span, msg }
    }
}
