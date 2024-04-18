use chumsky::error::Rich;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub type SyntaxResult<T> = Result<T, SyntaxErrors>;

#[derive(Error, Debug, Diagnostic)]
#[error("Parsing failed with the following errors:")]
#[diagnostic()]
pub struct SyntaxErrors {
    #[source_code]
    pub src: NamedSource<String>,
    #[related]
    pub related: Vec<SyntaxError>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(tyd_syntax::parser), url(docsrs), help("Please read the Book"))]
pub struct SyntaxError {
    #[label("This bit here")]
    pub span: SourceSpan,
    pub msg: String,
}

impl<'src> From<Rich<'src, char>> for SyntaxError {
    fn from(e: Rich<'src, char>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();

        Self { span, msg }
    }
}
