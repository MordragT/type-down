use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SmallCaps;

impl Plugin for SmallCaps {
    fn signature() -> Signature {
        Signature::new("smallcaps").positional(Type::Content)
    }

    fn call(mut args: ir::Arguments, tracer: &mut Tracer) -> Value {
        let content = args.pop::<ir::Content>().unwrap();
        let inline = ir::Inline::SmallCaps(content);

        Value::Inline(inline)
    }
}
