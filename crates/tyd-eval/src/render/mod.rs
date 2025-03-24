//! Document rendering module
//!
//! This module provides functionality for rendering documents in various formats
//! including DOCX, HTML, Pandoc, and PDF.

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

/// A trait for rendering documents in different formats.
///
/// Implementations of this trait can convert a Pandoc document to various
/// output formats, writing to either a file or stdout.
pub trait Render {
    /// Renders a Pandoc document to the specified output.
    ///
    /// # Parameters
    /// * `pandoc` - The Pandoc document to render
    /// * `output` - The destination for the rendered document (file or stdout)
    /// * `tracer` - A tracer for logging the rendering process
    fn render(pandoc: ir::Pandoc, output: Output, tracer: &mut Tracer);
}

/// Specifies where the rendered document should be output.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    /// Output to a file at the specified path
    File(PathBuf),
    /// Output to standard output
    Stdout,
}
