use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct HorizontalRule;

impl Plugin for HorizontalRule {
    fn signature() -> Signature {
        Signature::new("hrule")
    }

    fn call(_args: ir::Arguments, tracer: &mut Tracer) -> Value {
        let block = ir::Block::HorizontalRule;

        Value::Block(block)
    }
}
