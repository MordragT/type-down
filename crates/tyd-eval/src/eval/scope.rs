use ecow::EcoString;
use std::{collections::BTreeMap, sync::Arc};

use super::Engine;
use crate::{hir, value::Value};

/// A stack of scopes.
#[derive(Debug, Clone)]
pub struct Scopes<E: Engine> {
    scopes: Vec<Scope<E>>,
    base: Arc<Scope<E>>,
}

impl<E: Engine> Scopes<E> {
    pub fn new(base: Arc<Scope<E>>) -> Self {
        Self {
            base,
            scopes: vec![Scope::new()],
        }
    }

    pub fn with_scope(base: Arc<Scope<E>>, scope: Scope<E>) -> Self {
        Self {
            base,
            scopes: vec![scope],
        }
    }

    pub fn enter(&mut self) {
        self.scopes.push(Scope::new())
    }

    pub fn exit(&mut self) {
        self.scopes.pop().expect("no active scope");
    }

    pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value<E>> {
        let name = name.as_ref();

        self.scopes
            .iter()
            .rev()
            .chain(std::iter::once(self.base.as_ref()))
            .find_map(|scope| scope.symbol(name))
    }

    pub fn func(&self, name: impl AsRef<str>) -> Option<hir::Func<E>> {
        let name = name.as_ref();

        self.scopes
            .iter()
            .rev()
            .chain(std::iter::once(self.base.as_ref()))
            .find_map(|scope| scope.func(name))
    }

    pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value<E>>
    where
        N: Into<EcoString>,
        V: Into<Value<E>>,
    {
        self.scopes
            .last_mut()
            .expect("no active scope")
            .define_symbol(name, value)
    }

    pub fn define_func<N, F>(&mut self, name: N, func: F) -> Option<Value<E>>
    where
        N: Into<EcoString>,
        F: Into<hir::Func<E>>,
    {
        self.scopes
            .last_mut()
            .expect("no active scope")
            .define_func(name, func)
    }
}

/// A scoped table binding names to values.
#[derive(Debug, Clone)]
pub struct Scope<E: Engine> {
    symbols: BTreeMap<EcoString, Value<E>>,
}

impl<E: Engine> Scope<E> {
    pub fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
        }
    }

    pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value<E>> {
        self.symbols.get(name.as_ref()).cloned()
    }

    pub fn func(&self, name: impl AsRef<str>) -> Option<hir::Func<E>> {
        self.symbols
            .get(name.as_ref())
            .cloned()
            .and_then(|value| value.into_func())
    }

    pub fn register_symbol<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<EcoString>,
        V: Into<Value<E>>,
    {
        self.define_symbol(name, value);
        self
    }

    pub fn register_func<N, F>(mut self, name: N, func: F) -> Self
    where
        N: Into<EcoString>,
        F: Into<hir::Func<E>>,
    {
        self.define_func(name, func);
        self
    }

    pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value<E>>
    where
        N: Into<EcoString>,
        V: Into<Value<E>>,
    {
        self.symbols.insert(name.into(), value.into())
    }

    pub fn define_func<N, F>(&mut self, name: N, func: F) -> Option<Value<E>>
    where
        N: Into<EcoString>,
        F: Into<hir::Func<E>>,
    {
        self.symbols.insert(name.into(), Value::Func(func.into()))
    }
}
