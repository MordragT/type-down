use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Underline;

impl Plugin for Underline {
    fn signature() -> Signature {
        Signature::new("underline").positional(Type::Content)
    }

    fn call(mut args: ir::Arguments, tracer: &mut Tracer) -> Value {
        let content = args.pop::<ir::Content>().unwrap();
        let inline = ir::Inline::Underline(content);

        Value::Inline(inline)
    }
}
