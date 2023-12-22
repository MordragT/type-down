pub mod grammar;
pub mod grammar_trait;
pub mod parser;
pub mod token;

pub mod prelude {
    pub use crate::grammar::Grammar;
    pub use crate::parser::parse;
}
