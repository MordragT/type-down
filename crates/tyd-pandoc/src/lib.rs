use pandoc_ast::Inline;

pub mod attr;
pub mod builder;
pub mod builtin;
pub mod error;
pub mod format;

pub type Content = Vec<Inline>;
pub type Context = tyd_render::Context<Content>;
