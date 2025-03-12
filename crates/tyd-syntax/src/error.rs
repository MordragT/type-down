use chumsky::error::Rich;
use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

use crate::source::Source;

pub type SyntaxResult<T> = Result<T, SyntaxErrors>;

#[derive(Error, Debug, Diagnostic)]
#[error("Parsing failed with the following errors:")]
#[diagnostic()]
pub struct SyntaxErrors {
    #[source_code]
    pub src: Source,
    #[related]
    pub related: Vec<SyntaxError>,
}

impl SyntaxErrors {
    pub fn new(src: Source) -> Self {
        Self {
            src,
            related: Vec::new(),
        }
    }

    pub fn with_related(src: Source, related: Vec<SyntaxError>) -> Self {
        Self { src, related }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(tyd_syntax::parser), url(docsrs), help("Please read the Book"))]
pub struct SyntaxError {
    #[label("This here")]
    pub span: SourceSpan,
    pub msg: String,
    #[label(collection, "Related to this")]
    pub trace: Vec<LabeledSpan>,
}

impl<'src> From<Rich<'src, char>> for SyntaxError {
    fn from(e: Rich<'src, char>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();
        let trace = e
            .contexts()
            .map(|(label, span)| {
                LabeledSpan::new_with_span(Some(label.to_string()), span.into_range())
            })
            .collect();

        Self { span, msg, trace }
    }
}
