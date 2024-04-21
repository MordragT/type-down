use crate::{error::EngineError, Shape, Value};
use tyd_syntax::Span;

pub use argument::*;
pub use signature::*;

mod argument;
mod signature;

pub trait Command<S: Shape> {
    fn signature(&self) -> Signature<S>;
    fn run(&self, args: &mut Arguments<S>) -> Result<Value<S>, EngineError>;

    fn dispatch(&self, args: Vec<Arg<S>>, span: Span) -> Result<Value<S>, Vec<EngineError>> {
        let mut args = validate(self.signature(), args, span)?;

        let result = self.run(&mut args).map_err(|e| vec![e])?;
        assert!(args.is_empty());

        Ok(result)
    }
}
