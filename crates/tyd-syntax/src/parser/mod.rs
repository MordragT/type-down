use chumsky::{
    combinator::{Map, ToSlice},
    prelude::*,
    text::whitespace,
};
use ecow::EcoString;
use miette::NamedSource;

use crate::{prelude::Ast, Span};

use self::{
    error::{SyntaxErrors, SyntaxResult},
    markup::{block_parser, hard_break_parser},
};

pub mod code;
pub mod error;
pub mod markup;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParserState {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ParserContext {
    indent: usize,
}

type Extra<'src> = extra::Full<Rich<'src, char, Span>, ParserState, ParserContext>;

pub trait ParserExt<'src, T>: Parser<'src, &'src str, T, Extra<'src>> + Sized {
    #[inline]
    fn to_ecow(self) -> Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString> {
        self.to_slice().map(EcoString::from)
    }
}

impl<'src, T, P: Parser<'src, &'src str, T, Extra<'src>> + Sized> ParserExt<'src, T> for P {}

pub fn ast<'src>() -> impl Parser<'src, &'src str, Ast, Extra<'src>> {
    let ast = block_parser()
        .separated_by(hard_break_parser())
        .at_least(1)
        .collect()
        .map(|blocks| Ast { blocks });

    whitespace().ignore_then(ast).then_ignore(whitespace())
}

pub fn parse<'src>(src: &'src str, name: impl AsRef<str>) -> SyntaxResult<Ast> {
    let parser = ast();
    let mut state = ParserState {};

    let ast = parser
        .parse_with_state(src, &mut state)
        .into_result()
        .map_err(|errs| SyntaxErrors {
            src: NamedSource::new(name, src.to_owned()),
            related: errs.into_iter().map(Into::into).collect(),
        })?;

    Ok(ast)
}
