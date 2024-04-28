use ecow::EcoString;

use crate::{
    eval::{Engine, Scope},
    hir,
    value::Value,
};

mod func;
mod signature;

pub use func::PluginFunc;
pub use signature::Signature;

pub struct Plugin<E: Engine> {
    scope: Scope<E>,
}

impl<E: Engine> Plugin<E> {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
        }
    }

    pub fn register_symbol<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<EcoString>,
        V: Into<Value<E>>,
    {
        self.scope.define_symbol(name, value);
        self
    }

    pub fn register_func<F>(mut self, name: impl Into<EcoString>) -> Self
    where
        F: PluginFunc<E>,
    {
        self.define_func::<F>(name);
        self
    }

    pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value<E>>
    where
        N: Into<EcoString>,
        V: Into<Value<E>>,
    {
        self.scope.define_symbol(name, value)
    }

    pub fn define_func<F>(&mut self, name: impl Into<EcoString>) -> Option<Value<E>>
    where
        F: PluginFunc<E>,
    {
        let f = func::dispatch::<E, F>;
        let func = hir::Func::new(f);

        self.scope.define_func(name, func)
    }

    pub fn into_scope(self) -> Scope<E> {
        self.scope
    }
}
