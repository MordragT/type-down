use miette::{Diagnostic, Result};
use pandoc_ast::{
    Alignment, Attr as PandocAttr, Block as PandocBlock, Cell, ColWidth, Inline, ListNumberDelim,
    ListNumberStyle, MetaValue, Pandoc, QuoteType, Row,
};
use std::{collections::BTreeMap, fs, io};
use thiserror::Error;

use tyd_render::{Args, Context, ContextError, Object, Output, Render};
use tyd_syntax::ast::{
    visitor::{
        walk_emphasis, walk_enclosed, walk_heading, walk_link, walk_paragraph, walk_quote,
        walk_strikeout, walk_strong, Visitor,
    },
    *,
};

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(code(type_down::compile::pandoc::PandocCompiler::compile))]
#[error(transparent)]
pub enum PandocError {
    Io(#[from] io::Error),
    Call(#[from] ContextError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    type Error = PandocError;

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

pub struct PandocBuilder {
    pandoc: Pandoc,
    stack: Vec<Inline>,
    ctx: Context,
}

impl PandocBuilder {
    pub fn new(ctx: Context) -> Self {
        // TODO better mapping of symbol table to pandoc meta

        let mut meta = BTreeMap::new();

        if let Ok(Object::Str(title)) = ctx.get("title") {
            meta.insert("title".to_owned(), MetaValue::MetaString(title.clone()));
        }

        Self {
            pandoc: Pandoc {
                pandoc_api_version: vec![1, 23, 1],
                meta,
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
    type Error = PandocError;

    fn visit_raw(&mut self, raw: &Raw) -> Result<(), Self::Error> {
        let attr = PandocAttrBuilder::new()
            .class_opt(raw.lang.as_ref())
            .build();
        let block = PandocBlock::CodeBlock(attr, raw.content.to_owned());
        self.add_block(block);

        Ok(())
    }

    fn visit_heading(&mut self, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, heading)?;

        let attr = PandocAttrBuilder::empty();
        let block = PandocBlock::Header(heading.level as i64, attr, self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_bullet_list(&mut self, list: &BulletList) -> Result<(), Self::Error> {
        let mut bullet_list = Vec::new();

        for line in &list.lines {
            let pos = self.start();

            self.visit_line(line)?;

            let plain = PandocBlock::Plain(self.end(pos).collect());
            bullet_list.push(vec![plain]);
        }

        let block = PandocBlock::BulletList(bullet_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_ordered_list(&mut self, ordered_list: &OrderedList) -> Result<(), Self::Error> {
        let mut list = Vec::new();

        for line in &ordered_list.lines {
            let pos = self.start();

            self.visit_line(line)?;

            let plain = PandocBlock::Plain(self.end(pos).collect());
            list.push(vec![plain]);
        }

        let attrs = (1, ListNumberStyle::Decimal, ListNumberDelim::Period);
        let block = PandocBlock::OrderedList(attrs, list);
        self.add_block(block);

        Ok(())
    }

    fn visit_table(&mut self, table: &Table) -> Result<(), Self::Error> {
        // let row_count = table.rows.len();
        let col_count = table.rows[0].cells.len();

        let mut rows: Vec<Row> = Vec::new();

        for tr in &table.rows {
            let mut cells: Vec<Cell> = Vec::new();

            for td in &tr.cells {
                let pos = self.start();

                self.visit_elements(td)?;

                let blocks = self
                    .end(pos)
                    .map(|inline| PandocBlock::Plain(vec![inline]))
                    .collect::<Vec<_>>();
                let cell = (
                    PandocAttrBuilder::empty(),
                    Alignment::AlignCenter,
                    1,
                    1,
                    blocks,
                );
                cells.push(cell);
            }

            rows.push((PandocAttrBuilder::empty(), cells));
        }

        let attr = PandocAttrBuilder::empty();
        let caption = (None, Vec::new());
        let col_spec = (Alignment::AlignCenter, ColWidth::ColWidthDefault);
        let col_specs = vec![col_spec; col_count];
        let head = (PandocAttrBuilder::empty(), Vec::new());
        let body = vec![(
            PandocAttrBuilder::empty(),
            col_count as i64,
            rows,
            Vec::new(),
        )];
        let foot = (PandocAttrBuilder::empty(), Vec::new());

        let block = PandocBlock::Table(attr, caption, col_specs, head, body, foot);
        self.add_block(block);
        Ok(())
    }

    fn visit_block_quote(&mut self, block_quote: &BlockQuote) -> Result<(), Self::Error> {
        let mut quote = Vec::new();

        for line in &block_quote.lines {
            let pos = self.start();

            self.visit_line(line)?;

            let plain = PandocBlock::Plain(self.end(pos).collect());
            quote.push(plain);
        }

        let block = PandocBlock::BlockQuote(quote);
        self.add_block(block);

        Ok(())
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<(), Self::Error> {
        walk_paragraph(self, paragraph)?;

        let block = PandocBlock::Para(self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_image(&mut self, image: &Image) -> Result<(), Self::Error> {
        let inlines = if let Some(alt) = &image.alt {
            vec![Inline::Str(alt.to_owned())]
        } else {
            Vec::new()
        };

        let image = Inline::Image(
            PandocAttrBuilder::empty(),
            inlines,
            (image.src.to_owned(), String::new()),
        );
        let block = PandocBlock::Plain(vec![image]);
        self.add_block(block);

        Ok(())
    }

    fn visit_quote(&mut self, quote: &Quote) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_quote(self, quote)?;

        let inline = Inline::Quoted(QuoteType::DoubleQuote, self.end(pos).collect());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_strikeout(&mut self, strikeout: &Strikeout) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_strikeout(self, strikeout)?;

        let inline = Inline::Strikeout(self.end(pos).collect());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_strong(&mut self, strong: &Strong) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_strong(self, strong)?;

        let inline = Inline::Strong(self.end(pos).collect());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_emphasis(self, emphasis)?;

        let inline = Inline::Emph(self.end(pos).collect());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_enclosed(&mut self, enclosed: &Enclosed) -> Result<(), Self::Error> {
        walk_enclosed(self, enclosed)?;

        Ok(())
    }

    fn visit_link(&mut self, link: &Link) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_link(self, link)?;

        let inline = Inline::Link(
            PandocAttrBuilder::empty(),
            self.end(pos).collect(),
            (link.link.to_owned(), link.link.to_owned()),
        );
        self.stack.push(inline);

        Ok(())
    }

    fn visit_escape(&mut self, escape: &Escape) -> Result<(), Self::Error> {
        let inline = Inline::Str(escape.0.to_string());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_raw_inline(&mut self, raw_inline: &RawInline) -> Result<(), Self::Error> {
        let inline = Inline::Code(PandocAttrBuilder::empty(), raw_inline.0.to_owned());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_sub_script(&mut self, sub_script: &SubScript) -> Result<(), Self::Error> {
        let inline = Inline::Subscript(vec![Inline::Str(sub_script.0.to_string())]);
        self.stack.push(inline);

        Ok(())
    }

    fn visit_sup_script(&mut self, sup_script: &SupScript) -> Result<(), Self::Error> {
        let inline = Inline::Superscript(vec![Inline::Str(sup_script.0.to_string())]);
        self.stack.push(inline);

        Ok(())
    }

    fn visit_spacing(&mut self, _spacing: &Spacing) -> Result<(), Self::Error> {
        // let inline = Inline::Str(" ".repeat(spacing.0));
        // self.stack.push(inline);
        self.stack.push(Inline::Space);

        Ok(())
    }

    fn visit_word(&mut self, word: &Word) -> Result<(), Self::Error> {
        let inline = Inline::Str(word.0.to_owned());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_access(&mut self, access: &Access) -> Result<(), Self::Error> {
        let Access { ident, tail } = access;

        if let Some(CallTail { args, content }) = tail {
            // FIXME better argument handling and evaluation layer
            let mut f_args = Args::new();

            for (key, value) in args {
                match value {
                    Value::Identifier(ident) => {
                        let object = self.ctx.get(ident)?;
                        f_args.insert(key.clone(), object.clone());
                    }
                    Value::String(s) => {
                        f_args.insert(key.clone(), Object::Str(s.clone()));
                    }
                }
            }

            if let Some(enclosed) = content {
                f_args.insert(
                    "content".to_owned(),
                    Object::List(
                        enclosed
                            .elements
                            .0
                            .iter()
                            .map(|el| Object::Element(el.clone()))
                            .collect(),
                    ),
                );
            }

            let f = self.ctx.call(ident)?;
            let result = f(f_args)?;

            match result {
                Object::Block(block) => self.visit_block(&block)?,
                Object::Element(el) => self.visit_element(&el)?,
                // TODO error
                _ => panic!("access calls must return block or element"),
            }
        } else {
            let object = self.ctx.get(ident)?.clone();

            match object {
                Object::Block(block) => self.visit_block(&block)?,
                Object::Element(el) => self.visit_element(&el)?,
                // TODO error
                _ => panic!("access calls must return block or element"),
            }
        }
        Ok(())
    }
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
