use crate::parse::ast::*;
use docx_rs::{DocumentChild, Docx, Paragraph, Run};
use miette::Diagnostic;
use std::{fs::File, io};
use thiserror::Error;

use self::visitor::Visitor;

use super::{Compiler, Context};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::docx::DocxCompiler::compile))]
pub enum DocxError {
    Io(#[from] io::Error),
    Docx(#[from] docx_rs::DocxError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocxCompiler;

impl Compiler for DocxCompiler {
    type Error = DocxError;
    type Context = Context;

    fn compile(ctx: Context, ast: &Ast) -> Result<(), Self::Error> {
        let mut builder = DocxBuilder::new();
        builder.visit_ast(ast);

        let docx = builder.build();
        let file = File::create(&ctx.dest)?;

        docx.build()
            .pack(file)
            .map_err(docx_rs::DocxError::ZipError)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DocxBuilder {
    stack: Vec<DocumentChild>,
}

impl DocxBuilder {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn build(self) -> Docx {
        let mut docx = Docx::new();

        for item in self.stack {
            docx = match item {
                DocumentChild::Paragraph(p) => docx.add_paragraph(*p),
                DocumentChild::Table(table) => docx.add_table(*table),
                _ => todo!(),
            };
        }

        docx
    }
}

impl Visitor for DocxBuilder {
    fn visit_raw(&mut self, raw: &Raw) {
        let p = Paragraph::new().add_run(Run::new().add_text(&raw.content));
        self.stack.push(DocumentChild::Paragraph(Box::new(p)));
    }
}
