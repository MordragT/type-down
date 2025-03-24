/// This module provides parsing functionality for converting source text into a structured document.
use chumsky::prelude::*;
use tyd_core::prelude::*;

use crate::{error::SourceDiagnostic, source::Source, SpanMetadata};

pub mod code;
pub mod ext;
pub mod extra;
pub mod markup;

/// Result of parsing a source document.
///
/// Contains the parsed document (if successful), span metadata, and any parsing errors.
pub struct ParseResult {
    /// The parsed document, if parsing was successful enough to produce one.
    pub doc: Option<Doc>,
    /// Metadata about spans in the source text.
    pub spans: SpanMetadata,
    /// Collection of diagnostics for any errors encountered during parsing.
    pub errors: Vec<SourceDiagnostic>,
}

/// Parses a source document into a structured representation.
///
/// This function takes a source document and attempts to parse it using the markup parser.
/// It returns a ParseResult containing the parsed document (if successful), span metadata,
/// and any errors encountered during parsing.
///
/// # Arguments
///
/// * `source` - The source document to parse
///
/// # Returns
///
/// A ParseResult containing the parsing outcome
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

    let errors = errors.into_iter().map(SourceDiagnostic::from).collect();

    return ParseResult { doc, spans, errors };
}
