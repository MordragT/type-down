use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Highlight;

impl Plugin for Highlight {
    fn signature() -> Signature {
        Signature::new("highlight").positional(Type::Content)
    }

    fn call(mut args: ir::Arguments, tracer: &mut Tracer) -> Value {
        let content = args.pop::<ir::Content>().unwrap();
        let inline = ir::Inline::Span(ir::AttrBuilder::new().class("mark").build(), content);

        Value::Inline(inline)
    }
}
