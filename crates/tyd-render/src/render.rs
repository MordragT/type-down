use miette::Diagnostic;
use std::path::PathBuf;
use tyd_syntax::ast::Ast;

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
