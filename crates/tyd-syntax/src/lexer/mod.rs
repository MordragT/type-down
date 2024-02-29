use chumsky::{error::Rich, extra, Parser};
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

type Extra<'src> = extra::Full<Rich<'src, char, Span>, usize, ()>;

pub fn lex<'src>(src: &'src str, name: impl AsRef<str>) -> Result<Vec<Node<'src>>, LexErrors> {
    let parser = nodes_parser();
    let mut indent_level = 0;

    let nodes = parser
        .parse_with_state(&src, &mut indent_level)
        .into_result()
        .map_err(|errs| LexErrors {
            src: NamedSource::new(name, src.to_owned()),
            related: errs.into_iter().map(Into::into).collect(),
        })?;

    Ok(nodes)
}

pub fn lex_spanned<'src>(
    src: &'src str,
    name: impl AsRef<str>,
) -> Result<Vec<(Node<'src>, Span)>, LexErrors> {
    let parser = nodes_spanned_parser();
    let mut indent_level = 0;

    let nodes = parser
        .parse_with_state(&src, &mut indent_level)
        .into_result()
        .map_err(|errs| LexErrors {
            src: NamedSource::new(name, src.to_owned()),
            related: errs.into_iter().map(Into::into).collect(),
        })?;

    Ok(nodes)
}
