use tyd_doc::{id::NodeId, meta::MetaContainer};
use tyd_syntax::{Span, SpanMetadata};

use crate::error::{EngineError, EngineMessage};

#[derive(Debug, Clone)]
pub struct Tracer {
    spans: SpanMetadata,
    errors: Vec<EngineError>,
}

impl Tracer {
    pub fn new(spans: SpanMetadata) -> Self {
        Self {
            spans,
            errors: Vec::new(),
        }
    }

    pub fn span<T>(&self, id: NodeId<T>) -> Span {
        *self.spans.get(id).inner_ref()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn node_error<T>(&mut self, id: NodeId<T>, msg: impl Into<EngineMessage>) {
        let span = self.span(id);
        self.errors.push(EngineError::new(span, msg.into()));
    }

    pub fn push(&mut self, e: EngineError) {
        self.errors.push(e)
    }

    pub fn error(&mut self, span: Span, msg: impl Into<EngineMessage>) {
        self.errors.push(EngineError::new(span, msg.into()));
    }

    pub fn errors(&mut self, errs: impl IntoIterator<Item = EngineError>) {
        self.errors.extend(errs)
    }

    pub fn into_errors(self) -> Vec<EngineError> {
        self.errors
    }

    pub fn drain_errors(&mut self) -> impl Iterator<Item = EngineError> + '_ {
        self.errors.drain(..)
    }
}
