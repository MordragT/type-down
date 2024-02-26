use miette::Diagnostic;
use thiserror::Error;

use crate::{Map, Object, ObjectKind};

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(tyd_render::Context::call))]
pub enum ContextError {
    #[error("{0}")]
    Message(String),
    #[error("Missing Argument {0}")]
    MissingArgument(String),
    #[error("Wrong Type for argument {arg}. Expected {expected}")]
    WrongArgType { arg: String, expected: ObjectKind },
    #[error("Wrong Arguments")]
    WrongArguments,
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),
    #[error("Symbol '{0}' not found")]
    SymbolNotFound(String),
}

pub type Args = Map<String, Object>;

// TODO create Function trait

pub trait Function {
    // maybe also expose which are required and wich are optionally(potentially with default params ?)
    fn args() -> &'static [&'static str];
}

pub type Func = Box<dyn Fn(Args) -> Result<Object, ContextError>>;

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
        f: impl Fn(Args) -> Result<Object, ContextError> + 'static,
    ) -> Self {
        self.function_table.insert(key.into(), Box::new(f));
        self
    }

    // TODO actually call the function insted of just returning it
    // automate argument validation then in here
    pub fn call(&self, key: impl AsRef<str>) -> Result<&Func, ContextError> {
        self.function_table
            .get(key.as_ref())
            .ok_or(ContextError::FunctionNotFound(key.as_ref().to_owned()))
    }

    pub fn symbol(mut self, key: impl Into<String>, value: impl Into<Object>) -> Self {
        self.symbol_table.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: impl AsRef<str>) -> Result<&Object, ContextError> {
        self.symbol_table
            .get(key.as_ref())
            .ok_or(ContextError::SymbolNotFound(key.as_ref().to_owned()))
    }
}
