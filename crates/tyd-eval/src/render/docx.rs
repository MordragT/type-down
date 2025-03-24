use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc};

use crate::{ir, tracer::Tracer};

use super::{Output, Render};

/// `DocxCompiler` is responsible for compiling documents into DOCX format.
///
/// This compiler uses the Pandoc library to convert the intermediate representation
/// to a Microsoft Word document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocxCompiler;

impl Render for DocxCompiler {
    /// Renders a Pandoc document into DOCX format.
    ///
    /// # Arguments
    ///
    /// * `pandoc` - The Pandoc intermediate representation to be rendered
    /// * `output` - The output destination (only file output is supported)
    /// * `tracer` - A tracer for error reporting and logging
    ///
    /// # Note
    ///
    /// This function does not support stdout output and will return an error if attempted.
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
            .set_output_format(OutputFormat::Docx, Vec::new())
            .set_output(OutputKind::File(dest));

        if let Err(e) = pandoc.execute() {
            tracer.error(e);
        }
    }
}
