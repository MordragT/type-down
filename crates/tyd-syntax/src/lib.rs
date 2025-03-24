#![feature(array_windows)]
#![feature(impl_trait_in_assoc_type)]
#![feature(let_chains)]

//! This crate provides a parser and utilities for working with structured text.
//! It uses the `chumsky` library for parsing and follows the phases defined in `tyd_core`.

use chumsky::span::SimpleSpan;
use tyd_core::meta::{Metadata, Phase};

/// Contains error types and utilities for handling parsing errors.
pub mod error;
/// Provides the core parsing functionality for structured text documents.
pub mod parser;
/// Defines the `Source` struct and related utilities for working with source text.
pub mod source;

/// Re-exports commonly used items from this crate for easier imports.
pub mod prelude {
    pub use crate::error::*;
    pub use crate::parser::*;
    pub use crate::source::Source;
    pub use crate::{Span, SpanMetadata, SyntaxPhase};
}

/// Represents span metadata created during the syntax processing phase.
pub type SpanMetadata = Metadata<SyntaxPhase>;

/// Represents a span of text in the source document, identified by byte indices.
pub type Span = SimpleSpan<usize>;

/// A value paired with location information showing where in the source it came from.
pub type Spanned<T> = (T, Span);

/// Represents the syntax processing phase in the document processing pipeline.
///
/// This phase is responsible for parsing the document into a structured AST
/// while maintaining location information for each element.
#[derive(Clone, Copy, Debug)]
pub struct SyntaxPhase;

/// Implementation of the `Phase` trait for the syntax processing stage.
///
/// Each associated type represents a different element that can appear in
/// a document, with the `Span` type providing location information for error
/// reporting and other operations that need source positioning.
impl Phase for SyntaxPhase {
    type Error = Span;
    type Tag = Span;
    type Text = Span;
    type Label = Span;

    // Block-level elements
    type Block = Span;
    type Raw = Span;
    type Heading = Span;
    type HeadingMarker = Span;
    type Table = Span;
    type TableRow = Span;
    type List = Span;
    type ListItem = Span;
    type Enum = Span;
    type EnumItem = Span;
    type Terms = Span;
    type TermItem = Span;
    type Paragraph = Span;
    type Plain = Span;

    // Inline elements
    type Inline = Span;
    type Quote = Span;
    type Strikeout = Span;
    type Emphasis = Span;
    type Strong = Span;
    type Subscript = Span;
    type Supscript = Span;
    type Link = Span;
    type Ref = Span;
    type RawInline = Span;
    type MathInline = Span;
    type Comment = Span;
    type Escape = Span;
    type Word = Span;
    type Spacing = Span;
    type SoftBreak = Span;

    // Code elements
    type Code = Span;
    type Expr = Span;
    type Let = Span;
    type Bind = Span;
    type If = Span;
    type For = Span;
    type Call = Span;
    type Args = Span;
    type Arg = Span;
    type Literal = Span;
    type Ident = Span;
    type Content = Span;
}
