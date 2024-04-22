use pandoc_ast as ir;
use tyd_render::{command::Command, error::EngineError, ty::Type};

use crate::{engine::PandocState, Call, PandocShape, Signature, Value};

pub struct SmallCaps;

impl Command<PandocShape, PandocState> for SmallCaps {
    fn signature(&self) -> Signature {
        Signature::new("smallcaps").positional(Type::list(Type::Inline))
    }

    fn run(&self, call: Call, _ctx: &PandocState) -> Result<Value, EngineError> {
        let mut args = call.args;

        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::SmallCaps(content);

        Ok(Value::Inline(inline))
    }
}
