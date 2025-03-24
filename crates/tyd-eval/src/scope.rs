use ecow::EcoString;
use std::collections::{
    btree_map::{IntoKeys, IntoValues, Keys, Values},
    BTreeMap,
};

use crate::{
    ir,
    value::{Type, TypeCast, Value},
    Plugin,
};

/// Represents a scoped environment for storing and retrieving named values.
///
/// A `Scope` can have a parent scope, enabling nested scope chains for variable
/// lookups. Values are stored in a tree map for ordered access.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    /// Optional parent scope for hierarchical lookups
    parent: Option<Box<Self>>,
    /// The map of names to values in the current scope level
    scope: BTreeMap<EcoString, Value>,
}

impl Scope {
    /// Creates a new scope with the provided scope as its parent.
    ///
    /// This creates a new scope level that inherits from the existing scope.
    ///
    /// # Arguments
    /// * `from` - The parent scope to inherit from
    pub fn new(from: Self) -> Self {
        Self {
            parent: Some(Box::new(from)),
            scope: ir::Map::new(),
        }
    }

    /// Creates an empty scope with no parent.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Initializes this scope with a plugin's predefined values.
    ///
    /// # Type Parameters
    /// * `P` - A type that implements the `Plugin` trait
    ///
    /// # Returns
    /// * `&mut Self` - Self reference for method chaining
    pub fn register<P: Plugin>(&mut self) -> &mut Self {
        P::init(self);
        self
    }

    /// Adds a name-value pair to the scope.
    ///
    /// # Arguments
    /// * `name` - The name to associate with the value
    /// * `value` - The value to store
    ///
    /// # Returns
    /// * `&mut Self` - Self reference for method chaining
    pub fn with(&mut self, name: impl Into<EcoString>, value: impl Into<Value>) -> &mut Self {
        self.insert(name.into(), value);
        self
    }

    /// Removes all name-value pairs from the current scope level.
    pub fn clear(&mut self) {
        self.scope.clear();
    }

    /// Returns an iterator over all symbol names in the current scope level.
    pub fn symbols(&self) -> Keys<EcoString, Value> {
        self.scope.keys()
    }

    /// Returns an iterator over all values in the current scope level.
    pub fn values(&self) -> Values<EcoString, Value> {
        self.scope.values()
    }

    /// Consumes the scope and returns an iterator over all symbol names.
    pub fn into_symbols(self) -> IntoKeys<EcoString, Value> {
        self.scope.into_keys()
    }

    /// Consumes the scope and returns an iterator over all values.
    pub fn into_values(self) -> IntoValues<EcoString, Value> {
        self.scope.into_values()
    }

    /// Looks up a value by name, searching in parent scopes if not found in current scope.
    ///
    /// # Arguments
    /// * `name` - The name to look up
    ///
    /// # Returns
    /// * `Option<Value>` - The value if found, or None
    pub fn get(&self, name: impl AsRef<str>) -> Option<Value> {
        if let Some(val) = self.scope.get(name.as_ref()) {
            Some(val.clone())
        } else if let Some(p) = self.parent.as_ref() {
            p.get(name)
        } else {
            None
        }
    }

    /// Attempts to retrieve a value by name and downcast it to the specified type.
    ///
    /// Searches in parent scopes if the name is not found in the current scope.
    ///
    /// # Type Parameters
    /// * `T` - The type to cast the value to, must implement TypeCast and Clone
    ///
    /// # Arguments
    /// * `name` - The name to look up
    ///
    /// # Returns
    /// * `Option<Result<T, Type>>` - None if not found, or a Result containing either
    ///   the successfully cast value or the expected type that failed casting
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

    /// Inserts a name-value pair into the current scope level.
    ///
    /// # Arguments
    /// * `name` - The name to associate with the value
    /// * `value` - The value to store
    ///
    /// # Returns
    /// * `Option<Value>` - The previous value associated with the name, if any
    pub fn insert(&mut self, name: EcoString, value: impl Into<Value>) -> Option<Value> {
        self.scope.insert(name, value.into())
    }

    /// Removes a name-value pair from the current scope level.
    ///
    /// # Arguments
    /// * `name` - The name to remove
    ///
    /// # Returns
    /// * `Option<Value>` - The value that was removed, if any
    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Value> {
        self.scope.remove(name.as_ref())
    }

    /// Removes a name-value pair from the current scope level and attempts to
    /// downcast it to the specified type.
    ///
    /// # Type Parameters
    /// * `T` - The type to cast the value to, must implement TypeCast
    ///
    /// # Arguments
    /// * `name` - The name to remove
    ///
    /// # Returns
    /// * `Option<Result<T, Type>>` - None if not found, or a Result containing either
    ///   the successfully cast value or the expected type that failed casting
    pub fn try_remove<T>(&mut self, name: impl AsRef<str>) -> Option<Result<T, Type>>
    where
        T: TypeCast,
    {
        let value = self.remove(name);

        value.map(T::try_downcast)
    }

    /// Adds multiple name-value pairs to the current scope level.
    ///
    /// # Arguments
    /// * `iter` - An iterator yielding name-value pairs
    pub fn extend(&mut self, iter: impl IntoIterator<Item = (EcoString, Value)>) {
        self.scope.extend(iter);
    }

    /// Creates a new nested scope level, making the current scope the parent.
    pub fn enter(&mut self) {
        let parent = std::mem::replace(self, Self::empty());
        self.parent = Some(Box::new(parent));
    }

    /// Exits the current scope level, restoring the parent scope.
    ///
    /// # Returns
    /// * `Self` - The previous scope that was exited
    pub fn exit(&mut self) -> Self {
        let mut parent = self.parent.take().unwrap();
        std::mem::swap(self, &mut parent);
        *parent
    }

    /// Consumes the scope and returns the underlying map of names to values.
    ///
    /// # Returns
    /// * `BTreeMap<EcoString, Value>` - The underlying map from the current scope level
    pub fn into_inner(self) -> BTreeMap<EcoString, Value> {
        self.scope
    }
}
