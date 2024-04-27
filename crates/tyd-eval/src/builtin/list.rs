use crate::{
    error::{ArgumentError, EngineError},
    eval::{Engine, Machine},
    foundations::{Arg, Call, Func},
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct List;

impl<E: Engine> Func<E> for List {
    fn signature(&self) -> crate::foundations::Signature<E> {
        unreachable!()
    }

    fn run(
        &self,
        call: crate::foundations::VerifiedCall<E>,
        machine: &Machine<E>,
    ) -> Result<Value<E>, EngineError> {
        unreachable!()
    }

    fn dispatch(&self, call: Call<E>, machine: &mut Machine<E>) -> Option<Value<E>> {
        let mut list = Vec::new();

        for Arg { name, span, value } in call.args {
            if let Some(name) = name {
                machine
                    .scope
                    .error(EngineError::arg(span, ArgumentError::UnknownNamed { name }));
            } else {
                list.push(value);
            }
        }

        Some(list.into())
    }
}
