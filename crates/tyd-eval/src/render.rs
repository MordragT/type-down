use miette::Diagnostic;
use std::path::PathBuf;
use tyd_syntax::ast::Document;

use crate::{eval::Engine, world::World};

pub trait Render {
    type Error: Diagnostic;
    type Engine: Engine;

    fn render(doc: Document, world: World<Self::Engine>, output: Output)
        -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}
