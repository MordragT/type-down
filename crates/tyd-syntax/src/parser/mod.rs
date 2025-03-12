use chumsky::prelude::*;
use tyd_doc::prelude::*;

use crate::{
    error::{SyntaxError, SyntaxErrors},
    source::Source,
    SpanMetadata,
};

pub mod code;
pub mod ext;
pub mod extra;
pub mod markup;

pub struct ParseResult {
    pub doc: Option<Doc>,
    pub spans: SpanMetadata,
    pub errors: SyntaxErrors,
}

pub fn parse(source: &Source) -> ParseResult {
    use self::extra::*;

    let input = source.as_str();
    let parser = markup::parser();

    let mut state = State::from(StateRepr::new());

    let (blocks, errors) = parser
        .parse_with_state(input, &mut state)
        .into_output_errors();

    let StateRepr { builder, meta } = state.0;

    let spans = Metadata::from(meta);
    let doc = blocks.map(|blocks| builder.finish(blocks));

    let related = errors.into_iter().map(SyntaxError::from).collect();
    let errors = SyntaxErrors::with_related(source.clone(), related);

    return ParseResult { doc, spans, errors };
}
