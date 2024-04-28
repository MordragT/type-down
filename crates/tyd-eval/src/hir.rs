use ecow::EcoString;
use std::{collections::BTreeMap, sync::Arc};
use tyd_syntax::Span;

use crate::{
    eval::Engine,
    value::{Cast, Value},
};

pub type Map<E> = Arc<BTreeMap<EcoString, Value<E>>>;
pub type List<E> = Arc<Vec<Value<E>>>;

#[derive(Debug, Clone)]
pub struct Func<E: Engine> {
    f: Arc<fn(Args<E>, &mut E, &E::Visitor) -> Option<Value<E>>>,
}

impl<E: Engine> Func<E> {
    pub fn new(f: fn(Args<E>, &mut E, &E::Visitor) -> Option<Value<E>>) -> Self {
        Self { f: Arc::new(f) }
    }

    pub fn call(&self, args: Args<E>, engine: &mut E, visitor: &E::Visitor) -> Option<Value<E>> {
        (self.f)(args, engine, visitor)
    }
}

#[derive(Debug, Clone)]
pub struct Call<E: Engine> {
    pub ident: EcoString,
    pub args: Args<E>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Arg<E: Engine> {
    pub name: Option<EcoString>,
    pub value: Value<E>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct NamedArg<E: Engine> {
    pub name: EcoString,
    pub value: Value<E>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PositionalArg<E: Engine> {
    pub value: Value<E>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Args<E: Engine> {
    pub named: Vec<NamedArg<E>>,
    pub positional: Vec<PositionalArg<E>>,
    pub span: Span,
}

impl<E: Engine> Args<E> {
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

    pub fn insert(&mut self, arg: Arg<E>) {
        let Arg { name, value, span } = arg;

        if let Some(name) = name {
            self.named.push(NamedArg { name, value, span });
        } else {
            self.positional.push(PositionalArg { value, span });
        }
    }

    pub fn add_named(&mut self, name: EcoString, value: Value<E>, span: Span) {
        self.named.push(NamedArg { name, value, span });
    }

    pub fn add_positional(&mut self, value: Value<E>, span: Span) {
        self.positional.push(PositionalArg { value, span });
    }

    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }

    pub fn remove_named<T: Cast<E>>(&mut self, name: impl AsRef<str>) -> T {
        let pos = self
            .named
            .iter()
            .position(|arg| arg.name.as_str() == name.as_ref())
            .unwrap();
        let arg = self.named.remove(pos);
        T::cast(arg.value)
    }

    pub fn remove_positonal<T: Cast<E>>(&mut self, pos: usize) -> T {
        let arg = self.positional.remove(pos);
        T::cast(arg.value)
    }

    pub fn pop_positional<T: Cast<E>>(&mut self) -> T {
        let arg = self.positional.pop().unwrap();
        T::cast(arg.value)
    }
}
