//! Core data structure manipulation library
//!
//! This crate provides a collection of modules for handling hierarchical representations of the AST.
//! It includes utilities for node identification, traversal and manipulation.

/// Document generation module
pub mod doc;
/// Node identification module
pub mod id;
/// Node type classification module
pub mod kind;
/// Metadata handling module
pub mod meta;
/// Base node structure module
pub mod node;
/// Tree structure module
pub mod tree;
/// Tree traversal and visitor pattern module
pub mod visit;

/// Common imports for working with the library
///
/// This module re-exports the most commonly used types and traits
/// to simplify import statements in consumer code.
pub mod prelude {
    pub use crate::doc::{Doc, DocBuilder};
    pub use crate::id::NodeId;
    pub use crate::kind::NodeKind;
    pub use crate::meta::*;
    pub use crate::node::Node;
    pub use crate::tree;
    pub use crate::visit::Visitor;
    pub use crate::{Full, TryAsMut, TryAsRef};
}

/// A tuple containing both a reference to a value and its node ID
///
/// This is commonly used when traversing a tree to maintain
/// both the node content and its unique identifier.
pub type Full<'a, T> = (&'a T, id::NodeId<T>);

/// Trait for attempting to get a reference to a specific variant in an enum
///
/// This trait enables safe downcasting from an enum to a specific variant's inner type.
pub trait TryAsRef<T> {
    /// Attempts to return a reference to the inner type if the enum variant matches
    ///
    /// # Returns
    /// - `Some(&T)` if the enum variant contains the requested type
    /// - `None` if the enum variant doesn't match
    fn try_as_ref(&self) -> Option<&T>;
}

/// Trait for attempting to get a mutable reference to a specific variant in an enum
///
/// This trait enables safe mutable downcasting from an enum to a specific variant's inner type.
pub trait TryAsMut<T> {
    /// Attempts to return a mutable reference to the inner type if the enum variant matches
    ///
    /// # Returns
    /// - `Some(&mut T)` if the enum variant contains the requested type
    /// - `None` if the enum variant doesn't match
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

/// Implements both `TryAsRef` and `TryAsMut` for an enum type
///
/// This macro generates implementations for trying to access specific variant types
/// through references or mutable references.
///
/// # Example
/// ```
/// enum MyEnum {
///     Variant1(String),
///     Variant2(usize),
/// }
///
/// impl_try_as!(MyEnum, Variant1(String), Variant2(usize));
/// ```
#[macro_export]
macro_rules! impl_try_as {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryAsRef<$variant_type> for $enum_type {
                fn try_as_ref(&self) -> Option<&$variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }

            impl TryAsMut<$variant_type> for $enum_type {
                fn try_as_mut(&mut self) -> Option<&mut $variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}

/// Implements `TryAsRef` for an enum type
///
/// This macro generates immutable reference access implementations for trying
/// to access specific variant types.
#[macro_export]
macro_rules! impl_try_as_ref {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryAsRef<$variant_type> for $enum_type {
                fn try_as_ref(&self) -> Option<&$variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}

/// Implements `TryAsMut` for an enum type
///
/// This macro generates mutable reference access implementations for trying
/// to access specific variant types.
#[macro_export]
macro_rules! impl_try_as_mut {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryAsMut<$variant_type> for $enum_type {
                fn try_as_mut(&mut self) -> Option<&mut $variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}

/// Implements `TryInto` for converting from an enum to its variant's inner type
///
/// This macro generates implementations to consume an enum and extract a specific
/// variant's inner value.
#[macro_export]
macro_rules! impl_try_into {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryInto<$variant_type> for $enum_type {
                type Error = ();

                fn try_into(self) -> Result<$variant_type, Self::Error> {
                    match self {
                        $enum_type::$variant(val) => Ok(val),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}

/// Implements `TryFrom` for converting from an enum to its variant's inner type
///
/// This macro generates implementations to consume an enum and extract a specific
/// variant's inner value.
#[macro_export]
macro_rules! impl_try_from {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryFrom<$enum_type> for $variant_type {
                type Error = ();

                fn try_from(value: $enum_type) -> Result<Self, Self::Error> {
                    match value {
                        $enum_type::$variant(val) => Ok(val),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}
