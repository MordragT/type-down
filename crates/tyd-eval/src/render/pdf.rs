use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOption};

use crate::{ir, tracer::Tracer};

use super::{Output, Render};

/// A compiler that renders documents to PDF format using Pandoc with Typst as the PDF engine.
///
/// This implementation of the `Render` trait takes an IR Pandoc document and converts it to a
/// PDF file. It uses the Typst engine for PDF generation and applies a default template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfCompiler;

impl Render for PdfCompiler {
    /// Renders a Pandoc document to a PDF file.
    ///
    /// # Arguments
    ///
    /// * `pandoc` - The intermediate representation of the document to be rendered
    /// * `output` - The output destination (must be a file, stdout is not supported)
    /// * `tracer` - A tracer for error reporting
    ///
    /// # Notes
    ///
    /// This renderer:
    /// - Does not support stdout output
    /// - Uses the Typst PDF engine
    /// - Applies the "templates/default.typst" template
    fn render(pandoc: ir::Pandoc, output: Output, tracer: &mut Tracer) {
        let dest = match output {
            Output::File(path) => path,
            Output::Stdout => {
                tracer.error("Stdout is unsupported for pdf");
                return;
            }
        };

        let contents = pandoc.to_json();
        let mut pandoc = Pandoc::new();

        pandoc
            .set_input_format(InputFormat::Json, Vec::new())
            .set_input(InputKind::Pipe(contents))
            .set_output_format(OutputFormat::Pdf, Vec::new())
            .set_output(OutputKind::File(dest))
            .add_option(PandocOption::Template("templates/default.typst".into()))
            .add_option(PandocOption::PdfEngine("typst".into()));

        if let Err(e) = pandoc.execute() {
            tracer.error(e);
        }
    }
}
