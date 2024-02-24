use std::{fs, io};

use builder::HtmlBuilder;
use miette::Diagnostic;
use thiserror::Error;
use tyd_render::{CallError, Context, Output, Render};
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
    Call(#[from] CallError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Render for HtmlCompiler {
    type Error = HtmlError;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error> {
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
