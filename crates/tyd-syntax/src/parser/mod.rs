use chumsky::{error::Rich, input::Input, Parser};
use miette::NamedSource;

use crate::{
    ast::Ast,
    error::SyntaxError,
    lexer::{lex_spanned, node::Node},
    parser::{combinator::ast_parser, error::ParseErrors},
    Span,
};

pub mod combinator;
pub mod error;

pub fn parse<'src>(src: &'src str, name: impl AsRef<str>) -> Result<Ast<'src>, SyntaxError> {
    let nodes = lex_spanned(src, name.as_ref())?;
    let input = nodes.as_slice().spanned((src.len()..src.len()).into());

    let parser = ast_parser();
    let ast = parser
        .parse(input)
        .into_result()
        .map_err(|errs| ParseErrors {
            src: NamedSource::new(name, src.to_owned()),
            related: errs.into_iter().map(Into::into).collect(),
        })?;

    Ok(ast)
}

pub fn parse_nodes<'src, 'tokens>(
    input: &'src [(Node<'src>, Span)],
    src: impl Into<String>,
    name: impl AsRef<str>,
) -> Result<Ast<'src>, SyntaxError> {
    let src = src.into();
    let input = input.spanned((src.len()..src.len()).into());

    let parser = ast_parser();
    let ast = parser
        .parse(input)
        .into_result()
        .map_err(|errs| ParseErrors {
            src: NamedSource::new(name, src),
            related: errs.into_iter().map(Into::into).collect(),
        })?;

    Ok(ast)
}
