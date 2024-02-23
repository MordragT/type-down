use html_writer::{
    tags::{BodyTag, HeadTag},
    DynHtmlElement, HtmlDocument, HtmlElement, HtmlStack,
};
use miette::Diagnostic;
use thiserror::Error;

use self::visitor::{
    walk_blockquote, walk_emphasis, walk_enclosed, walk_heading, walk_paragraph, walk_quote,
    walk_strikethrough, walk_strong, walk_table, Visitor,
};

use super::{Compiler, Context};
use crate::parse::ast::*;
use std::{fs, io};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::html::HtmlCompiler::compile))]
pub struct HtmlError(#[from] pub io::Error);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Compiler for HtmlCompiler {
    type Error = HtmlError;

    fn compile(ctx: &Context, ast: &Ast) -> Result<(), Self::Error> {
        let mut builder = HtmlBuilder::new(&ctx.title);
        builder.visit_ast(ast);

        let doc = builder.build();
        let contents = doc.to_string();

        fs::write(&ctx.dest, contents)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlBuilder {
    head: HtmlElement<HeadTag>,
    body: HtmlElement<BodyTag>,
    stack: HtmlStack,
}

impl HtmlBuilder {
    pub fn new(title: &str) -> Self {
        let head = HtmlElement::head()
            .child(HtmlElement::stylesheet(
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css",
            ))
            .with_title(title);

        Self {
            head,
            body: HtmlElement::body(),
            stack: HtmlStack::new(),
        }
    }

    pub fn build(self) -> HtmlDocument {
        let Self { head, body, stack } = self;

        assert!(stack.is_empty());

        let body = body
            .child(HtmlElement::script().attribute(
                "src",
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js",
            ))
            .child(HtmlElement::script().attribute(
                "src",
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js",
            ))
            .child(HtmlElement::script().child("hljs.highlightAll();"));

        HtmlDocument::new()
            .default_doctype()
            .default_language()
            .default_namespace()
            .child(head)
            .child(body)
    }
}

impl Visitor for HtmlBuilder {
    fn visit_raw(&mut self, raw: &Raw) {
        let mut code = HtmlElement::code().child(&raw.content);

        if let Some(lang) = &raw.lang {
            code.add_class(format!("language-{lang}"));
        }

        let pre = HtmlElement::pre().child(code);
        self.body.add_child(pre);
    }

    fn visit_heading(&mut self, heading: &Heading) {
        let h = DynHtmlElement::new(&format!("h{}", heading.level));
        let pos = self.stack.push_open(h);

        walk_heading(self, heading);

        let h = self.stack.fold(pos);
        self.body.add_child(h);
    }

    fn visit_list(&mut self, list: &List) {
        let pos = self.stack.push_open(HtmlElement::ul());

        for line in &list.lines {
            let pos = self.stack.push_open(HtmlElement::li());

            self.visit_line(line);

            self.stack.fold_push(pos)
        }

        let ul = self.stack.fold(pos);
        self.body.add_child(ul);
    }

    fn visit_ordered_list(&mut self, ordered_list: &OrderedList) {
        let pos = self.stack.push_open(HtmlElement::ol());

        for line in &ordered_list.lines {
            let pos = self.stack.push_open(HtmlElement::li());

            self.visit_line(line);

            self.stack.fold_push(pos)
        }

        let ol = self.stack.fold(pos);
        self.body.add_child(ol);
    }

    fn visit_table(&mut self, table: &Table) {
        let pos = self.stack.push_open(HtmlElement::table());

        walk_table(self, table);

        let tbl = self.stack.fold(pos);
        self.body.add_child(tbl);
    }

    fn visit_table_row(&mut self, table_row: &TableRow) {
        let pos = self.stack.push_open(HtmlElement::tr());

        for cell in &table_row.cells {
            let pos = self.stack.push_open(HtmlElement::td());

            self.visit_elements(cell);

            self.stack.fold_push(pos);
        }

        self.stack.fold_push(pos);
    }

    fn visit_blockquote(&mut self, blockquote: &Blockquote) {
        let pos = self.stack.push_open(HtmlElement::blockquote());

        walk_blockquote(self, blockquote);

        let blqt = self.stack.fold(pos);
        self.body.add_child(blqt)
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) {
        let pos = self.stack.push_open(HtmlElement::p());

        walk_paragraph(self, paragraph);

        let p = self.stack.fold(pos);
        self.body.add_child(p);
    }

    fn visit_quote(&mut self, quote: &Quote) {
        let pos = self.stack.push_open(HtmlElement::q());

        walk_quote(self, quote);

        self.stack.fold_push(pos);
    }

    fn visit_strikethrough(&mut self, strikethrough: &Strikethrough) {
        let pos = self.stack.push_open(HtmlElement::del());

        walk_strikethrough(self, strikethrough);

        self.stack.fold_push(pos);
    }

    fn visit_strong(&mut self, strong: &Strong) {
        let pos = self.stack.push_open(HtmlElement::strong());

        walk_strong(self, strong);

        self.stack.fold_push(pos);
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) {
        let pos = self.stack.push_open(HtmlElement::em());

        walk_emphasis(self, emphasis);

        self.stack.fold_push(pos);
    }

    fn visit_enclosed(&mut self, enclosed: &Enclosed) {
        let pos = self.stack.push_open(HtmlElement::div());

        walk_enclosed(self, enclosed);

        self.stack.fold_push(pos);
    }

    fn visit_link(&mut self, link: &Link) {
        let a = HtmlElement::a().href(&link.link);

        if let Some(elements) = &link.elements {
            let pos = self.stack.push_open(a);

            self.visit_elements(elements);

            self.stack.fold_push(pos);
        } else {
            self.stack.push_close(a.child(&link.link));
        }
    }

    fn visit_escape(&mut self, escape: &Escape) {
        self.stack.add_content(escape.0.to_string());
    }

    fn visit_monospace(&mut self, monospace: &Monospace) {
        self.stack
            .push_close(HtmlElement::code().child(&monospace.0));
    }

    fn visit_sub_script(&mut self, sub_script: &SubScript) {
        self.stack
            .push_close(HtmlElement::sub().child(sub_script.0.to_string()));
    }

    fn visit_sup_script(&mut self, sup_script: &SupScript) {
        self.stack
            .push_close(HtmlElement::sup().child(sup_script.0.to_string()));
    }

    fn visit_spacing(&mut self, spacing: &Spacing) {
        self.stack.add_content(" ".repeat(spacing.0));
    }

    fn visit_word(&mut self, word: &crate::parse::cst::terminal::Word) {
        self.stack.add_content(&word.0);
    }
}
