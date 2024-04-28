use std::collections::BTreeMap;

use crate::{
    error::{ArgumentError, EngineError},
    eval::Engine,
    hir,
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct Map;

impl<E: Engine> Into<hir::Func<E>> for Map {
    fn into(self) -> hir::Func<E> {
        hir::Func::new(map::<E>)
    }
}

pub fn map<E: Engine>(
    args: hir::Args<E>,
    engine: &mut E,
    _visitor: &E::Visitor,
) -> Option<Value<E>> {
    let hir::Args {
        named,
        positional,
        span: _,
    } = args;

    for (pos, arg) in positional.into_iter().enumerate() {
        engine.tracer_mut().error(EngineError::arg(
            arg.span,
            ArgumentError::UnknownPositional { pos },
        ));
    }

    let map = named
        .into_iter()
        .map(|arg| (arg.name, arg.value))
        .collect::<BTreeMap<_, _>>();

    Some(map.into())
}
