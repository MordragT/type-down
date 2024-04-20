use std::sync::Arc;

use tyd_render::Shape;

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

pub type Value = tyd_render::Value<PandocShape>;
pub type CommandBox = Arc<dyn tyd_render::Command<PandocShape>>;
pub type Signature = tyd_render::Signature<PandocShape>;
pub type ValidArgs = tyd_render::ValidArgs<PandocShape>;
