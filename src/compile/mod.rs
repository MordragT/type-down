use crate::parse::Ast;
use miette::Diagnostic;
use std::path::PathBuf;

pub mod html;
#[cfg(feature = "pdf")]
pub mod pdf;

pub trait Compiler {
    type Error: Diagnostic;

    fn compile(ctx: &Context, ast: &Ast) -> Result<(), Self::Error>;
}

// TODO font-family etc.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context {
    pub title: String,
    pub source: PathBuf,
    pub dest: PathBuf,
}

impl Context {
    pub fn new(title: String, source: PathBuf, dest: PathBuf) -> Self {
        Self {
            title,
            source,
            dest,
        }
    }
}
