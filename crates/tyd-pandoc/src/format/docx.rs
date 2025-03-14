use miette::Diagnostic;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc};
use std::io;
use thiserror::Error;
use tyd_eval::{
    eval::Engine,
    render::{Output, Render},
    world::World,
};
use tyd_syntax::ast::Document;

use crate::engine::PandocEngine;

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
    type Engine = PandocEngine;

    fn render(
        doc: Document,
        world: World<Self::Engine>,
        output: Output,
    ) -> Result<(), Self::Error> {
        let dest = match output {
            Output::File(path) => path,
            Output::Stdout => return Err(DocxError::StdoutUnsupported),
        };

        let engine = PandocEngine::from_world(world);
        let pandoc = engine.build(doc)?;
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
