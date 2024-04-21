use pandoc_ast as ir;
use tyd_render::{error::EngineError, Command};

use crate::{Arguments, PandocShape, Signature, Value};

pub struct LineBreak;

impl Command<PandocShape> for LineBreak {
    fn signature(&self) -> Signature {
        Signature::new("linebreak")
    }

    fn run(&self, _args: &mut Arguments) -> Result<Value, EngineError> {
        Ok(Value::Inline(ir::Inline::LineBreak))
    }
}
