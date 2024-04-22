use std::collections::BTreeMap;

use crate::{
    command::{Arg, Call, Command, Signature, UnverifiedCall},
    context::Context,
    error::{ArgumentError, EngineError},
    value::{Shape, Value},
};

pub struct Map;

impl<S, C> Command<S, C> for Map
where
    S: Shape,
    C: Context<S>,
{
    fn run(&self, _call: Call<S>, _ctx: &C) -> Result<Value<S>, EngineError> {
        unreachable!()
    }

    fn signature(&self) -> Signature<S> {
        unreachable!()
    }

    fn dispatch(
        &self,
        call: UnverifiedCall<S>,
        _ctx: &mut C,
    ) -> Result<Value<S>, Vec<EngineError>> {
        use ArgumentError::*;

        let mut map = BTreeMap::new();
        let mut errors = Vec::new();

        for Arg { name, span, value } in call.args {
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
