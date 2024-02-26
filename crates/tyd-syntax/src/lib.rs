use chumsky::span::SimpleSpan;

pub mod ast;
pub mod cst;
pub mod error;

// pub mod prelude {
//     pub use crate::ast::{visitor::Visitor, Ast};
//     pub use crate::cst::{parse, Cst};
//     pub use crate::error::*;
// }

pub type Span = SimpleSpan<usize>;
