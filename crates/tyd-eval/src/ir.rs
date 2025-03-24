use std::collections::BTreeMap;

use ecow::EcoString;
use tyd_syntax::{source::Source, Span};

use crate::{scope::Scope, stack::Stack, tracer::Tracer, value::Value};

pub use pandoc_ast::*;

/// Represents a definition with an inline part and multiple blocks.
pub type Definition = (Vec<Inline>, Vec<Vec<Block>>);

/// A map-like structure for storing values with string keys.
pub type Map = BTreeMap<EcoString, Value>;
/// A list of values.
pub type List = Vec<Value>;
/// A collection of inline content elements.
pub type Content = Vec<Inline>;

/// A function type for evaluating expressions in the context of a stack, scope, source, and tracer.
pub type Func = fn(Stack, Scope, Source, Span, &mut Tracer) -> Value;

/// A builder for constructing Pandoc attributes.
///
/// Provides a fluent interface for building attributes with identifiers,
/// classes, and key-value pairs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttrBuilder {
    /// The identifier for the attribute.
    ident: String,
    /// A list of classes associated with the attribute.
    classes: Vec<String>,
    /// A list of key-value pairs for the attribute.
    pairs: Vec<(String, String)>,
}

impl AttrBuilder {
    /// Creates a new empty `AttrBuilder`.
    pub fn new() -> Self {
        Self {
            ident: String::new(),
            classes: Vec::new(),
            pairs: Vec::new(),
        }
    }

    /// Sets the identifier for the attribute.
    ///
    /// # Arguments
    ///
    /// * `ident` - The identifier to set
    pub fn ident(mut self, ident: impl Into<String>) -> Self {
        self.ident = ident.into();
        self
    }

    /// Conditionally sets the identifier if the provided option contains a value.
    ///
    /// # Arguments
    ///
    /// * `ident` - An optional identifier to set
    pub fn ident_opt(mut self, ident: Option<impl Into<String>>) -> Self {
        if let Some(ident) = ident {
            self.ident = ident.into();
        }
        self
    }

    /// Adds a class to the attribute.
    ///
    /// # Arguments
    ///
    /// * `class` - The class to add
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Conditionally adds a class if the provided option contains a value.
    ///
    /// # Arguments
    ///
    /// * `class` - An optional class to add
    pub fn class_opt(mut self, class: Option<impl Into<String>>) -> Self {
        if let Some(class) = class {
            self.classes.push(class.into())
        }
        self
    }

    /// Adds a key-value attribute pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The attribute key
    /// * `value` - The attribute value
    pub fn attr<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
        self
    }

    /// Adds a key-value attribute pair to an existing builder.
    ///
    /// Unlike `attr`, this method modifies the builder in place rather than
    /// returning a new instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The attribute key
    /// * `value` - The attribute value
    pub fn add_attr<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
    }

    /// Builds and returns the final Attr value.
    pub fn build(self) -> Attr {
        let Self {
            ident,
            classes,
            pairs,
        } = self;
        (ident, classes, pairs)
    }

    /// Creates an empty Attr value without using the builder pattern.
    pub fn empty() -> Attr {
        (String::new(), Vec::new(), Vec::new())
    }
}
