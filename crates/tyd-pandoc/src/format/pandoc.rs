use std::fs;
use tyd_render::{Output, Render};
use tyd_syntax::{ast::Ast, visitor::Visitor};

use crate::{builder::PandocBuilder, error::PandocError, Content, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    type Error = PandocError;
    type Content = Content;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error> {
        let mut builder = PandocBuilder::new(ctx);
        builder.visit_ast(ast)?;

        let pandoc = builder.build();
        let contents = pandoc.to_json();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => fs::write(path, contents)?,
        }

        Ok(())
    }
}
