use miette::Diagnostic;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOption};
use std::io;
use thiserror::Error;

use tyd_render::{Context, Output, Render};
use tyd_syntax::ast::{visitor::Visitor, Ast};

use super::pandoc::PandocBuilder;
#[derive(Debug, Error, Diagnostic)]
#[diagnostic(code(type_down::compile::pdf::PdfCompiler::compile))]
pub enum PdfError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Pandoc(#[from] pandoc::PandocError),
    #[error("Stdout is unsupported for pdf")]
    StdoutUnsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfCompiler;

impl Render for PdfCompiler {
    type Error = PdfError;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error> {
        let dest = match output {
            Output::File(path) => path,
            Output::Stdout => return Err(PdfError::StdoutUnsupported),
        };

        let mut builder = PandocBuilder::new(ctx);
        builder.visit_ast(ast);

        let pandoc = builder.build();
        let contents = pandoc.to_json();

        let mut pandoc = Pandoc::new();

        pandoc
            .set_input_format(InputFormat::Json, Vec::new())
            .set_input(InputKind::Pipe(contents))
            .set_output_format(OutputFormat::Pdf, Vec::new())
            .set_output(OutputKind::File(dest))
            .add_option(PandocOption::PdfEngine("typst".into()));

        pandoc.execute()?;

        Ok(())
    }
}
