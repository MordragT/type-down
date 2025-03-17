use miette::Diagnostic;
use std::{fs, io};
use thiserror::Error;
use tyd_eval::{
    error::EngineErrors,
    ir,
    render::{Output, Render},
};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
pub enum PandocError {
    #[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
    Io(#[from] io::Error),
    #[diagnostic(transparent)]
    Engine(#[from] EngineErrors),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    type Error = PandocError;

    fn render(pandoc: ir::Pandoc, output: Output) -> Result<(), Self::Error> {
        let contents = serde_json::to_string_pretty(&pandoc).unwrap();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => fs::write(path, contents)?,
        }

        Ok(())
    }
}
