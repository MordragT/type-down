use tyd_syntax::{source::Source, Span};

use crate::{
    error::{ArgumentError, TypeError},
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    ty::Type,
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct SmallCaps;

impl Into<Value> for SmallCaps {
    fn into(self) -> Value {
        Value::Func(smallcaps)
    }
}

pub fn smallcaps(
    mut stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let content = match stack.try_pop::<ir::Content>() {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Content,
                },
            );
            return Value::None;
        }
        None => {
            tracer.source_error(
                span,
                ArgumentError::MissingPositional {
                    pos: 0,
                    ty: Type::Content,
                },
            );
            return Value::None;
        }
    };

    for (pos, _) in stack.into_iter().enumerate() {
        tracer.source_warn(span, ArgumentError::UnknownPositional { pos: pos + 1 });
    }

    for name in scope.into_symbols() {
        tracer.source_warn(span, ArgumentError::UnknownNamed { name });
    }

    let inline = ir::Inline::SmallCaps(content);

    Value::Inline(inline)
}
