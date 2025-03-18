pub use docx::*;
pub use html::*;
pub use pandoc::*;
pub use pdf::*;

mod docx;
mod html;
mod pandoc;
mod pdf;

use std::path::PathBuf;

use crate::{ir, tracer::Tracer};

pub trait Render {
    fn render(pandoc: ir::Pandoc, output: Output, tracer: &mut Tracer);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}
