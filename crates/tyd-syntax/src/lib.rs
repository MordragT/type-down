use chumsky::span::SimpleSpan;

pub mod ast;
pub mod code;
pub mod inline;
pub mod line;
pub mod visitor;

pub mod error;

pub mod prelude {
    pub use crate::ast::{Ast, Block, BlockQuote, Enum, List, Nested, Paragraph, Table};
    pub use crate::code::{Arg, Call, Code, Expr, Literal};
    pub use crate::inline::{
        Cite, Comment, Emphasis, Escape, Link, Quote, RawInline, Spacing, Strikeout, Strong,
        Subscript, Supscript, Word,
    };
    pub use crate::line::{
        BlockQuoteElement, BlockQuoteItem, Div, EnumItem, Heading, ListItem, Raw, TableCell,
        TableRow,
    };
    pub use crate::visitor::*;
}

pub type Span = SimpleSpan<usize>;
