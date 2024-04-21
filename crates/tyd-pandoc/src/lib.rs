use std::sync::Arc;

use tyd_render::{Cast, Shape};

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
    fn cast(value: tyd_render::Value<PandocShape>) -> Self {
        value.into_inline().unwrap()
    }
}

impl Cast<PandocShape> for pandoc_ast::Block {
    fn cast(value: tyd_render::Value<PandocShape>) -> Self {
        value.into_block().unwrap()
    }
}

pub type Value = tyd_render::Value<PandocShape>;
pub type CommandBox = Arc<dyn tyd_render::Command<PandocShape>>;
pub type Signature = tyd_render::Signature<PandocShape>;
pub type Arguments = tyd_render::Arguments<PandocShape>;
