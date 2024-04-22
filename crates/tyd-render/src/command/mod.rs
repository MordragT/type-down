use crate::{
    context::{Context, SymbolTable},
    error::EngineError,
    value::{Shape, Value},
};
use tyd_syntax::Span;

pub use argument::*;
pub use signature::*;

mod argument;
mod signature;

pub trait Command<S, C>
where
    S: Shape,
    C: Context<S>,
{
    fn signature(&self) -> Signature<S>;
    fn run(&self, call: Call<S>, ctx: &C) -> Result<Value<S>, EngineError>;

    fn dispatch(&self, call: UnverifiedCall<S>, ctx: &mut C) -> Result<Value<S>, Vec<EngineError>> {
        let UnverifiedCall { args, span } = call;

        let args = validate(self.signature(), args, span)?;
        let call = Call { args, span };

        let result = self.run(call, ctx).map_err(|e| vec![e])?;

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct UnverifiedCall<S: Shape> {
    pub args: Vec<Arg<S>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Call<S: Shape> {
    pub args: Arguments<S>,
    pub span: Span,
}
