use chumsky::{error::Rich, extra, ParseResult, Parser as P};

use self::markup::parser;
use crate::{
    error::{SyntaxErrors, SyntaxResult},
    node::Node,
    Source, Span,
};

pub mod code;
pub mod ext;
pub mod markup;

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

    pub fn parse(&mut self) -> SyntaxResult<Node> {
        let parser = parser();
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

    pub fn try_parse<'src>(&'src mut self) -> ParseResult<Node, Rich<'src, char, Span>> {
        let parser = parser();
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

pub fn try_parse<'src>(source: &'src str) -> ParseResult<Node, Rich<'src, char, Span>> {
    let parser = parser();
    let mut state = ParserState {};
    parser.parse_with_state(source, &mut state)
}
