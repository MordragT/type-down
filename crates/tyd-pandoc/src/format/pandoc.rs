use std::fs;
use tyd_render::{Output, Render};
use tyd_syntax::ast::Ast;

use crate::{
    engine::{PandocEngine, PandocState},
    error::PandocError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    type Error = PandocError;
    type Context = PandocState;

    fn render(ast: &Ast, ctx: Self::Context, output: Output) -> Result<(), Self::Error> {
        let engine = PandocEngine::new();
        let pandoc = engine.build(ctx, ast)?;
        let contents = pandoc.to_json();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => fs::write(path, contents)?,
        }

        Ok(())
    }
}
