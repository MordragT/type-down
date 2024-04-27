use crate::{
    error::{ArgumentError, EngineError},
    eval::{Engine, Machine},
    value::Value,
};
use ecow::EcoString;
use std::{collections::HashSet, fmt::Debug};
use tyd_syntax::Span;

use super::{Arg, Args, Signature};

pub trait Func<E>: Debug
where
    E: Engine,
{
    fn signature(&self) -> Signature<E>;
    fn run(&self, call: VerifiedCall<E>, machine: &Machine<E>) -> Result<Value<E>, EngineError>;

    fn dispatch(&self, call: Call<E>, machine: &mut Machine<E>) -> Option<Value<E>> {
        let Call { args, span } = call;

        let args = match validate(self.signature(), args, span) {
            Ok(args) => args,
            Err(errs) => {
                machine.scope.errors(errs);
                return None;
            }
        };
        let call = VerifiedCall { args, span };

        match self.run(call, machine) {
            Ok(value) => Some(value),
            Err(e) => {
                machine.scope.error(e);
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerifiedCall<E: Engine> {
    pub args: Args<E>,
    pub span: Span,
}

pub struct Call<E: Engine> {
    pub args: Vec<Arg<E>>,
    pub span: Span,
}

pub fn validate<E: Engine>(
    signature: Signature<E>,
    args: Vec<Arg<E>>,
    span: Span,
) -> Result<Args<E>, Vec<EngineError>> {
    use ArgumentError::*;

    let mut errors = Vec::new();
    let mut arguments = Args::new();
    let mut pos = 0;

    for Arg { name, span, value } in args {
        if let Some(name) = name {
            if let Err(e) = validate_named(&signature, &name, &value, span) {
                errors.push(e);
            } else {
                arguments.add_named(name, value);
            }
        } else {
            if let Err(e) = validate_positional(&signature, pos, &value, span) {
                errors.push(e);
            } else {
                arguments.add_positional(value);
            }
            pos += 1;
        }
    }

    let gotten = arguments.names().cloned().collect::<HashSet<_>>();
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
        arguments.add_named(name, value);
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(arguments)
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
