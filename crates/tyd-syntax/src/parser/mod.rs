pub mod code;
pub mod markup;

use chumsky::{
    combinator::{Map, ToSlice},
    error::Rich,
    extra, ParseResult, Parser as ChumskyParser,
};
use ecow::EcoString;

use self::markup::ast;
use crate::{
    ast::Ast,
    error::{SyntaxErrors, SyntaxResult},
    Source, Span,
};

pub struct Parser {
    source: Source,
    state: ParserState,
}

impl Parser {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            state: ParserState::default(),
        }
    }

    pub fn with_state(source: Source, state: ParserState) -> Self {
        Self { source, state }
    }

    pub fn parse(&mut self) -> SyntaxResult<Ast> {
        let parser = ast();
        let source = self.source.as_str();

        let ast = parser
            .parse_with_state(source, &mut self.state)
            .into_result()
            .map_err(|errs| SyntaxErrors {
                src: self.source.named_source(),
                related: errs.into_iter().map(Into::into).collect(),
            })?;

        Ok(ast)
    }

    pub fn try_parse<'src>(&'src mut self) -> ParseResult<Ast, Rich<'src, char, Span>> {
        let parser = ast();
        let source = self.source.as_str();

        parser.parse_with_state(source, &mut self.state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParserState {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ParserContext {
    indent: usize,
}

type Extra<'src> = extra::Full<Rich<'src, char, Span>, ParserState, ParserContext>;

pub trait ParserExt<'src, T>: ChumskyParser<'src, &'src str, T, Extra<'src>> + Sized {
    #[inline]
    fn to_ecow(self) -> Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString> {
        self.to_slice().map(EcoString::from)
    }
}

impl<'src, T, P: ChumskyParser<'src, &'src str, T, Extra<'src>> + Sized> ParserExt<'src, T> for P {}

pub fn try_parse<'src>(source: &'src str) -> ParseResult<Ast, Rich<'src, char, Span>> {
    let parser = ast();
    let mut state = ParserState {};
    parser.parse_with_state(source, &mut state)
}
