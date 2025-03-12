use ecow::EcoString;
use std::sync::Arc;
use tyd_syntax::Span;

use crate::{
    eval::Engine,
    value::{Downcast, Value},
};

pub use pandoc_ast::*;

type Repr = fn(Args, &mut Engine) -> Option<Value>;

#[derive(Debug, Clone)]
pub struct Func(Arc<Repr>);

impl Func {
    pub fn new(f: fn(Args, &mut Engine) -> Option<Value>) -> Self {
        Self(Arc::new(f))
    }

    pub fn call(&self, args: Args, engine: &mut Engine) -> Option<Value> {
        (self.0)(args, engine)
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub ident: EcoString,
    pub args: Args,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: Option<EcoString>,
    pub value: Value,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct NamedArg {
    pub name: EcoString,
    pub value: Value,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PositionalArg {
    pub value: Value,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub named: Vec<NamedArg>,
    pub positional: Vec<PositionalArg>,
    pub span: Span,
}

impl Args {
    pub fn new(span: Span) -> Self {
        Self {
            named: Vec::new(),
            positional: Vec::new(),
            span,
        }
    }

    pub fn names(&self) -> impl Iterator<Item = EcoString> + '_ {
        self.named.iter().map(|arg| arg.name.clone())
    }

    pub fn insert(&mut self, arg: Arg) {
        let Arg { name, value, span } = arg;

        if let Some(name) = name {
            self.named.push(NamedArg { name, value, span });
        } else {
            self.positional.push(PositionalArg { value, span });
        }
    }

    pub fn add_named(&mut self, name: EcoString, value: Value, span: Span) {
        self.named.push(NamedArg { name, value, span });
    }

    pub fn add_positional(&mut self, value: Value, span: Span) {
        self.positional.push(PositionalArg { value, span });
    }

    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }

    pub fn remove_named<T: Downcast>(&mut self, name: impl AsRef<str>) -> T {
        let pos = self
            .named
            .iter()
            .position(|arg| arg.name.as_str() == name.as_ref())
            .unwrap();
        let arg = self.named.remove(pos);
        T::downcast(arg.value)
    }

    pub fn remove_positonal<T: Downcast>(&mut self, pos: usize) -> T {
        let arg = self.positional.remove(pos);
        T::downcast(arg.value)
    }

    pub fn pop_positional<T: Downcast>(&mut self) -> T {
        let arg = self.positional.pop().unwrap();
        T::downcast(arg.value)
    }
}
