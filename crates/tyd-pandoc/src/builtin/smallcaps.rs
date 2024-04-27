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
pub struct SmallCaps;

impl Func<PandocEngine> for SmallCaps {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("smallcaps").positional(Type::list(Type::Inline))
    }

    fn run(
        &self,
        call: VerifiedCall<PandocEngine>,
        _machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let mut args = call.args;

        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::SmallCaps(content);

        Ok(Value::Inline(inline))
    }
}
