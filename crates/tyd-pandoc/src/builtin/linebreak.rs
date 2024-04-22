use pandoc_ast as ir;
use tyd_render::{command::Command, error::EngineError};

use crate::{engine::PandocState, Call, PandocShape, Signature, Value};

pub struct LineBreak;

impl Command<PandocShape, PandocState> for LineBreak {
    fn signature(&self) -> Signature {
        Signature::new("linebreak")
    }

    fn run(&self, _call: Call, _ctx: &PandocState) -> Result<Value, EngineError> {
        Ok(Value::Inline(ir::Inline::LineBreak))
    }
}
