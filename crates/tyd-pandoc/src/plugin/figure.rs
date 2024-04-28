use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    hir,
    plugin::{PluginFunc, Signature},
    ty::Type,
    value::Value,
};

use crate::{attr::AttrBuilder, engine::PandocEngine, visitor::PandocVisitor};

#[derive(Debug, Clone, Copy)]
pub struct Figure;

impl PluginFunc<PandocEngine> for Figure {
    fn signature() -> Signature<PandocEngine> {
        Signature::new("figure")
            .required("caption", Type::list(Type::Inline))
            .positional(Type::list(Type::Inline))
    }

    fn call(
        mut args: hir::Args<PandocEngine>,
        _engine: &mut PandocEngine,
        _visitor: &PandocVisitor,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let caption = args.remove_named::<Vec<ir::Inline>>("caption");
        let content = args.pop_positional::<Vec<ir::Inline>>();

        let caption = (None, vec![ir::Block::Plain(caption)]);
        let content = ir::Block::Plain(content);
        let block = ir::Block::Figure(AttrBuilder::empty(), caption, vec![content]);

        Ok(Value::Block(block))
    }
}
