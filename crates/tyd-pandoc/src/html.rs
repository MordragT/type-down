use miette::Diagnostic;
use pandoc::{
    InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, Pandoc, PandocOption,
    PandocOutput,
};
use std::io;
use thiserror::Error;
use tyd_eval::{
    error::EngineErrors,
    ir,
    render::{Output, Render},
};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::docx::DocxCompiler::compile))]
pub enum HtmlError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Pandoc(#[from] pandoc::PandocError),
    #[error(transparent)]
    Engine(#[from] EngineErrors),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Render for HtmlCompiler {
    type Error = HtmlError;

    fn render(pandoc: ir::Pandoc, output: Output) -> Result<(), Self::Error> {
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

        match pandoc.execute()? {
            PandocOutput::ToBuffer(buf) => println!("{buf}"),
            _ => (),
        }

        Ok(())
    }
}
