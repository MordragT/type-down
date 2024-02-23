use crate::parse::{
    ast::{Args, Value},
    Ast,
};
use html_writer::{DynHtmlElement, HtmlElement};
use miette::Diagnostic;
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

#[cfg(feature = "docx")]
pub mod docx;
#[cfg(feature = "html")]
pub mod html;
#[cfg(feature = "pdf")]
pub mod pdf;

pub trait Compiler {
    type Error: Diagnostic;
    type Context;

    fn compile(ctx: Self::Context, ast: &Ast) -> Result<(), Self::Error>;
}

// TODO font-family etc.

#[derive(Debug, Error, Diagnostic, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[diagnostic(code(type_down::compile::ContextBuilder))]
pub enum ContextError {
    #[error("Missing title")]
    MissingTitle,
    #[error("Missing source")]
    MissingSource,
    #[error("Missing destination")]
    MissingDest,
}

pub struct ContextBuilder {
    title: Option<String>,
    source: Option<PathBuf>,
    dest: Option<PathBuf>,
    symbol_table: SymbolTable,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            source: None,
            dest: None,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn source(mut self, source: PathBuf) -> Self {
        self.source = Some(source);
        self
    }

    pub fn destination(mut self, dest: PathBuf) -> Self {
        self.dest = Some(dest);
        self
    }

    pub fn register_func(
        mut self,
        key: impl Into<String>,
        f: impl Fn(&Args) -> DynHtmlElement + 'static,
    ) -> Self {
        self.symbol_table.register_func(key, f);
        self
    }

    pub fn build(self) -> Result<Context, ContextError> {
        let Self {
            title,
            source,
            dest,
            symbol_table,
        } = self;

        let title = title.ok_or(ContextError::MissingTitle)?;
        let source = source.ok_or(ContextError::MissingSource)?;
        let dest = dest.ok_or(ContextError::MissingDest)?;

        Ok(Context {
            title,
            source,
            dest,
            symbol_table,
        })
    }
}

pub struct Context {
    pub title: String,
    pub source: PathBuf,
    pub dest: PathBuf,
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
        Value::StringValue(val) => val,
        _ => todo!(),
    };

    HtmlElement::image().src(src).into()
}
