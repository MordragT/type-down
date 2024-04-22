use pandoc_ast as ir;
use tyd_render::{command::Command, error::EngineError, ty::Type};

use crate::{attr::AttrBuilder, engine::PandocState, Call, PandocShape, Signature, Value};

pub struct Figure;

impl Command<PandocShape, PandocState> for Figure {
    fn signature(&self) -> Signature {
        Signature::new("figure")
            .required("caption", Type::list(Type::Inline))
            .positional(Type::list(Type::Inline))
    }

    fn run(&self, call: Call, _ctx: &PandocState) -> Result<Value, EngineError> {
        let mut args = call.args;

        let caption = args.remove_named::<Vec<ir::Inline>>("caption");
        let content = args.pop_positional::<Vec<ir::Inline>>();

        let caption = (None, vec![ir::Block::Plain(caption)]);
        let content = ir::Block::Plain(content);
        let block = ir::Block::Figure(AttrBuilder::empty(), caption, vec![content]);

        Ok(Value::Block(block))
    }
}
