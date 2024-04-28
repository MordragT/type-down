use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    hir,
    plugin::{PluginFunc, Signature},
    ty::Type,
    value::Value,
};

use crate::{engine::PandocEngine, visitor::PandocVisitor};

#[derive(Debug, Clone, Copy)]
pub struct SmallCaps;

impl PluginFunc<PandocEngine> for SmallCaps {
    fn signature() -> Signature<PandocEngine> {
        Signature::new("smallcaps").positional(Type::list(Type::Inline))
    }

    fn call(
        mut args: hir::Args<PandocEngine>,
        _engine: &mut PandocEngine,
        _visitor: &PandocVisitor,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::SmallCaps(content);

        Ok(Value::Inline(inline))
    }
}
