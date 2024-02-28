use miette::Result;
use pandoc_ast::{
    Alignment, Block as PandocBlock, Cell, ColWidth, Inline, ListNumberDelim, ListNumberStyle,
    MetaValue, Pandoc, QuoteType, Row,
};
use std::collections::BTreeMap;

use tyd_render::Value;
use tyd_syntax::prelude::*;

use crate::{attr::AttrBuilder, error::PandocError, Context};

pub struct PandocBuilder {
    pandoc: Pandoc,
    stack: Vec<Inline>,
    ctx: Context,
}

impl PandocBuilder {
    pub fn new(ctx: Context) -> Self {
        // TODO better mapping of symbol table to pandoc meta

        let mut meta = BTreeMap::new();

        if let Ok(Value::Str(title)) = ctx.eval_symbol("title") {
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

    pub fn pop_block(&mut self) -> PandocBlock {
        self.pandoc.blocks.pop().unwrap()
    }
}

impl Visitor for PandocBuilder {
    type Error = PandocError;

    fn visit_raw(&mut self, raw: &Raw) -> Result<(), Self::Error> {
        let attr = AttrBuilder::new()
            .ident_opt(raw.label)
            .class_opt(raw.lang)
            .build();

        let block = PandocBlock::CodeBlock(attr, raw.content.to_owned());
        self.add_block(block);

        Ok(())
    }

    fn visit_div(&mut self, div: &Div) -> Result<(), Self::Error> {
        let attr = AttrBuilder::new()
            .ident_opt(div.label)
            .class_opt(div.class)
            .build();

        let block = PandocBlock::Div(attr, vec![PandocBlock::Plain(self.take_stack())]);
        self.add_block(block);

        Ok(())
    }

    fn visit_heading(&mut self, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, heading)?;

        let attr = AttrBuilder::new().ident_opt(heading.label).build();

        let block = PandocBlock::Header(heading.level as i64, attr, self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_list(&mut self, list: &List) -> Result<(), Self::Error> {
        let mut bullet_list = Vec::new();

        for item in &list.head {
            let start = self.start();

            self.visit_list_item(item)?;

            let plain = PandocBlock::Plain(self.end(start).collect());
            bullet_list.push(vec![plain]);
        }

        let tail = bullet_list.last_mut().unwrap();

        if let Some(nested) = &list.body {
            self.visit_nested(nested)?;

            let block = self.pop_block();
            // bullet_list.push(vec![block]);
            tail.push(block);
        }

        let block = PandocBlock::BulletList(bullet_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_enum(&mut self, enumeration: &Enum) -> Result<(), Self::Error> {
        let mut ordered_list = Vec::new();

        for item in &enumeration.head {
            let start = self.start();

            self.visit_enum_item(item)?;

            let plain = PandocBlock::Plain(self.end(start).collect());
            ordered_list.push(vec![plain]);
        }

        if let Some(nested) = &enumeration.body {
            self.visit_nested(nested)?;

            let block = self.pop_block();
            ordered_list.push(vec![block]);
        }

        let attrs = (1, ListNumberStyle::Decimal, ListNumberDelim::Period);
        let block = PandocBlock::OrderedList(attrs, ordered_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_table(&mut self, table: &Table) -> Result<(), Self::Error> {
        let col_count = table.col_count;

        let mut rows: Vec<Row> = Vec::new();

        for tr in &table.rows {
            let mut cells: Vec<Cell> = Vec::new();

            for td in &tr.cells {
                self.visit_block(td)?;

                let blocks = vec![self.pop_block()];

                let cell = (AttrBuilder::empty(), Alignment::AlignCenter, 1, 1, blocks);
                cells.push(cell);
            }

            rows.push((AttrBuilder::empty(), cells));
        }

        let attr = AttrBuilder::empty();
        let caption = (None, Vec::new());
        let col_spec = (Alignment::AlignCenter, ColWidth::ColWidthDefault);
        let col_specs = vec![col_spec; col_count];
        let head = (AttrBuilder::empty(), Vec::new());
        let body = vec![(AttrBuilder::empty(), col_count as i64, rows, Vec::new())];
        let foot = (AttrBuilder::empty(), Vec::new());

        let block = PandocBlock::Table(attr, caption, col_specs, head, body, foot);
        self.add_block(block);
        Ok(())
    }

    fn visit_block_quote(&mut self, block_quote: &BlockQuote) -> Result<(), Self::Error> {
        let mut quote = Vec::new();

        for el in &block_quote.content {
            self.visit_block_quote_item(el)?;

            quote.push(self.pop_block());
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

    // fn visit_image(&mut self, image: &Image) -> Result<(), Self::Error> {
    //     let inlines = if let Some(alt) = &image.alt {
    //         vec![Inline::Str(alt.to_owned())]
    //     } else {
    //         Vec::new()
    //     };

    //     let image = Inline::Image(
    //         PandocAttrBuilder::empty(),
    //         inlines,
    //         (image.src.to_owned(), String::new()),
    //     );
    //     let block = PandocBlock::Plain(vec![image]);
    //     self.add_block(block);

    //     Ok(())
    // }

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

    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_emphasis(self, emphasis)?;

        let inline = Inline::Emph(self.end(pos).collect());
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

    fn visit_subscript(&mut self, subscript: &Subscript) -> Result<(), Self::Error> {
        let start = self.start();

        walk_subscript(self, subscript)?;

        let content = self.end(start).collect();
        let inline = Inline::Subscript(content);
        self.stack.push(inline);

        Ok(())
    }

    fn visit_supscript(&mut self, supscript: &Supscript) -> Result<(), Self::Error> {
        let start = self.start();

        walk_supscript(self, supscript)?;

        let content = self.end(start).collect();
        let inline = Inline::Superscript(content);
        self.stack.push(inline);

        Ok(())
    }

    fn visit_link(&mut self, link: &Link) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_link(self, link)?;

        let href = link.href.to_owned();
        let inline = Inline::Link(
            AttrBuilder::empty(),
            self.end(pos).collect(),
            (href.clone(), href),
        );
        self.stack.push(inline);

        Ok(())
    }

    fn visit_cite(&mut self, cite: &Cite) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_raw_inline(&mut self, raw_inline: &RawInline) -> Result<(), Self::Error> {
        let inline = Inline::Code(AttrBuilder::empty(), raw_inline.content.to_owned());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_comment(&mut self, _comment: &Comment) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_escape(&mut self, escape: &Escape) -> Result<(), Self::Error> {
        let inline = Inline::Str(escape.content.to_owned());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_word(&mut self, word: &Word) -> Result<(), Self::Error> {
        self.stack.push(Inline::Str(word.content.to_owned()));

        Ok(())
    }

    fn visit_spacing(&mut self, _spacing: &Spacing) -> Result<(), Self::Error> {
        self.stack.push(Inline::Space);

        Ok(())
    }

    fn visit_softbreak(&mut self) -> Result<(), Self::Error> {
        self.stack.push(Inline::SoftBreak);

        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<(), Self::Error> {
        let value = self.ctx.eval(expr)?;

        match value {
            Value::Content(mut content) => self.stack.append(&mut content),
            Value::Bool(b) => self.stack.push(Inline::Str(format!("{b}"))),
            Value::Float(f) => self.stack.push(Inline::Str(format!("{f}"))),
            Value::Int(i) => self.stack.push(Inline::Str(format!("{i}"))),
            Value::Str(s) => self.stack.push(Inline::Str(s)),
            Value::List(l) => self.stack.push(Inline::Str(format!("{l:?}"))),
            Value::Map(m) => self.stack.push(Inline::Str(format!("{m:?}"))),
        }

        Ok(())
    }
}
