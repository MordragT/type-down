use miette::Diagnostic;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc};
use std::io;
use thiserror::Error;

use tyd_render::{Context, Output, Render};
use tyd_syntax::ast::{visitor::Visitor, Ast};

use super::pandoc::PandocBuilder;

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::docx::DocxCompiler::compile))]
pub enum DocxError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Pandoc(#[from] pandoc::PandocError),
    #[error("Stdout is unsupported for docx")]
    StdoutUnsupported,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocxCompiler;

impl Render for DocxCompiler {
    type Error = DocxError;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error> {
        let dest = match output {
            Output::File(path) => path,
            Output::Stdout => return Err(DocxError::StdoutUnsupported),
        };

        let mut builder = PandocBuilder::new(ctx);
        builder.visit_ast(ast);

        let pandoc = builder.build();
        let contents = pandoc.to_json();

        let mut pandoc = Pandoc::new();

        pandoc
            .set_input_format(InputFormat::Json, Vec::new())
            .set_input(InputKind::Pipe(contents))
            .set_output_format(OutputFormat::Docx, Vec::new())
            .set_output(OutputKind::File(dest));

        pandoc.execute()?;

        Ok(())
    }
}
