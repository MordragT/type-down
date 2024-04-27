use std::fmt::Debug;
use tyd_syntax::{ast, visitor::Visitor};

use crate::value::Cast;

use super::Context;

/// The core component, responsible for typesetting
pub trait Engine: Sized + Clone + Visitor<State = Context<Self>> {
    type Inline: Debug + Clone + Cast<Self> + 'static;
    type Block: Debug + Clone + Cast<Self> + 'static;

    fn process_inline(&mut self, inline: &ast::Inline) -> Option<Self::Inline>;
    fn process_block(&mut self, block: &ast::Block) -> Option<Self::Block>;
}
