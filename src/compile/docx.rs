use crate::parse::ast::*;
use docx_rs::{
    DocumentChild, Docx, Paragraph, Run, Table as DocxTable, TableCell, TableRow as DocxTableRow,
};
use miette::Diagnostic;
use std::{fs::File, io};
use thiserror::Error;

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

    fn compile(ctx: &Context, ast: &Ast) -> Result<(), Self::Error> {
        let title = &ctx.title;

        let docx = ast.to_docx();

        let file = File::create(&ctx.dest)?;
        docx.build()
            .pack(file)
            .map_err(docx_rs::DocxError::ZipError)?;

        Ok(())
    }
}

impl Ast {
    pub fn to_docx(&self) -> Docx {
        let mut docx = Docx::new();

        for block in &self.blocks {
            let child = block.to_docx();
            docx = match child {
                DocumentChild::Table(table) => docx.add_table(*table),
                DocumentChild::Paragraph(paragraph) => docx.add_paragraph(*paragraph),
                _ => todo!(),
            };
        }

        docx
    }
}

impl Block {
    pub fn to_docx(&self) -> DocumentChild {
        match &self {
            Block::Raw(raw) => DocumentChild::Paragraph(Box::new(raw.to_docx())),
            Block::Table(table) => DocumentChild::Table(Box::new(table.to_docx())),
            _ => todo!(),
        }
    }
}

impl Raw {
    pub fn to_docx(&self) -> Paragraph {
        Paragraph::new().add_run(Run::new().add_text(&self.content))
    }
}

impl Table {
    pub fn to_docx(&self) -> DocxTable {
        DocxTable::new(self.rows.iter().map(|row| row.to_docx()).collect())
    }
}

impl TableRow {
    pub fn to_docx(&self) -> DocxTableRow {
        DocxTableRow::new(
            self.cells
                .iter()
                .map(|elements| elements.to_docx_cell())
                .collect(),
        )
    }
}

impl Elements {
    pub fn to_docx_cell(&self) -> TableCell {
        let mut paragraph = Paragraph::new();

        for el in &self.0 {
            paragraph = paragraph.add_run(el.to_docx());
        }

        TableCell::new().add_paragraph(paragraph)
    }
}

impl Element {
    pub fn to_docx(&self) -> Run {
        todo!()
    }
}
