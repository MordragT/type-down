use miette::Diagnostic;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc};
use std::io;
use thiserror::Error;
use tyd_render::{Output, Render};
use tyd_syntax::parser::{ast::Ast, visitor::Visitor};

use crate::{builder::PandocBuilder, Content, Context};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::docx::DocxCompiler::compile))]
pub enum DocxError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    PandocExec(#[from] pandoc::PandocError),
    #[error(transparent)]
    Pandoc(#[from] crate::error::PandocError),
    #[error("Stdout is unsupported for pdf")]
    StdoutUnsupported,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocxCompiler;

impl Render for DocxCompiler {
    type Error = DocxError;
    type Content = Content;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error> {
        let dest = match output {
            Output::File(path) => path,
            Output::Stdout => return Err(DocxError::StdoutUnsupported),
        };

        let mut builder = PandocBuilder::new(ctx);
        builder.visit_ast(ast)?;

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
