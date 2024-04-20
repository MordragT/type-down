use miette::Diagnostic;
use std::io;
use thiserror::Error;
use tyd_render::error::{EngineError, EngineErrors};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
pub enum PandocError {
    #[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
    Io(#[from] io::Error),
    #[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
    Engine(#[from] EngineError),
    #[diagnostic(transparent)]
    EngineMulti(#[from] EngineErrors),
}
