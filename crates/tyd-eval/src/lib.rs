//! # TypeDown Evaluation Library
//!
//! This library provides implementations for rendering content in various formats.

/// Provides built-in functionality and standard plugins
pub mod builtin;
/// Core engine implementation
pub mod engine;
/// Error handling structures and utilities
pub mod error;
/// Intermediate representation for parsed documents
pub mod ir;
/// Output formatting and compilation for various formats
pub mod render;
/// Variable and function scoping management
pub mod scope;
/// Execution stack implementation
pub mod stack;
/// Debug tracing capabilities
pub mod tracer;
/// Value and Type representation and manipulation
pub mod value;

//// Common imports for working with the library
///
/// This module re-exports the most commonly used types and traits
/// to simplify import statements in consumer code.
pub mod prelude {
    pub use crate::builtin::BuiltinPlugin;
    pub use crate::engine::{Engine, EngineResult};
    pub use crate::error::*;
    pub use crate::ir;
    pub use crate::render::{
        DocxCompiler, HtmlCompiler, Output, PandocCompiler, PdfCompiler, Render,
    };
    pub use crate::scope::Scope;
    pub use crate::tracer::Tracer;
    pub use crate::value::{Type, TypeCast, TypeChecker, Typed, Value};
    pub use crate::Plugin;
}

/// Interface for extension with custom functionality
///
/// Implementing this trait allows you to add new functions, types, and
/// behaviors. Plugins are initialized with a scope
/// which they can populate with their provided functionality.
pub trait Plugin {
    /// Initialize the plugin with the given scope
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope to register plugin functionality with
    fn init(scope: &mut crate::scope::Scope);
}
