use miette::Diagnostic;
use pandoc::{
    InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, Pandoc, PandocOption,
    PandocOutput,
};
use std::io;
use thiserror::Error;
use tyd_render::{Output, Render};
use tyd_syntax::ast::Ast;

use crate::engine::{PandocEngine, PandocState};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::docx::DocxCompiler::compile))]
pub enum HtmlError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    PandocExec(#[from] pandoc::PandocError),
    #[error(transparent)]
    Pandoc(#[from] crate::error::PandocError),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Render for HtmlCompiler {
    type Error = HtmlError;
    type Context = PandocState;

    fn render(ast: &Ast, ctx: Self::Context, output: Output) -> Result<(), Self::Error> {
        let output_kind = match output {
            Output::File(path) => OutputKind::File(path),
            Output::Stdout => OutputKind::Pipe,
        };

        let engine = PandocEngine::new();
        let pandoc = engine.build(ctx, ast)?;
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
