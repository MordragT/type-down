use miette::Diagnostic;
use thiserror::Error;

use crate::{Map, Object, ObjectKind};

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(tyd_render::Context::call))]
pub enum CallError {
    #[error("{0}")]
    Message(String),
    #[error("Missing Argument {0}")]
    MissingArgument(String),
    #[error("Wrong Type for argument {arg}. Expected {expected}")]
    WrongType { arg: String, expected: ObjectKind },
    #[error("Wrong Arguments")]
    WrongArguments,
}

pub type Args = Map<String, Object>;
pub type Func = Box<dyn Fn(Args) -> Result<Object, CallError>>;

pub type SymbolTable = Map<String, Object>;
pub type FunctionTable = Map<String, Func>;

// TODO font-family etc.

pub struct Context {
    function_table: FunctionTable,
    symbol_table: SymbolTable,
}

impl Context {
    pub fn new() -> Self {
        Self {
            function_table: FunctionTable::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn function(
        mut self,
        key: impl Into<String>,
        f: impl Fn(Args) -> Result<Object, CallError> + 'static,
    ) -> Self {
        self.function_table.insert(key.into(), Box::new(f));
        self
    }

    pub fn call(&self, key: impl AsRef<str>) -> Option<&Func> {
        self.function_table.get(key.as_ref())
    }

    pub fn symbol(mut self, key: impl Into<String>, value: impl Into<Object>) -> Self {
        self.symbol_table.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&Object> {
        self.symbol_table.get(key.as_ref())
    }
}
