use chumsky::span::SimpleSpan;

pub mod lexer;
pub mod parser;

pub mod prelude {
    pub use crate::lexer::code::{Arg, Call, Code, Expr, Literal};
    pub use crate::lexer::node::*;
    pub use crate::parser::ast::{Ast, Block, BlockQuote, Enum, List, Nested, Paragraph, Table};
    pub use crate::parser::visitor::*;
}

pub type Span = SimpleSpan<usize>;
