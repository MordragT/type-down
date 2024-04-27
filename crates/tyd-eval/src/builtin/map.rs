use std::collections::BTreeMap;

use crate::{
    error::{ArgumentError, EngineError},
    eval::{Engine, Machine},
    foundations::{Arg, Func},
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct Map;

impl<E: Engine> Func<E> for Map {
    fn run(
        &self,
        call: crate::foundations::VerifiedCall<E>,
        machine: &Machine<E>,
    ) -> Result<Value<E>, EngineError> {
        unreachable!()
    }

    fn signature(&self) -> crate::foundations::Signature<E> {
        unreachable!()
    }

    fn dispatch(
        &self,
        call: crate::foundations::Call<E>,
        machine: &mut Machine<E>,
    ) -> Option<Value<E>> {
        let mut map = BTreeMap::new();

        for Arg { name, span, value } in call.args {
            if let Some(name) = name {
                map.insert(name, value);
            } else {
                machine.scope.error(EngineError::arg(
                    span,
                    ArgumentError::UnknownPositional { pos: 0 },
                ));
            }
        }

        Some(map.into())
    }
}
