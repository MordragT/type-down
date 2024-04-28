use crate::{
    error::{ArgumentError, EngineError},
    eval::Engine,
    hir,
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct List;

impl<E: Engine> Into<hir::Func<E>> for List {
    fn into(self) -> hir::Func<E> {
        hir::Func::new(list::<E>)
    }
}

pub fn list<E: Engine>(
    args: hir::Args<E>,
    engine: &mut E,
    _visitor: &E::Visitor,
) -> Option<Value<E>> {
    let hir::Args {
        named,
        positional,
        span: _,
    } = args;

    for hir::NamedArg {
        name,
        value: _,
        span,
    } in named
    {
        engine
            .tracer_mut()
            .error(EngineError::arg(span, ArgumentError::UnknownNamed { name }));
    }

    let list = positional
        .into_iter()
        .map(|arg| arg.value)
        .collect::<Vec<_>>();

    Some(list.into())
}
