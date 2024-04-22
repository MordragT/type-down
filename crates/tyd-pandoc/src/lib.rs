use std::sync::Arc;

use engine::PandocState;
use tyd_render::command as cmd;
use tyd_render::value::{Cast, Shape, Value as RenderValue};

pub mod attr;
pub mod builtin;
pub mod engine;
pub mod error;
pub mod format;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PandocShape;

impl Shape for PandocShape {
    type Block = pandoc_ast::Block;
    type Inline = pandoc_ast::Inline;
}

impl Cast<PandocShape> for pandoc_ast::Inline {
    fn cast(value: RenderValue<PandocShape>) -> Self {
        value.into_inline().unwrap()
    }
}

impl Cast<PandocShape> for pandoc_ast::Block {
    fn cast(value: RenderValue<PandocShape>) -> Self {
        value.into_block().unwrap()
    }
}

pub type Value = RenderValue<PandocShape>;
pub type CommandBox = Arc<dyn cmd::Command<PandocShape, PandocState>>;
pub type Signature = cmd::Signature<PandocShape>;
pub type Call = cmd::Call<PandocShape>;
