use ecow::EcoString;
use std::sync::Arc;

use crate::{
    ir,
    value::{Map, Value},
};

/// A stack of scopes.
#[derive(Debug, Clone)]
pub struct Scopes {
    scopes: Vec<Scope>,
    base: Arc<Scope>,
}

impl Scopes {
    pub fn new(base: Arc<Scope>) -> Self {
        Self {
            base,
            scopes: vec![Scope::new()],
        }
    }

    pub fn with_scope(base: Arc<Scope>, scope: Scope) -> Self {
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

    pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value> {
        let name = name.as_ref();

        self.scopes
            .iter()
            .rev()
            .chain(std::iter::once(self.base.as_ref()))
            .find_map(|scope| scope.symbol(name))
    }

    pub fn func(&self, name: impl AsRef<str>) -> Option<ir::Func> {
        let name = name.as_ref();

        self.scopes
            .iter()
            .rev()
            .chain(std::iter::once(self.base.as_ref()))
            .find_map(|scope| scope.func(name))
    }

    pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value>
    where
        N: Into<EcoString>,
        V: Into<Value>,
    {
        self.scopes
            .last_mut()
            .expect("no active scope")
            .define_symbol(name, value)
    }

    pub fn define_func<N, F>(&mut self, name: N, func: F) -> Option<Value>
    where
        N: Into<EcoString>,
        F: Into<ir::Func>,
    {
        self.scopes
            .last_mut()
            .expect("no active scope")
            .define_func(name, func)
    }
}

/// A scoped table binding names to values.
#[derive(Debug, Clone)]
pub struct Scope {
    symbols: Map,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: Map::new(),
        }
    }

    pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value> {
        self.symbols.get(name.as_ref()).cloned()
    }

    pub fn func(&self, name: impl AsRef<str>) -> Option<ir::Func> {
        self.symbols
            .get(name.as_ref())
            .cloned()
            .and_then(|value| value.into_func())
    }

    pub fn register_symbol<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<EcoString>,
        V: Into<Value>,
    {
        self.define_symbol(name, value);
        self
    }

    pub fn register_func<N, F>(mut self, name: N, func: F) -> Self
    where
        N: Into<EcoString>,
        F: Into<ir::Func>,
    {
        self.define_func(name, func);
        self
    }

    pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value>
    where
        N: Into<EcoString>,
        V: Into<Value>,
    {
        self.symbols.insert(name.into(), value.into())
    }

    pub fn define_func<N, F>(&mut self, name: N, func: F) -> Option<Value>
    where
        N: Into<EcoString>,
        F: Into<ir::Func>,
    {
        self.symbols.insert(name.into(), Value::Func(func.into()))
    }
}
