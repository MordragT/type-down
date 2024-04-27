use pandoc_ast as ir;
use tyd_eval::{
    error::EngineError,
    eval::Machine,
    foundations::{Func, Signature, VerifiedCall},
    ty::Type,
    value::Value,
};

use crate::{attr::AttrBuilder, engine::PandocEngine};

#[derive(Debug, Clone, Copy)]
pub struct Figure;

impl Func<PandocEngine> for Figure {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("figure")
            .required("caption", Type::list(Type::Inline))
            .positional(Type::list(Type::Inline))
    }

    fn run(
        &self,
        call: VerifiedCall<PandocEngine>,
        _machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let mut args = call.args;

        let caption = args.remove_named::<Vec<ir::Inline>>("caption");
        let content = args.pop_positional::<Vec<ir::Inline>>();

        let caption = (None, vec![ir::Block::Plain(caption)]);
        let content = ir::Block::Plain(content);
        let block = ir::Block::Figure(AttrBuilder::empty(), caption, vec![content]);

        Ok(Value::Block(block))
    }
}
