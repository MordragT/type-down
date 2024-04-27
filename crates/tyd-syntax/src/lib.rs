#![feature(array_windows)]

use std::{fmt::Debug, sync::Arc};

use chumsky::span::SimpleSpan;
use miette::NamedSource;

pub mod ast;
pub mod error;
pub mod parser;
pub mod visitor;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::error::*;
    pub use crate::parser::*;
    pub use crate::visitor::*;
    pub use crate::Source;
    pub use crate::Span;
}

pub type Span = SimpleSpan<usize>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Source(NamedSource<Arc<str>>);

impl Source {
    pub fn new(name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        Self(NamedSource::new(name, Arc::from(source.as_ref())))
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn as_str(&self) -> &str {
        self.0.inner()
    }

    pub fn named_source(&self) -> NamedSource<Arc<str>> {
        self.0.clone()
    }
}
