use miette::Diagnostic;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOption};
use std::io;
use thiserror::Error;
use tyd_render::{Output, Render};
use tyd_syntax::ast::Ast;

use crate::engine::{PandocEngine, PandocState};

#[derive(Debug, Error, Diagnostic)]
pub enum PdfError {
    #[diagnostic(code(type_down::compile::pdf::PdfCompiler::compile))]
    #[error(transparent)]
    Io(#[from] io::Error),
    #[diagnostic(code(type_down::compile::pdf::PdfCompiler::compile))]
    #[error(transparent)]
    PandocExec(#[from] pandoc::PandocError),
    #[diagnostic(transparent)]
    #[error(transparent)]
    Pandoc(#[from] crate::error::PandocError),
    #[diagnostic(code(type_down::compile::pdf::PdfCompiler::compile))]
    #[error("Stdout is unsupported for pdf")]
    StdoutUnsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfCompiler;

impl Render for PdfCompiler {
    type Error = PdfError;
    type Context = PandocState;

    fn render(ast: &Ast, ctx: Self::Context, output: Output) -> Result<(), Self::Error> {
        let dest = match output {
            Output::File(path) => path,
            Output::Stdout => return Err(PdfError::StdoutUnsupported),
        };

        let engine = PandocEngine::new();
        let pandoc = engine.build(ctx, ast)?;
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
