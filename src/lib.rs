//! # TypeDown Interpreter Libraries
//!
//! This library re-exports the core components of the TypeDown interpreter system:
//!
//! - `core`: Core types, traits, and utilities for the TypeDown language
//! - `eval`: Evaluation engine and runtime for TypeDown documents
//! - `syntax`: Syntax parsing, AST definitions, and lexical analysis
//!
//! These modules provide a complete implementation of the TypeDown language interpreter,
//! allowing embedding, extension, and use of TypeDown in Rust applications.

pub use tyd_core as core;
pub use tyd_eval as eval;
pub use tyd_syntax as syntax;
