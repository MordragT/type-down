use miette::Diagnostic;
use std::path::PathBuf;

use crate::ir;

pub trait Render {
    type Error: Diagnostic;

    fn render(pandoc: ir::Pandoc, output: Output) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}
