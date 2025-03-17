use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct LineBreak;

impl Plugin for LineBreak {
    fn signature() -> Signature {
        Signature::new("linebreak")
    }

    fn call(_args: ir::Arguments, tracer: &mut Tracer) -> Value {
        Value::Inline(ir::Inline::LineBreak)
    }
}
