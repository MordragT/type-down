use std::collections::BTreeMap;

use crate::{
    error::{ArgumentError, EngineError},
    Arg, Command, Shape, Value,
};

pub struct List;

impl<S: Shape> Command<S> for List {
    fn run(&self, _args: &mut crate::Arguments<S>) -> Result<Value<S>, EngineError> {
        unreachable!()
    }

    fn signature(&self) -> crate::Signature<S> {
        unreachable!()
    }

    fn dispatch(
        &self,
        args: Vec<Arg<S>>,
        _span: tyd_syntax::Span,
    ) -> Result<Value<S>, Vec<EngineError>> {
        use ArgumentError::*;

        let mut list = Vec::new();
        let mut errors = Vec::new();

        for Arg { name, span, value } in args {
            if let Some(name) = name {
                errors.push(EngineError::arg(span, UnknownNamed { name }));
            } else {
                list.push(value);
            }
        }

        if errors.is_empty() {
            Ok(Value::List(list))
        } else {
            Err(errors)
        }
    }
}

pub struct Dict;

impl<S: Shape> Command<S> for Dict {
    fn run(&self, _args: &mut crate::Arguments<S>) -> Result<Value<S>, EngineError> {
        unreachable!()
    }

    fn signature(&self) -> crate::Signature<S> {
        unreachable!()
    }

    fn dispatch(
        &self,
        args: Vec<Arg<S>>,
        _span: tyd_syntax::Span,
    ) -> Result<Value<S>, Vec<EngineError>> {
        use ArgumentError::*;

        let mut map = BTreeMap::new();
        let mut errors = Vec::new();

        for Arg { name, span, value } in args {
            if let Some(name) = name {
                map.insert(name, value);
            } else {
                errors.push(EngineError::arg(span, UnknownPositional { pos: 0 }));
            }
        }

        if errors.is_empty() {
            Ok(Value::Map(map))
        } else {
            Err(errors)
        }
    }
}
