use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    eval::Machine,
    foundations::{Func, Signature, VerifiedCall},
    value::Value,
};

use crate::engine::PandocEngine;

#[derive(Debug, Clone, Copy)]
pub struct HorizontalRule;

impl Func<PandocEngine> for HorizontalRule {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("hrule")
    }

    fn run(
        &self,
        _call: VerifiedCall<PandocEngine>,
        _machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let block = ir::Block::HorizontalRule;

        Ok(Value::Block(block))
    }
}
