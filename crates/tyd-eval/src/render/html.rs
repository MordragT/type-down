use pandoc::{
    InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, Pandoc, PandocOption,
    PandocOutput,
};

use crate::{ir, tracer::Tracer};

use super::{Output, Render};

/// HtmlCompiler is responsible for rendering IR documents as HTML5.
///
/// This compiler uses the Pandoc library to convert internal representation
/// to HTML5 format with various Markdown extensions enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Render for HtmlCompiler {
    /// Renders the given Pandoc IR document to HTML5 format.
    ///
    /// # Arguments
    ///
    /// * `pandoc` - The internal representation document to render
    /// * `output` - Where to direct the rendered output (file or stdout)
    /// * `tracer` - Error tracer for reporting issues during rendering
    ///
    /// # Process
    ///
    /// 1. Converts the IR to JSON format
    /// 2. Configures Pandoc with appropriate HTML5 settings
    /// 3. Executes the conversion and handles the result
    fn render(pandoc: ir::Pandoc, output: Output, tracer: &mut Tracer) {
        let output_kind = match output {
            Output::File(path) => OutputKind::File(path),
            Output::Stdout => OutputKind::Pipe,
        };

        let contents = pandoc.to_json();
        let mut pandoc = Pandoc::new();

        use MarkdownExtension::*;

        pandoc
            .add_option(PandocOption::Standalone)
            .set_input_format(InputFormat::Json, Vec::new())
            .set_input(InputKind::Pipe(contents))
            .set_output_format(
                OutputFormat::Html5,
                vec![AutoIdentifiers, LineBlocks, NativeDivs, NativeSpans],
            )
            .set_output(output_kind);

        match pandoc.execute() {
            Ok(PandocOutput::ToBuffer(buf)) => println!("{buf}"),
            Err(e) => tracer.error(e),
            _ => (),
        }
    }
}
