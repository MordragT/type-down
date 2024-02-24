use miette::Diagnostic;
use pandoc_ast::{
    Attr as PandocAttr, Block as PandocBlock, Inline, ListNumberDelim, ListNumberStyle, Pandoc,
    QuoteType,
};
use std::{collections::BTreeMap, fs, io};
use thiserror::Error;

use tyd_render::{Context, Output, Render};
use tyd_syntax::ast::{
    visitor::{
        walk_emphasis, walk_enclosed, walk_heading, walk_link, walk_paragraph, walk_quote,
        walk_strikeout, walk_strong, Visitor,
    },
    *,
};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
pub struct PandocError(#[from] pub io::Error);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    type Error = PandocError;

    fn render(ast: &Ast, ctx: Context, output: Output) -> Result<(), Self::Error> {
        let mut builder = PandocBuilder::new(ctx);
        builder.visit_ast(ast);

        let pandoc = builder.build();
        let contents = pandoc.to_json();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => fs::write(path, contents)?,
        }

        Ok(())
    }
}

pub struct PandocBuilder {
    pandoc: Pandoc,
    stack: Vec<Inline>,
    ctx: Context,
}

impl PandocBuilder {
    pub fn new(ctx: Context) -> Self {
        Self {
            pandoc: Pandoc {
                pandoc_api_version: vec![1, 23, 1],
                meta: BTreeMap::new(),
                blocks: Vec::new(),
            },
            stack: Vec::new(),
            ctx,
        }
    }

    pub fn build(self) -> Pandoc {
        self.pandoc
    }

    pub fn start(&self) -> usize {
        self.stack.len()
    }

    pub fn end(&mut self, start: usize) -> impl Iterator<Item = Inline> + '_ {
        self.stack.drain(start..)
    }

    pub fn take_stack(&mut self) -> Vec<Inline> {
        std::mem::replace(&mut self.stack, Vec::new())
    }

    pub fn add_block(&mut self, block: PandocBlock) {
        self.pandoc.blocks.push(block)
    }
}

impl Visitor for PandocBuilder {
    fn visit_raw(&mut self, raw: &Raw) {
        let attr = PandocAttrBuilder::new()
            .class_opt(raw.lang.as_ref())
            .build();
        let block = PandocBlock::CodeBlock(attr, raw.content.to_owned());
        self.add_block(block);
    }

    fn visit_heading(&mut self, heading: &Heading) {
        walk_heading(self, heading);

        let attr = PandocAttrBuilder::empty();
        let block = PandocBlock::Header(heading.level as i64, attr, self.take_stack());
        self.add_block(block);
    }

    fn visit_bullet_list(&mut self, list: &BulletList) {
        let mut bullet_list = Vec::new();

        for line in &list.lines {
            let pos = self.start();

            self.visit_line(line);

            let plain = PandocBlock::Plain(self.end(pos).collect());
            bullet_list.push(vec![plain]);
        }

        let block = PandocBlock::BulletList(bullet_list);
        self.add_block(block);
    }

    fn visit_ordered_list(&mut self, ordered_list: &OrderedList) {
        let mut list = Vec::new();

        for line in &ordered_list.lines {
            let pos = self.start();

            self.visit_line(line);

            let plain = PandocBlock::Plain(self.end(pos).collect());
            list.push(vec![plain]);
        }

        let attrs = (1, ListNumberStyle::Decimal, ListNumberDelim::Period);
        let block = PandocBlock::OrderedList(attrs, list);
        self.add_block(block);
    }

    fn visit_table(&mut self, table: &Table) {}

    fn visit_block_quote(&mut self, block_quote: &BlockQuote) {
        let mut quote = Vec::new();

        for line in &block_quote.lines {
            let pos = self.start();

            self.visit_line(line);

            let plain = PandocBlock::Plain(self.end(pos).collect());
            quote.push(plain);
        }

        let block = PandocBlock::BlockQuote(quote);
        self.add_block(block);
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) {
        walk_paragraph(self, paragraph);

        let block = PandocBlock::Para(self.take_stack());
        self.add_block(block);
    }

    fn visit_quote(&mut self, quote: &Quote) {
        let pos = self.start();

        walk_quote(self, quote);

        let inline = Inline::Quoted(QuoteType::DoubleQuote, self.end(pos).collect());
        self.stack.push(inline);
    }

    fn visit_strikeout(&mut self, strikeout: &Strikeout) {
        let pos = self.start();

        walk_strikeout(self, strikeout);

        let inline = Inline::Strikeout(self.end(pos).collect());
        self.stack.push(inline);
    }

    fn visit_strong(&mut self, strong: &Strong) {
        let pos = self.start();

        walk_strong(self, strong);

        let inline = Inline::Strong(self.end(pos).collect());
        self.stack.push(inline);
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) {
        let pos = self.start();

        walk_emphasis(self, emphasis);

        let inline = Inline::Emph(self.end(pos).collect());
        self.stack.push(inline);
    }

    fn visit_enclosed(&mut self, enclosed: &Enclosed) {
        walk_enclosed(self, enclosed);
    }

    fn visit_link(&mut self, link: &Link) {
        let pos = self.start();

        walk_link(self, link);

        let inline = Inline::Link(
            PandocAttrBuilder::empty(),
            self.end(pos).collect(),
            (link.link.to_owned(), link.link.to_owned()),
        );
        self.stack.push(inline);
    }

    fn visit_escape(&mut self, escape: &Escape) {
        let inline = Inline::Str(escape.0.to_string());
        self.stack.push(inline);
    }

    fn visit_raw_inline(&mut self, raw_inline: &RawInline) {
        let inline = Inline::Code(PandocAttrBuilder::empty(), raw_inline.0.to_owned());
        self.stack.push(inline);
    }

    fn visit_sub_script(&mut self, sub_script: &SubScript) {
        let inline = Inline::Subscript(vec![Inline::Str(sub_script.0.to_string())]);
        self.stack.push(inline);
    }

    fn visit_sup_script(&mut self, sup_script: &SupScript) {
        let inline = Inline::Superscript(vec![Inline::Str(sup_script.0.to_string())]);
        self.stack.push(inline);
    }

    fn visit_spacing(&mut self, _spacing: &Spacing) {
        // let inline = Inline::Str(" ".repeat(spacing.0));
        // self.stack.push(inline);
        self.stack.push(Inline::Space);
    }

    fn visit_word(&mut self, word: &Word) {
        let inline = Inline::Str(word.0.to_owned());
        self.stack.push(inline);
    }

    fn visit_access(&mut self, access: &Access) {}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocAttrBuilder {
    ident: String,
    classes: Vec<String>,
    pairs: Vec<(String, String)>,
}

impl PandocAttrBuilder {
    pub fn new() -> Self {
        Self {
            ident: String::new(),
            classes: Vec::new(),
            pairs: Vec::new(),
        }
    }

    pub fn ident(mut self, ident: impl Into<String>) -> Self {
        self.ident = ident.into();
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    pub fn class_opt(mut self, class: Option<impl Into<String>>) -> Self {
        if let Some(class) = class {
            self.classes.push(class.into())
        }
        self
    }

    pub fn attr<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
        self
    }

    pub fn build(self) -> PandocAttr {
        let Self {
            ident,
            classes,
            pairs,
        } = self;
        (ident, classes, pairs)
    }

    pub fn empty() -> PandocAttr {
        (String::new(), Vec::new(), Vec::new())
    }
}
