use tyd_syntax::{source::Source, Span};

use crate::{error::ArgumentError, scope::Scope, stack::Stack, tracer::Tracer, value::Value};

#[derive(Debug, Clone, Copy)]
pub struct List;

impl Into<Value> for List {
    fn into(self) -> Value {
        Value::Func(list)
    }
}

pub fn list(stack: Stack, scope: Scope, _source: Source, span: Span, tracer: &mut Tracer) -> Value {
    for name in scope.into_symbols() {
        tracer.source_warn(span, ArgumentError::UnknownNamed { name });
    }

    Value::List(stack.into_inner())
}
