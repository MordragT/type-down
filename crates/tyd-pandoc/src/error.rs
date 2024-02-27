use miette::Diagnostic;
use std::io;
use thiserror::Error;
use tyd_render::error::ContextError;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
#[error(transparent)]
pub enum PandocError {
    Io(#[from] io::Error),
    Call(#[from] ContextError),
}
