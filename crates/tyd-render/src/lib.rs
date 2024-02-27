use miette::Diagnostic;
use std::path::PathBuf;
use tyd_syntax::ast::Ast;

pub use context::*;
pub use value::*;

mod context;
pub mod error;
mod value;

pub trait Render {
    type Error: Diagnostic;
    type Content;

    fn render(ast: &Ast, ctx: Context<Self::Content>, output: Output) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}
