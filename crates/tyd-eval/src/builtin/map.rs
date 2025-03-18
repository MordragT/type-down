use tyd_syntax::{source::Source, Span};

use crate::{error::ArgumentError, scope::Scope, stack::Stack, tracer::Tracer, value::Value};

#[derive(Debug, Clone, Copy)]
pub struct Map;

impl Into<Value> for Map {
    fn into(self) -> Value {
        Value::Func(map)
    }
}

pub fn map(stack: Stack, scope: Scope, _source: Source, span: Span, tracer: &mut Tracer) -> Value {
    for (pos, _) in stack.into_iter().enumerate() {
        tracer.source_warn(span, ArgumentError::UnknownPositional { pos });
    }

    Value::Map(scope.into_inner())
}
