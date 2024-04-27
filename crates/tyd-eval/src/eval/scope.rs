use ecow::EcoString;
use std::{collections::BTreeMap, sync::Arc};

use super::Engine;
use crate::{error::EngineError, foundations::Func, value::Value};

// TODO save func also in value

/// A scoped table binding names to values.
#[derive(Debug, Clone)]
pub struct Scope<E: Engine> {
    symbols: BTreeMap<EcoString, Value<E>>,
    funcs: BTreeMap<EcoString, Arc<dyn Func<E>>>,
    errors: Vec<EngineError>,
}

impl<E: Engine> Scope<E> {
    pub fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
            funcs: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value<E>> {
        self.symbols.get(name.as_ref()).cloned()
    }

    pub fn func(&self, name: impl AsRef<str>) -> Option<Arc<dyn Func<E>>> {
        self.funcs.get(name.as_ref()).cloned()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error(&mut self, e: EngineError) {
        self.errors.push(e);
    }

    pub fn errors(&mut self, errs: impl IntoIterator<Item = EngineError>) {
        self.errors.extend(errs)
    }

    pub fn into_errors(self) -> Vec<EngineError> {
        self.errors
    }

    pub fn register_symbol(
        mut self,
        name: impl Into<EcoString>,
        value: impl Into<Value<E>>,
    ) -> Self {
        self.define_symbol(name, value);
        self
    }

    pub fn register_func(
        mut self,
        name: impl Into<EcoString>,
        func: impl Func<E> + 'static,
    ) -> Self {
        self.define_func(name, func);
        self
    }

    pub fn define_symbol(
        &mut self,
        name: impl Into<EcoString>,
        value: impl Into<Value<E>>,
    ) -> Option<Value<E>> {
        self.symbols.insert(name.into(), value.into())
    }

    pub fn define_func(
        &mut self,
        name: impl Into<EcoString>,
        func: impl Func<E> + 'static,
    ) -> Option<Arc<dyn Func<E>>> {
        self.funcs.insert(name.into(), Arc::new(func))
    }
}
