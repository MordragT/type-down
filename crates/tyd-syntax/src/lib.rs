use chumsky::span::SimpleSpan;

pub mod ast;
pub mod parser;
pub mod visitor;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::parser::*;
    pub use crate::visitor::*;
}

pub type Span = SimpleSpan<usize>;
