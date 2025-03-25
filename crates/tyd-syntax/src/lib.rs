#![feature(array_windows)]
#![feature(impl_trait_in_assoc_type)]
#![feature(let_chains)]

//! This crate provides a parser and utilities for working with structured text.
//! It uses the `chumsky` library for parsing and follows the phases defined in `tyd_core`.

use chumsky::span::SimpleSpan;
use tyd_core::meta::{Metadata, UniformPhase};

/// Contains error types and utilities for handling parsing errors.
pub mod error;
/// Provides the core parsing functionality for structured text documents.
pub mod parser;
/// Defines the `Source` struct and related utilities for working with source text.
pub mod source;

/// Re-exports commonly used items from this crate for easier imports.
pub mod prelude {
    pub use crate::error::*;
    // pub use crate::kind::SyntaxKind;
    pub use crate::parser::*;
    pub use crate::source::Source;
    pub use crate::{LocationPhase, Span, Spans};
}

/// Represents span metadata created during the syntax processing phase.
pub type Spans = Metadata<LocationPhase>;

/// Represents a span of text in the source document, identified by byte indices.
pub type Span = SimpleSpan<usize>;

/// A value paired with location information showing where in the source it came from.
pub type Spanned<T> = (T, Span);

/// Represents the location phase in document processing.
///
/// This phase tracks the source code location information for elements in a
/// structured text document.
#[derive(Clone, Copy, Debug)]
pub struct LocationPhase;

impl UniformPhase for LocationPhase {
    /// The metadata type for this phase, which is a `Span` representing
    /// a location in the source text.
    type Meta = Span;
}
