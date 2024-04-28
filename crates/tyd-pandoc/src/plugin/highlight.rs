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
pub struct Highlight;

impl PluginFunc<PandocEngine> for Highlight {
    fn signature() -> Signature<PandocEngine> {
        Signature::new("highlight").positional(Type::list(Type::Inline))
    }

    fn call(
        mut args: hir::Args<PandocEngine>,
        _engine: &mut PandocEngine,
        _visitor: &PandocVisitor,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::Span(AttrBuilder::new().class("mark").build(), content);

        Ok(Value::Inline(inline))
    }
}
