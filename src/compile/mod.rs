use crate::parse::{
    ast::{Args, Value},
    Ast,
};
use html_writer::{DynHtmlElement, HtmlElement};
use miette::Diagnostic;
use std::{collections::HashMap, path::PathBuf};

#[cfg(feature = "docx")]
pub mod docx;
#[cfg(feature = "html")]
pub mod html;
pub mod pandoc;
#[cfg(feature = "pdf")]
pub mod pdf;

pub trait Compiler {
    type Error: Diagnostic;
    type Context;

    fn compile(ast: &Ast, ctx: Self::Context, output: Output) -> Result<(), Self::Error>;
}

// TODO font-family etc.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    File(PathBuf),
    Stdout,
}

pub struct ContextBuilder {
    title: String,
    symbol_table: SymbolTable,
}

impl ContextBuilder {
    pub fn new(title: String) -> Self {
        Self {
            title,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn register_func(
        mut self,
        key: impl Into<String>,
        f: impl Fn(&Args) -> DynHtmlElement + 'static,
    ) -> Self {
        self.symbol_table.register_func(key, f);
        self
    }

    pub fn build(self) -> Context {
        let Self {
            title,
            symbol_table,
        } = self;

        Context {
            title,
            symbol_table,
        }
    }
}

pub struct Context {
    pub title: String,
    pub symbol_table: SymbolTable,
}

pub type Func = Box<dyn Fn(&Args) -> DynHtmlElement>;

pub struct SymbolTable {
    functions: HashMap<String, Func>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn register_func(
        &mut self,
        key: impl Into<String>,
        f: impl Fn(&Args) -> DynHtmlElement + 'static,
    ) {
        self.functions.insert(key.into(), Box::new(f));
    }

    pub fn func(&self, key: impl AsRef<str>) -> Option<&Func> {
        self.functions.get(key.as_ref())
    }
}

pub fn image(args: &Args) -> DynHtmlElement {
    let src = match &args["src"] {
        Value::String(val) => val,
        _ => todo!(),
    };

    HtmlElement::image().src(src).into()
}
