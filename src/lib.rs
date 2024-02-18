use ast::Ast;
use context::Context;

pub mod ast;
pub mod context;
pub mod cst;
pub mod html;

pub trait Compiler {
    type Error;

    fn compile(ctx: &Context, ast: &Ast) -> Result<(), Self::Error>;
}
