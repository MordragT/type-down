use crate::{
    error::{ArgumentError, EngineError},
    eval::Engine,
    hir,
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct Let;

impl<E: Engine> Into<hir::Func<E>> for Let {
    fn into(self) -> hir::Func<E> {
        hir::Func::new(bind::<E>)
    }
}

pub fn bind<E: Engine>(
    args: hir::Args<E>,
    engine: &mut E,
    _visitor: &E::Visitor,
) -> Option<Value<E>> {
    let hir::Args {
        named,
        positional,
        span: _,
    } = args;

    for arg in named {
        engine.scopes_mut().define_symbol(arg.name, arg.value);
    }

    for (pos, arg) in positional.into_iter().enumerate() {
        engine.tracer_mut().error(EngineError::arg(
            arg.span,
            ArgumentError::UnknownPositional { pos },
        ));
    }

    None
}
