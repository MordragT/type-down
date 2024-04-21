use std::collections::HashSet;
use tyd_syntax::Span;

use crate::{
    error::{ArgumentError, EngineError},
    Cast, Shape, Signature, Value,
};

#[derive(Debug, Clone)]
pub struct Arguments<S: Shape> {
    named: Vec<(String, Value<S>)>,
    positional: Vec<Value<S>>,
}

impl<S: Shape> Arguments<S> {
    pub fn new() -> Self {
        Self {
            named: Vec::new(),
            positional: Vec::new(),
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.named.iter().map(|(n, _)| n)
    }

    pub fn add_named(&mut self, name: impl Into<String>, value: impl Into<Value<S>>) {
        self.named.push((name.into(), value.into()))
    }

    pub fn add_positional(&mut self, value: impl Into<Value<S>>) {
        self.positional.push(value.into())
    }

    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }

    pub fn remove_named<T: Cast<S>>(&mut self, name: impl AsRef<str>) -> T {
        let pos = self
            .named
            .iter()
            .position(|(n, _)| n == name.as_ref())
            .unwrap();
        let (_, value) = self.named.remove(pos);
        T::cast(value)
    }

    pub fn remove_positonal<T: Cast<S>>(&mut self, pos: usize) -> T {
        let value = self.positional.remove(pos);
        T::cast(value)
    }
}

pub struct Arg<S: Shape> {
    pub name: Option<String>,
    pub span: Span,
    pub value: Value<S>,
}

pub fn validate<S: Shape>(
    signature: Signature<S>,
    args: Vec<Arg<S>>,
    span: Span,
) -> Result<Arguments<S>, Vec<EngineError>> {
    use ArgumentError::*;

    let mut errors = Vec::new();
    let mut arguments = Arguments::new();
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

    let optional = signature.optional_names().cloned().collect::<HashSet<_>>();

    for name in optional.difference(&gotten) {
        let value = signature.get_default(name).unwrap();
        arguments.add_named(name, value);
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(arguments)
    }
}

pub fn validate_named<S: Shape>(
    signature: &Signature<S>,
    name: &String,
    value: &Value<S>,
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

pub fn validate_positional<S: Shape>(
    signature: &Signature<S>,
    pos: usize,
    value: &Value<S>,
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
