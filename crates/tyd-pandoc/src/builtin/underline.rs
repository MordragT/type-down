use pandoc_ast as ir;
use tyd_render::{error::EngineError, Command, Type};

use crate::{Arguments, PandocShape, Signature, Value};

pub struct Underline;

impl Command<PandocShape> for Underline {
    fn signature(&self) -> Signature {
        Signature::new("underline").required("content", Type::list(Type::Inline))
    }

    fn run(&self, args: &mut Arguments) -> Result<Value, EngineError> {
        let content = args.remove_named::<Vec<ir::Inline>>("content");
        let inline = ir::Inline::Underline(content);

        Ok(Value::Inline(inline))
    }
}
