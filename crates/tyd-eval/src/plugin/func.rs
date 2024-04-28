use std::collections::HashSet;

use ecow::EcoString;
use tyd_syntax::Span;

use super::Signature;
use crate::{
    error::{ArgumentError, EngineError},
    eval::Engine,
    hir,
    value::Value,
};

pub trait PluginFunc<E: Engine> {
    fn signature() -> Signature<E>;
    fn call(
        args: hir::Args<E>,
        engine: &mut E,
        visitor: &E::Visitor,
    ) -> Result<Value<E>, EngineError>;
}

pub fn dispatch<E, F>(args: hir::Args<E>, engine: &mut E, visitor: &E::Visitor) -> Option<Value<E>>
where
    E: Engine,
    F: PluginFunc<E>,
{
    let signature = F::signature();
    let args = match validate(signature, args) {
        Ok(args) => args,
        Err(errs) => {
            engine.tracer_mut().errors(errs);
            return None;
        }
    };
    match F::call(args, engine, visitor) {
        Ok(val) => Some(val),
        Err(e) => {
            engine.tracer_mut().error(e);
            None
        }
    }
}

pub fn validate<E: Engine>(
    signature: Signature<E>,
    args: hir::Args<E>,
) -> Result<hir::Args<E>, Vec<EngineError>> {
    use ArgumentError::*;

    let hir::Args {
        named,
        positional,
        span,
    } = args;

    let mut errors = Vec::new();
    let mut args = hir::Args::new(span);

    for hir::NamedArg { name, value, span } in named {
        if let Err(e) = validate_named(&signature, &name, &value, span) {
            errors.push(e);
        } else {
            args.add_named(name, value, span);
        }
    }

    let mut pos = 0;
    for hir::PositionalArg { value, span } in positional {
        if let Err(e) = validate_positional(&signature, pos, &value, span) {
            errors.push(e);
        } else {
            args.add_positional(value, span);
        }
        pos += 1;
    }

    let gotten = args.names().collect::<HashSet<_>>();
    let required = signature.required_names().cloned().collect::<HashSet<_>>();

    for name in required.difference(&gotten) {
        let ty = signature.get_required(name).unwrap();

        errors.push(EngineError::arg(
            span,
            MissingRequired {
                name: name.clone(),
                ty,
            },
        ))
    }

    for pos in pos..signature.positonal_count() {
        let ty = signature.get_positional(pos).unwrap();

        errors.push(EngineError::arg(span, MissingPositional { pos, ty }))
    }

    let optional = signature.optional_names().cloned().collect::<HashSet<_>>();

    for name in optional.difference(&gotten) {
        let value = signature.get_default(name).unwrap().to_owned();
        args.add_named(name.clone(), value, span);
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(args)
    }
}

pub fn validate_named<E: Engine>(
    signature: &Signature<E>,
    name: &EcoString,
    value: &Value<E>,
    span: Span,
) -> Result<(), EngineError> {
    use ArgumentError::*;

    let ty = signature
        .get_required(&name)
        .or(signature.get_optional(&name))
        .ok_or(EngineError::arg(span, UnknownNamed { name: name.clone() }))?;

    let got = value.ty();

    if ty == got {
        Ok(())
    } else {
        Err(EngineError::arg(span, WrongType { got, expected: ty }))
    }
}

pub fn validate_positional<E: Engine>(
    signature: &Signature<E>,
    pos: usize,
    value: &Value<E>,
    span: Span,
) -> Result<(), EngineError> {
    use ArgumentError::*;

    let ty = signature
        .get_positional(pos)
        .ok_or(EngineError::arg(span, UnknownPositional { pos }))?;

    let got = value.ty();

    if ty == got {
        Ok(())
    } else {
        Err(EngineError::arg(span, WrongType { got, expected: ty }))
    }
}
