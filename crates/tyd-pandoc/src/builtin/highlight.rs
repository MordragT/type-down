use pandoc_ast as ir;
use tyd_render::{error::EngineError, Command, Type};

use crate::{attr::AttrBuilder, Arguments, PandocShape, Signature, Value};

pub struct Highlight;

impl Command<PandocShape> for Highlight {
    fn signature(&self) -> Signature {
        Signature::new("highlight").required("content", Type::list(Type::Inline))
    }

    fn run(&self, args: &mut Arguments) -> Result<Value, EngineError> {
        let content = args.remove_named::<Vec<ir::Inline>>("content");
        let inline = ir::Inline::Span(AttrBuilder::new().class("mark").build(), content);

        Ok(Value::Inline(inline))
    }
}
