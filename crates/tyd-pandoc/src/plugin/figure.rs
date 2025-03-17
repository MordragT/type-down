use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Figure;

impl Plugin for Figure {
    fn signature() -> Signature {
        Signature::new("figure")
            .required("caption", Type::Content)
            .positional(Type::Content)
    }

    fn call(mut args: ir::Arguments, tracer: &mut Tracer) -> Value {
        let caption = args.remove::<ir::Content>("caption").unwrap();
        let content = args.pop::<ir::Content>().unwrap();

        let caption = (None, vec![ir::Block::Plain(caption)]);
        let content = ir::Block::Plain(content);
        let block = ir::Block::Figure(ir::AttrBuilder::empty(), caption, vec![content]);

        Value::Block(block)
    }
}
