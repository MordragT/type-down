use chumsky::error::Rich;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::io;
use thiserror::Error;

use crate::cst::Node;

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
    pub related: Vec<RichError>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(code(type_down::parse), url(docsrs), help("Please read the Book"))]
pub struct RichError {
    #[label("This bit here")]
    pub span: SourceSpan,
    pub msg: String,
}

impl<'src> From<Rich<'src, char>> for RichError {
    fn from(e: Rich<'src, char>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();

        Self { span, msg }
    }
}

impl<'tokens, 'src: 'tokens> From<Rich<'tokens, Node<'src>>> for RichError {
    fn from(e: Rich<'tokens, Node<'src>>) -> Self {
        let span = SourceSpan::from(e.span().into_range());
        let msg = e.to_string();

        Self { span, msg }
    }
}

// impl From<Simple<char>> for SimpleError {
//     fn from(e: Simple<char>) -> Self {
//         let span = SourceSpan::from(e.span());
//         let msg = e.to_string();

//         Self { span, msg }
//     }
// }

// impl<'src> From<Simple<&'src str>> for SimpleError {
//     fn from(e: Simple<&'src str>) -> Self {
//         let span = SourceSpan::from(e.span());
//         let msg = e.to_string();

//         Self { span, msg }
//     }
// }
