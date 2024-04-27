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
pub struct Highlight;

impl Func<PandocEngine> for Highlight {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("highlight").positional(Type::list(Type::Inline))
    }

    fn run(
        &self,
        call: VerifiedCall<PandocEngine>,
        _machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let mut args = call.args;

        let content = args.pop_positional::<Vec<ir::Inline>>();
        let inline = ir::Inline::Span(AttrBuilder::new().class("mark").build(), content);

        Ok(Value::Inline(inline))
    }
}
