use miette::Result;
use pandoc_ast::{
    Alignment, Block as PandocBlock, Cell, ColWidth, Inline, ListNumberDelim, ListNumberStyle,
    MathType, MetaValue, Pandoc, QuoteType, Row,
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
            meta.insert("title".to_owned(), MetaValue::MetaString(title.to_string()));
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
            .ident_opt(raw.label.as_ref())
            .class_opt(raw.lang.as_ref())
            .build();

        let block = PandocBlock::CodeBlock(attr, raw.content.to_string());
        self.add_block(block);

        Ok(())
    }

    fn visit_heading(&mut self, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, heading)?;

        let attr = AttrBuilder::new().ident_opt(heading.label.as_ref()).build();

        let block = PandocBlock::Header(heading.level as i64, attr, self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_list(&mut self, list: &List) -> Result<(), Self::Error> {
        let mut bullet_list = Vec::new();

        for item in &list.items {
            let mut bullet_point = Vec::new();
            for block in &item.content {
                self.visit_block(block)?;
                let block = self.pop_block();
                bullet_point.push(block);
            }
            bullet_list.push(bullet_point);
        }

        let block = PandocBlock::BulletList(bullet_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_enum(&mut self, enumeration: &Enum) -> Result<(), Self::Error> {
        let mut ordered_list = Vec::new();

        for item in &enumeration.items {
            let mut ordered_point = Vec::new();
            for block in &item.content {
                self.visit_block(block)?;
                let block = self.pop_block();
                ordered_point.push(block);
            }
            ordered_list.push(ordered_point);
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

    fn visit_term(&mut self, term: &Term) -> Result<(), Self::Error> {
        let mut definition_list = Vec::new();

        for item in &term.content {
            let start = self.start();
            self.visit_text(&item.term)?;
            let definition = self.end(start).collect();

            let start = self.start();
            self.visit_text(&item.content)?;
            let body = vec![vec![PandocBlock::Plain(self.end(start).collect())]];

            definition_list.push((definition, body));
        }

        let block = PandocBlock::DefinitionList(definition_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<(), Self::Error> {
        walk_paragraph(self, paragraph)?;

        let block = PandocBlock::Para(self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_plain(&mut self, plain: &Plain) -> std::prelude::v1::Result<(), Self::Error> {
        walk_plain(self, plain)?;

        let block = PandocBlock::Plain(self.take_stack());
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

        let href = link.href.to_string();
        let mut content = self.end(pos).collect::<Vec<_>>();

        if content.is_empty() {
            content.push(Inline::Str(href.clone()));
        }

        let inline = Inline::Link(AttrBuilder::empty(), content, (href, String::new()));
        self.stack.push(inline);

        Ok(())
    }

    fn visit_cite(&mut self, cite: &Cite) -> Result<(), Self::Error> {
        let href = format!("#{}", cite.ident);
        let content = vec![Inline::Str(cite.ident.to_string())];

        let inline = Inline::Link(AttrBuilder::empty(), content, (href, String::new()));
        self.stack.push(inline);
        Ok(())
    }

    fn visit_raw_inline(&mut self, raw_inline: &RawInline) -> Result<(), Self::Error> {
        let inline = Inline::Code(AttrBuilder::empty(), raw_inline.content.to_string());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_math_inline(&mut self, math_inline: &MathInline) -> Result<(), Self::Error> {
        let inline = Inline::Math(MathType::InlineMath, math_inline.content.to_string());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_comment(&mut self, _comment: &Comment) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_escape(&mut self, escape: &Escape) -> Result<(), Self::Error> {
        let inline = Inline::Str(escape.content.to_string());
        self.stack.push(inline);

        Ok(())
    }

    fn visit_word(&mut self, word: &Word) -> Result<(), Self::Error> {
        self.stack.push(Inline::Str(word.content.to_string()));

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
            Value::Str(s) => self.stack.push(Inline::Str(s.to_string())),
            Value::List(l) => self.stack.push(Inline::Str(format!("{l:?}"))),
            Value::Map(m) => self.stack.push(Inline::Str(format!("{m:?}"))),
        }

        Ok(())
    }
}
