use std::collections::{BTreeMap, HashSet};
use tyd_syntax::Span;

use crate::{
    error::{EngineError, EngineErrorMessage},
    Shape, Signature, Type, Value,
};

pub type RawArgs<S> = BTreeMap<String, Arg<S>>;

#[derive(Debug, Clone)]
pub struct RawArgsBuilder<S: Shape>(RawArgs<S>);

impl<S: Shape> RawArgsBuilder<S> {
    pub fn new() -> Self {
        Self(RawArgs::new())
    }

    pub fn insert(&mut self, name: impl Into<String>, span: Span, value: Value<S>) {
        self.0.insert(
            name.into(),
            Arg {
                ty: value.ty(),
                span,
                value,
            },
        );
    }

    pub fn build(self) -> RawArgs<S> {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Arg<S: Shape> {
    span: Span,
    value: Value<S>,
    ty: Type,
}

#[derive(Debug, Clone)]
pub struct ValidArgs<S: Shape>(RawArgs<S>);

impl<S: Shape> ValidArgs<S> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn map(&mut self, name: impl AsRef<str>) -> BTreeMap<String, Value<S>> {
        let arg = self.0.remove(name.as_ref()).unwrap();

        // assert!(TypeChecker::<S>::check::<BTreeMap<String, Value<S>>>(
        //     arg.ty
        // ));

        arg.value.into_map().unwrap()
    }

    pub fn list(&mut self, name: impl AsRef<str>) -> Vec<Value<S>> {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_list()
            .unwrap()
    }

    pub fn bool(&mut self, name: impl AsRef<str>) -> bool {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_bool()
            .unwrap()
    }

    pub fn str(&mut self, name: impl AsRef<str>) -> String {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_string()
            .unwrap()
    }

    pub fn float(&mut self, name: impl AsRef<str>) -> f64 {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_float()
            .unwrap()
    }

    pub fn int(&mut self, name: impl AsRef<str>) -> i64 {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_int()
            .unwrap()
    }

    pub fn inline(&mut self, name: impl AsRef<str>) -> S::Inline {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_inline()
            .unwrap()
    }

    pub fn block(&mut self, name: impl AsRef<str>) -> S::Block {
        self.0
            .remove(name.as_ref())
            .unwrap()
            .value
            .into_block()
            .unwrap()
    }

    pub fn any(&mut self, name: impl AsRef<str>) -> Value<S> {
        self.0.remove(name.as_ref()).unwrap().value
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult<S: Shape>(pub ValidArgs<S>, pub Vec<EngineError>);

impl<S: Shape> ValidationResult<S> {
    pub fn has_errors(&self) -> bool {
        !self.1.is_empty()
    }
}

pub struct Validator<S: Shape> {
    span: Span,
    args: RawArgs<S>,
    signature: Signature<S>,
    errors: Vec<EngineError>,
}

impl<S: Shape> Validator<S> {
    pub fn new(span: Span, args: RawArgs<S>, signature: Signature<S>) -> Self {
        Self {
            span,
            args,
            signature,
            errors: Vec::new(),
        }
    }

    pub fn validate(self) -> ValidationResult<S> {
        use EngineErrorMessage::*;

        let Self {
            span,
            mut args,
            signature,
            mut errors,
        } = self;

        let allowed_keys = signature.params.keys().collect::<HashSet<_>>();
        let actual_keys = args.keys().collect::<HashSet<_>>();

        let diff = actual_keys
            .difference(&allowed_keys)
            .cloned()
            .cloned()
            .collect::<Vec<_>>();

        if !diff.is_empty() {
            errors.push(EngineError::new(span, WrongArguments(diff)));
        }

        for (key, param) in signature.params {
            if let Some(arg) = args.get(&key) {
                if arg.ty != param.ty {
                    errors.push(EngineError::new(
                        span,
                        WrongArgType {
                            key,
                            expected: param.ty,
                        },
                    ));
                }
            } else if let Some(value) = param.default {
                args.insert(
                    key,
                    Arg {
                        value,
                        ty: param.ty,
                        span,
                    },
                );
            } else {
                errors.push(EngineError::new(span, MissingArgument(key)));
            }
        }

        ValidationResult(ValidArgs(args), errors)
    }
}
