use miette::Diagnostic;
use std::io;
use thiserror::Error;
use tyd_render::error::EngineErrors;

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
pub enum PandocError {
    #[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
    Io(#[from] io::Error),
    #[diagnostic(transparent)]
    Engine(#[from] EngineErrors),
}
