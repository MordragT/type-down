use chumsky::error::Rich;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub type LexResult<T> = Result<T, LexError>;

#[derive(Error, Debug, Diagnostic)]
#[error("Lexing failed with the following errors:")]
#[diagnostic()]
pub struct LexErrors {
    #[source_code]
    pub src: NamedSource<String>,
    #[related]
    pub related: Vec<LexError>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(tyd_syntax::lexer), url(docsrs), help("Please read the Book"))]
pub struct LexError {
    #[label("This bit here")]
    pub span: SourceSpan,
    pub msg: String,
}

impl<'src> From<Rich<'src, char>> for LexError {
    fn from(e: Rich<'src, char>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();

        Self { span, msg }
    }
}
