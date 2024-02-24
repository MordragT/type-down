use miette::Diagnostic;
use std::{collections::BTreeMap, path::PathBuf};
use tyd_syntax::ast::Ast;

pub use context::*;
pub use object::*;

pub mod builtin;
mod context;
mod object;
pub trait Render {
    type Error: Diagnostic;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}

pub type Map<K, V> = BTreeMap<K, V>;
