use ecow::EcoString;
use std::collections::{
    btree_map::{IntoKeys, IntoValues, Keys, Values},
    BTreeMap,
};

use crate::{
    ir,
    ty::Type,
    value::{TypeCast, Value},
    Plugin,
};

#[derive(Debug, Clone, Default)]
pub struct Scope {
    parent: Option<Box<Self>>,
    scope: BTreeMap<EcoString, Value>,
}

impl Scope {
    pub fn new(from: Self) -> Self {
        Self {
            parent: Some(Box::new(from)),
            scope: ir::Map::new(),
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn register<P: Plugin>(&mut self) -> &mut Self {
        P::init(self);
        self
    }

    pub fn with(&mut self, name: impl Into<EcoString>, value: impl Into<Value>) -> &mut Self {
        self.insert(name.into(), value);
        self
    }

    pub fn clear(&mut self) {
        self.scope.clear();
    }

    pub fn symbols(&self) -> Keys<EcoString, Value> {
        self.scope.keys()
    }

    pub fn values(&self) -> Values<EcoString, Value> {
        self.scope.values()
    }

    pub fn into_symbols(self) -> IntoKeys<EcoString, Value> {
        self.scope.into_keys()
    }

    pub fn into_values(self) -> IntoValues<EcoString, Value> {
        self.scope.into_values()
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<Value> {
        if let Some(val) = self.scope.get(name.as_ref()) {
            Some(val.clone())
        } else if let Some(p) = self.parent.as_ref() {
            p.get(name)
        } else {
            None
        }
    }

    pub fn try_get<T>(&self, name: impl AsRef<str>) -> Option<Result<T, Type>>
    where
        T: TypeCast + Clone,
    {
        if let Some(val) = self.scope.get(name.as_ref()) {
            Some(T::try_downcast_cloned(val))
        } else if let Some(p) = self.parent.as_ref() {
            p.try_get(name)
        } else {
            None
        }
    }

    pub fn insert(&mut self, name: EcoString, value: impl Into<Value>) -> Option<Value> {
        self.scope.insert(name, value.into())
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Value> {
        self.scope.remove(name.as_ref())
    }

    pub fn try_remove<T>(&mut self, name: impl AsRef<str>) -> Option<Result<T, Type>>
    where
        T: TypeCast,
    {
        let value = self.remove(name);

        value.map(T::try_downcast)
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = (EcoString, Value)>) {
        self.scope.extend(iter);
    }

    pub fn enter(&mut self) {
        let parent = std::mem::replace(self, Self::empty());
        self.parent = Some(Box::new(parent));
    }

    pub fn exit(&mut self) -> Self {
        let mut parent = self.parent.take().unwrap();
        std::mem::swap(self, &mut parent);
        *parent
    }

    pub fn into_inner(self) -> BTreeMap<EcoString, Value> {
        self.scope
    }
}
