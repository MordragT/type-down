use crate::{error::EngineError, Shape, Value};

pub use argument::*;
pub use signature::*;

mod argument;
mod signature;

pub trait Command<S: Shape> {
    fn signature(&self) -> Signature<S>;
    fn run(&self, args: &mut ValidArgs<S>) -> Result<Value<S>, EngineError>;
}
