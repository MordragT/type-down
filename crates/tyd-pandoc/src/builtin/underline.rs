use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    eval::Machine,
    foundations::{Func, Signature, VerifiedCall},
    ty::Type,
    value::Value,
};

use crate::engine::PandocEngine;

#[derive(Debug, Clone, Copy)]
pub struct Underline;

impl Func<PandocEngine> for Underline {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("underline").positional(Type::list(Type::Inline))
    }

    fn run(
        &self,
        call: VerifiedCall<PandocEngine>,
        _machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let mut args = call.args;

        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::Underline(content);

        Ok(Value::Inline(inline))
    }
}
