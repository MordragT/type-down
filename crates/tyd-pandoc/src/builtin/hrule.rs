use pandoc_ast as ir;
use tyd_render::{command::Command, error::EngineError};

use crate::{engine::PandocState, Call, PandocShape, Signature, Value};

pub struct HorizontalRule;

impl Command<PandocShape, PandocState> for HorizontalRule {
    fn signature(&self) -> Signature {
        Signature::new("hrule")
    }

    fn run(&self, _call: Call, _ctx: &PandocState) -> Result<Value, EngineError> {
        let block = ir::Block::HorizontalRule;

        Ok(Value::Block(block))
    }
}
