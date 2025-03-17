use std::collections::BTreeMap;

use ecow::EcoString;
use tyd_syntax::{source::Source, Span};

use crate::{tracer::Tracer, value::Value};

pub use pandoc_ast::*;

pub type Definition = (Vec<Inline>, Vec<Vec<Block>>);

pub type Map = BTreeMap<EcoString, Value>;
pub type List = Vec<Value>;
pub type Content = Vec<Inline>;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub named: Map,
    pub positional: List,
    pub span: Span,
    pub source: Source,
}

impl Arguments {
    pub fn pop<T>(&mut self) -> Option<T>
    where
        T: TryFrom<Value>,
    {
        let value = self.positional.pop()?;
        value.try_into().ok()
    }

    pub fn remove<T>(&mut self, name: impl AsRef<str>) -> Option<T>
    where
        T: TryFrom<Value>,
    {
        let value = self.named.remove(name.as_ref())?;
        value.try_into().ok()
    }
}

pub type Func = fn(Arguments, &mut Tracer) -> Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttrBuilder {
    ident: String,
    classes: Vec<String>,
    pairs: Vec<(String, String)>,
}

impl AttrBuilder {
    pub fn new() -> Self {
        Self {
            ident: String::new(),
            classes: Vec::new(),
            pairs: Vec::new(),
        }
    }

    pub fn ident(mut self, ident: impl Into<String>) -> Self {
        self.ident = ident.into();
        self
    }

    pub fn ident_opt(mut self, ident: Option<impl Into<String>>) -> Self {
        if let Some(ident) = ident {
            self.ident = ident.into();
        }
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    pub fn class_opt(mut self, class: Option<impl Into<String>>) -> Self {
        if let Some(class) = class {
            self.classes.push(class.into())
        }
        self
    }

    pub fn attr<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
        self
    }

    pub fn add_attr<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
    }

    pub fn build(self) -> Attr {
        let Self {
            ident,
            classes,
            pairs,
        } = self;
        (ident, classes, pairs)
    }

    pub fn empty() -> Attr {
        (String::new(), Vec::new(), Vec::new())
    }
}
