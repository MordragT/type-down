use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    hir,
    plugin::{PluginFunc, Signature},
    value::Value,
};

use crate::{engine::PandocEngine, visitor::PandocVisitor};

#[derive(Debug, Clone, Copy)]
pub struct LineBreak;

impl PluginFunc<PandocEngine> for LineBreak {
    fn signature() -> Signature<PandocEngine> {
        Signature::new("linebreak")
    }

    fn call(
        _args: hir::Args<PandocEngine>,
        _engine: &mut PandocEngine,
        _visitor: &PandocVisitor,
    ) -> Result<Value<PandocEngine>, EngineError> {
        Ok(Value::Inline(ir::Inline::LineBreak))
    }
}
