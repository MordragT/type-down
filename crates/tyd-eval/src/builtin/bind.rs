use crate::{
    error::{ArgumentError, EngineError},
    eval::{Engine, Machine},
    foundations::{Arg, Call, Func},
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct Let;

impl<E: Engine> Func<E> for Let {
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

    fn dispatch(&self, call: Call<E>, machine: &mut Machine<E>) -> Option<Value<E>> {
        for Arg { name, span, value } in call.args {
            if let Some(name) = name {
                machine.scope.define_symbol(name, value);
            } else {
                machine.scope.error(EngineError::arg(
                    span,
                    ArgumentError::UnknownPositional { pos: 0 },
                ));
            }
        }

        None
    }
}
