use chumsky::Parser;
use miette::NamedSource;

use crate::Span;

use self::{
    error::LexErrors,
    markup::{nodes_parser, nodes_spanned_parser},
    node::Node,
};

pub mod code;
pub mod error;
pub mod markup;
pub mod node;

pub fn lex<'src>(src: &'src str, name: impl AsRef<str>) -> Result<Vec<Node<'src>>, LexErrors> {
    let parser = nodes_parser();

    let cst = parser.parse(&src).into_result().map_err(|errs| LexErrors {
        src: NamedSource::new(name, src.to_owned()),
        related: errs.into_iter().map(Into::into).collect(),
    })?;

    Ok(cst)
}

pub fn lex_spanned<'src>(
    src: &'src str,
    name: impl AsRef<str>,
) -> Result<Vec<(Node<'src>, Span)>, LexErrors> {
    let parser = nodes_spanned_parser();

    let nodes = parser.parse(&src).into_result().map_err(|errs| LexErrors {
        src: NamedSource::new(name, src.to_owned()),
        related: errs.into_iter().map(Into::into).collect(),
    })?;

    Ok(nodes)
}
