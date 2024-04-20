use miette::Diagnostic;
use std::path::PathBuf;
use tyd_syntax::ast::Ast;

pub use command::*;
pub use engine::*;
pub use table::*;
pub use ty::*;
pub use value::*;

// pub mod builtin;
mod command;
mod engine;
pub mod error;
mod table;
mod ty;
mod value;

pub trait Render {
    type Error: Diagnostic;
    type Context;

    fn render(ast: &Ast, ctx: Self::Context, output: Output) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}
