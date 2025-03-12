use miette::Diagnostic;
use std::path::PathBuf;
use tyd_doc::doc::Doc;

use crate::world::World;

pub trait Render {
    type Error: Diagnostic;

    fn render(doc: Doc, world: World, output: Output) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}
