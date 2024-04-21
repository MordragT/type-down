use pandoc_ast as ir;
use tyd_render::{error::EngineError, Command, Type};

use crate::{Arguments, PandocShape, Signature, Value};

pub struct SmallCaps;

impl Command<PandocShape> for SmallCaps {
    fn signature(&self) -> Signature {
        Signature::new("smallcaps").required("content", Type::list(Type::Inline))
    }

    fn run(&self, args: &mut Arguments) -> Result<Value, EngineError> {
        let content = args.remove_named::<Vec<ir::Inline>>("content");
        let inline = ir::Inline::SmallCaps(content);

        Ok(Value::Inline(inline))
    }
}
