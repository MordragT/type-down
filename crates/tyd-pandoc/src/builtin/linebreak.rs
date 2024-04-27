use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    eval::Machine,
    foundations::{Func, Signature, VerifiedCall},
    value::Value,
};

use crate::engine::PandocEngine;

#[derive(Debug, Clone, Copy)]
pub struct LineBreak;

impl Func<PandocEngine> for LineBreak {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("linebreak")
    }

    fn run(
        &self,
        _call: VerifiedCall<PandocEngine>,
        _machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        Ok(Value::Inline(ir::Inline::LineBreak))
    }
}
