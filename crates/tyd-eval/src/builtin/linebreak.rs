use tyd_syntax::{source::Source, Span};

use crate::{error::ArgumentError, ir, scope::Scope, stack::Stack, tracer::Tracer, value::Value};

#[derive(Debug, Clone, Copy)]
pub struct LineBreak;

impl Into<Value> for LineBreak {
    fn into(self) -> Value {
        Value::Func(linebreak)
    }
}

pub fn linebreak(
    stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    for (pos, _) in stack.into_iter().enumerate() {
        tracer.source_warn(span, ArgumentError::UnknownPositional { pos });
    }

    for name in scope.into_symbols() {
        tracer.source_warn(span, ArgumentError::UnknownNamed { name });
    }

    Value::Inline(ir::Inline::LineBreak)
}
