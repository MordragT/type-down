use crate::{
    command::{Arg, Call, Command, Signature, UnverifiedCall},
    context::Context,
    error::{ArgumentError, EngineError},
    value::{Shape, Value},
};

pub struct List;

impl<S, C> Command<S, C> for List
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

        let mut list = Vec::new();
        let mut errors = Vec::new();

        for Arg { name, span, value } in call.args {
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
