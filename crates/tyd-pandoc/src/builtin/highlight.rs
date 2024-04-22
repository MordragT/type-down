use pandoc_ast as ir;
use tyd_render::{command::Command, error::EngineError, ty::Type};

use crate::{attr::AttrBuilder, engine::PandocState, Call, PandocShape, Signature, Value};

pub struct Highlight;

impl Command<PandocShape, PandocState> for Highlight {
    fn signature(&self) -> Signature {
        Signature::new("highlight").positional(Type::list(Type::Inline))
    }

    fn run(&self, call: Call, _ctx: &PandocState) -> Result<Value, EngineError> {
        let mut args = call.args;

        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::Span(AttrBuilder::new().class("mark").build(), content);

        Ok(Value::Inline(inline))
    }
}
