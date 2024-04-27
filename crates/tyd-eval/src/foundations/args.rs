use ecow::EcoString;
use tyd_syntax::Span;

use crate::{
    eval::Engine,
    value::{Cast, Value},
};

#[derive(Debug, Clone)]
pub struct Arg<E: Engine> {
    pub name: Option<EcoString>,
    pub value: Value<E>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Args<E: Engine> {
    named: Vec<(String, Value<E>)>,
    positional: Vec<Value<E>>,
}

impl<E: Engine> Args<E> {
    pub fn new() -> Self {
        Self {
            named: Vec::new(),
            positional: Vec::new(),
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.named.iter().map(|(n, _)| n)
    }

    pub fn add_named(&mut self, name: impl Into<String>, value: impl Into<Value<E>>) {
        self.named.push((name.into(), value.into()))
    }

    pub fn add_positional(&mut self, value: impl Into<Value<E>>) {
        self.positional.push(value.into())
    }

    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }

    pub fn remove_named<T: Cast<E>>(&mut self, name: impl AsRef<str>) -> T {
        let pos = self
            .named
            .iter()
            .position(|(n, _)| n == name.as_ref())
            .unwrap();
        let (_, value) = self.named.remove(pos);
        T::cast(value)
    }

    pub fn remove_positonal<T: Cast<E>>(&mut self, pos: usize) -> T {
        let value = self.positional.remove(pos);
        T::cast(value)
    }

    pub fn pop_positional<T: Cast<E>>(&mut self) -> T {
        let value = self.positional.pop().unwrap();
        T::cast(value)
    }
}
