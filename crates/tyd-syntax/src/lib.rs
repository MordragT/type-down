#![feature(array_windows)]
#![feature(impl_trait_in_assoc_type)]
#![feature(let_chains)]

use chumsky::span::SimpleSpan;
use tyd_doc::meta::{Metadata, Phase};

pub mod error;
pub mod parser;
pub mod source;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::parser::*;
    pub use crate::source::Source;
    pub use crate::{Span, SpanMetadata, SyntaxPhase};
}

pub type SpanMetadata = Metadata<SyntaxPhase>;
pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Copy, Debug)]
pub struct SyntaxPhase;

impl Phase for SyntaxPhase {
    type Error = Span;
    type Tag = Span;
    type Text = Span;
    type Label = Span;

    // Block
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

    // Inline
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

    // Code
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
