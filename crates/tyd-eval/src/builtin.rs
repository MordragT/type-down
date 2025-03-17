use crate::{error::ArgumentError, ir::Arguments, tracer::Tracer, value::Value};

#[derive(Debug, Clone, Copy)]
pub struct List;

impl Into<Value> for List {
    fn into(self) -> Value {
        Value::Func(list)
    }
}

pub fn list(args: Arguments, tracer: &mut Tracer) -> Value {
    let Arguments {
        named,
        positional,
        span,
        source: _,
    } = args;

    for name in named.keys().cloned() {
        tracer.error(span, ArgumentError::UnknownNamed { name });
    }

    Value::List(positional)
}

#[derive(Debug, Clone, Copy)]
pub struct Map;

impl Into<Value> for Map {
    fn into(self) -> Value {
        Value::Func(map)
    }
}

pub fn map(args: Arguments, tracer: &mut Tracer) -> Value {
    let Arguments {
        named,
        positional,
        span,
        source: _,
    } = args;

    for (pos, _) in positional.iter().enumerate() {
        tracer.error(span, ArgumentError::UnknownPositional { pos });
    }

    Value::Map(named)
}
