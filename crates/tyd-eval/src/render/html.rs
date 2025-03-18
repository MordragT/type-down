use pandoc::{
    InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, Pandoc, PandocOption,
    PandocOutput,
};

use crate::{ir, tracer::Tracer};

use super::{Output, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Render for HtmlCompiler {
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
