use std::fs;
use tyd_eval::{
    eval::Engine,
    render::{Output, Render},
    world::World,
};
use tyd_syntax::ast::Document;

use crate::{engine::PandocEngine, error::PandocError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    type Error = PandocError;
    type Engine = PandocEngine;

    fn render(
        doc: Document,
        world: World<Self::Engine>,
        output: Output,
    ) -> Result<(), Self::Error> {
        let engine = PandocEngine::from_world(world);
        let pandoc = engine.build(doc)?;
        let contents = pandoc.to_json();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => fs::write(path, contents)?,
        }

        Ok(())
    }
}
