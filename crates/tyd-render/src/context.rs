use std::{path::Path, sync::Arc};

use miette::NamedSource;

use crate::{
    command::Command,
    error::EngineError,
    value::{Shape, Value},
};

pub trait Context<S: Shape>: SymbolTable<S> {
    fn named_source(&self) -> NamedSource<Arc<str>>;
    fn file_path(&self) -> &Path;
    fn work_path(&self) -> &Path;
    fn error(&mut self, e: EngineError);
    fn errors(&mut self, errs: impl IntoIterator<Item = EngineError>);
    fn has_errors(&self) -> bool;
    fn into_errors(self) -> Vec<EngineError>;
}

pub trait SymbolTable<S: Shape> {
    fn symbol(&self, name: impl AsRef<str>) -> Option<Value<S>>;
    fn command(&self, name: impl AsRef<str>) -> Option<Arc<dyn Command<S, Self>>>;

    fn add_symbol(
        &mut self,
        name: impl Into<String>,
        value: impl Into<Value<S>>,
    ) -> Option<Value<S>>;
}
