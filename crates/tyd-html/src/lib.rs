use std::{fs, io};

use builder::HtmlBuilder;
use miette::Diagnostic;
use thiserror::Error;
use tyd_render::{ContextError, Engine, Output, Render};
use tyd_syntax::{ast::visitor::Visitor, Ast};

pub mod builder;
pub mod document;
pub mod element;
pub mod stack;

// pub const INDENT: usize = 2;
pub const TAB: &str = "  ";
pub const NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub const DOCTYPE: &str = "<!DOCTYPE html>";
pub const LANGUAGE: &str = "en";

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::html::HtmlCompiler::compile))]
pub enum HtmlError {
    Io(#[from] io::Error),
    Call(#[from] ContextError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Render for HtmlCompiler {
    type Error = HtmlError;

    fn render(ast: &Ast, ctx: Engine, output: Output) -> Result<(), Self::Error> {
        let mut builder = HtmlBuilder::new(ctx);
        builder.visit_ast(ast)?;

        let doc = builder.build();
        let contents = doc.to_string();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => fs::write(path, contents)?,
        }

        Ok(())
    }
}

// TODO reuse pandoc html backend and expand from that onwards
// would also allow better indentation for html renderer as
// text elemments do not need to be considered anymore
