use std::fmt::Debug;
use tyd_syntax::{ast, visitor::Visitor};

use super::{Scopes, Tracer};
use crate::{value::Cast, world::World};

/// The core component, responsible for typesetting
pub trait Engine: Sized + Clone {
    type Inline: Debug + Clone + Cast<Self> + 'static;
    type Block: Debug + Clone + Cast<Self> + 'static;
    type Visitor: Debug + Clone + Visitor<State = Self>;

    fn eval_inline(
        &mut self,
        visitor: &Self::Visitor,
        inline: &ast::Inline,
    ) -> Option<Self::Inline>;
    fn eval_block(&mut self, visitor: &Self::Visitor, block: &ast::Block) -> Option<Self::Block>;

    fn world(&self) -> World<Self>;
    fn scopes(&self) -> &Scopes<Self>;
    fn scopes_mut(&mut self) -> &mut Scopes<Self>;
    fn tracer(&self) -> &Tracer;
    fn tracer_mut(&mut self) -> &mut Tracer;
}
