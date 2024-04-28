use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    hir,
    plugin::{PluginFunc, Signature},
    value::Value,
};

use crate::{engine::PandocEngine, visitor::PandocVisitor};

#[derive(Debug, Clone, Copy)]
pub struct HorizontalRule;

impl PluginFunc<PandocEngine> for HorizontalRule {
    fn signature() -> Signature<PandocEngine> {
        Signature::new("hrule")
    }

    fn call(
        _args: hir::Args<PandocEngine>,
        _engine: &mut PandocEngine,
        _visitor: &PandocVisitor,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let block = ir::Block::HorizontalRule;

        Ok(Value::Block(block))
    }
}
