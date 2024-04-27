use std::sync::Arc;

use miette::Result;
use pandoc_ast as ir;
use tyd_eval::{
    error::{EngineError, EngineErrors, EngineMessage},
    eval::{Context, Engine, Eval},
    value::{Cast, Value},
    world::World,
};
use tyd_syntax::prelude::*;

use crate::{attr::AttrBuilder, error::PandocError};

impl Cast<PandocEngine> for ir::Inline {
    fn cast(value: Value<PandocEngine>) -> Self {
        value.into_inline().unwrap()
    }
}

impl Cast<PandocEngine> for ir::Block {
    fn cast(value: Value<PandocEngine>) -> Self {
        value.into_block().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct PandocEngine {
    pandoc: ir::Pandoc,
    stack: Vec<ir::Inline>,
    world: World<Self>,
}

impl PandocEngine {
    pub fn new(world: World<Self>) -> Self {
        Self {
            pandoc: ir::Pandoc {
                pandoc_api_version: vec![1, 23, 1],
                meta: ir::Map::new(),
                blocks: Vec::new(),
            },
            stack: Vec::new(),
            world,
        }
    }

    pub fn build(mut self, ast: &Ast) -> Result<ir::Pandoc, PandocError> {
        let mut context = Context::new(self.world.clone());
        self.visit_ast(&mut context, ast)?;

        if context.scope.has_errors() {
            return Err(EngineErrors {
                src: context.world.source().named_source(),
                related: context.scope.into_errors(),
            })?;
        }

        if let Some(Value::Str(title)) = context.scope.symbol("title") {
            self.pandoc.meta.insert(
                "title".to_owned(),
                ir::MetaValue::MetaString(title.to_string()),
            );
        }

        Ok(self.pandoc)
    }

    pub(crate) fn start(&self) -> usize {
        self.stack.len()
    }

    pub(crate) fn end(&mut self, start: usize) -> impl Iterator<Item = ir::Inline> + '_ {
        self.stack.drain(start..)
    }

    pub(crate) fn take_stack(&mut self) -> Vec<ir::Inline> {
        std::mem::replace(&mut self.stack, Vec::new())
    }

    pub(crate) fn stack_is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub(crate) fn push(&mut self, inline: ir::Inline) {
        self.stack.push(inline);
    }

    pub(crate) fn add_block(&mut self, block: ir::Block) {
        self.pandoc.blocks.push(block)
    }

    pub(crate) fn pop_block(&mut self) -> ir::Block {
        self.pandoc.blocks.pop().unwrap()
    }
}

impl Engine for PandocEngine {
    type Inline = ir::Inline;
    type Block = ir::Block;

    fn process_inline(&mut self, inline: &Inline) -> Option<ir::Inline> {
        let start = self.start();
        let mut context = Context::new(self.world.clone());
        self.visit_inline(&mut context, inline).ok()?;

        // TODO error handling

        let content = self.end(start).last().unwrap();

        Some(content)
    }

    fn process_block(&mut self, block: &Block) -> Option<ir::Block> {
        let mut context = Context::new(self.world.clone());
        self.visit_block(&mut context, block).ok()?;
        let block = self.pop_block();
        Some(block)
    }
}

impl Visitor for PandocEngine {
    type Error = PandocError;
    type State = Context<Self>;

    fn visit_raw(&mut self, _state: &mut Self::State, raw: &Raw) -> Result<(), Self::Error> {
        let attr = AttrBuilder::new().class_opt(raw.lang.as_ref()).build();

        let block = ir::Block::CodeBlock(attr, raw.content.to_string());
        self.add_block(block);

        Ok(())
    }

    fn visit_heading(
        &mut self,
        state: &mut Self::State,
        heading: &Heading,
    ) -> Result<(), Self::Error> {
        walk_heading(self, state, heading)?;

        let attr = AttrBuilder::new().ident_opt(heading.label.as_ref()).build();

        let block = ir::Block::Header(heading.level.level as i64, attr, self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_list(&mut self, state: &mut Self::State, list: &List) -> Result<(), Self::Error> {
        let mut bullet_list = Vec::new();

        for item in &list.items {
            let mut bullet_point = Vec::new();
            for block in &item.content {
                self.visit_block(state, block)?;
                let block = self.pop_block();
                bullet_point.push(block);
            }
            bullet_list.push(bullet_point);
        }

        let block = ir::Block::BulletList(bullet_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_enum(
        &mut self,
        state: &mut Self::State,
        enumeration: &Enum,
    ) -> Result<(), Self::Error> {
        let mut ordered_list = Vec::new();

        for item in &enumeration.items {
            let mut ordered_point = Vec::new();
            for block in &item.content {
                self.visit_block(state, block)?;
                let block = self.pop_block();
                ordered_point.push(block);
            }
            ordered_list.push(ordered_point);
        }
        let attrs = (1, ir::ListNumberStyle::Decimal, ir::ListNumberDelim::Period);
        let block = ir::Block::OrderedList(attrs, ordered_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_table(&mut self, state: &mut Self::State, table: &Table) -> Result<(), Self::Error> {
        let col_count = table.col_count;

        let mut rows: Vec<ir::Row> = Vec::new();

        for tr in &table.rows {
            let mut cells: Vec<ir::Cell> = Vec::new();

            for td in &tr.cells {
                self.visit_block(state, td)?;

                let blocks = vec![self.pop_block()];

                let cell = (
                    AttrBuilder::empty(),
                    ir::Alignment::AlignCenter,
                    1,
                    1,
                    blocks,
                );
                cells.push(cell);
            }

            rows.push((AttrBuilder::empty(), cells));
        }

        let attr = AttrBuilder::new().ident_opt(table.label.as_ref()).build();
        let caption = (None, Vec::new());
        let col_spec = (ir::Alignment::AlignCenter, ir::ColWidth::ColWidthDefault);
        let col_specs = vec![col_spec; col_count];
        let head = (AttrBuilder::empty(), Vec::new());
        let body = vec![(AttrBuilder::empty(), col_count as i64, rows, Vec::new())];
        let foot = (AttrBuilder::empty(), Vec::new());

        let block = ir::Block::Table(attr, caption, col_specs, head, body, foot);
        self.add_block(block);
        Ok(())
    }

    fn visit_term(&mut self, state: &mut Self::State, term: &Terms) -> Result<(), Self::Error> {
        let mut definition_list = Vec::new();

        for item in &term.content {
            let start = self.start();
            self.visit_text(state, &item.term)?;
            let definition = self.end(start).collect();

            let start = self.start();
            self.visit_text(state, &item.content)?;
            let body = vec![vec![ir::Block::Plain(self.end(start).collect())]];

            definition_list.push((definition, body));
        }

        let block = ir::Block::DefinitionList(definition_list);
        self.add_block(block);

        Ok(())
    }

    fn visit_paragraph(
        &mut self,
        state: &mut Self::State,
        paragraph: &Paragraph,
    ) -> Result<(), Self::Error> {
        walk_paragraph(self, state, paragraph)?;

        let block = ir::Block::Para(self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_plain(
        &mut self,
        state: &mut Self::State,
        plain: &Plain,
    ) -> std::prelude::v1::Result<(), Self::Error> {
        walk_plain(self, state, plain)?;

        let block = ir::Block::Plain(self.take_stack());
        self.add_block(block);

        Ok(())
    }

    fn visit_quote(&mut self, state: &mut Self::State, quote: &Quote) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_quote(self, state, quote)?;

        let inline = ir::Inline::Quoted(ir::QuoteType::DoubleQuote, self.end(pos).collect());
        self.push(inline);

        Ok(())
    }

    fn visit_strikeout(
        &mut self,
        state: &mut Self::State,
        strikeout: &Strikeout,
    ) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_strikeout(self, state, strikeout)?;

        let inline = ir::Inline::Strikeout(self.end(pos).collect());
        self.push(inline);

        Ok(())
    }

    fn visit_emphasis(
        &mut self,
        state: &mut Self::State,
        emphasis: &Emphasis,
    ) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_emphasis(self, state, emphasis)?;

        let inline = ir::Inline::Emph(self.end(pos).collect());
        self.push(inline);

        Ok(())
    }

    fn visit_strong(
        &mut self,
        state: &mut Self::State,
        strong: &Strong,
    ) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_strong(self, state, strong)?;

        let inline = ir::Inline::Strong(self.end(pos).collect());
        self.push(inline);

        Ok(())
    }

    fn visit_subscript(
        &mut self,
        state: &mut Self::State,
        subscript: &Subscript,
    ) -> Result<(), Self::Error> {
        let start = self.start();

        walk_subscript(self, state, subscript)?;

        let content = self.end(start).collect();
        let inline = ir::Inline::Subscript(content);
        self.push(inline);

        Ok(())
    }

    fn visit_supscript(
        &mut self,
        state: &mut Self::State,
        supscript: &Supscript,
    ) -> Result<(), Self::Error> {
        let start = self.start();

        walk_supscript(self, state, supscript)?;

        let content = self.end(start).collect();
        let inline = ir::Inline::Superscript(content);
        self.push(inline);

        Ok(())
    }

    fn visit_link(&mut self, state: &mut Self::State, link: &Link) -> Result<(), Self::Error> {
        let pos = self.start();

        walk_link(self, state, link)?;

        let href = link.href.to_string();
        let mut content = self.end(pos).collect::<Vec<_>>();

        if content.is_empty() {
            content.push(ir::Inline::Str(href.clone()));
        }

        let inline = ir::Inline::Link(AttrBuilder::empty(), content, (href, String::new()));
        self.push(inline);

        Ok(())
    }

    fn visit_cite(&mut self, state: &mut Self::State, cite: &Cite) -> Result<(), Self::Error> {
        let href = format!("#{}", cite.ident);
        let content = vec![ir::Inline::Str(cite.ident.to_string())];

        let inline = ir::Inline::Link(AttrBuilder::empty(), content, (href, String::new()));
        self.push(inline);
        Ok(())
    }

    fn visit_raw_inline(
        &mut self,
        state: &mut Self::State,
        raw_inline: &RawInline,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Code(AttrBuilder::empty(), raw_inline.content.to_string());
        self.push(inline);

        Ok(())
    }

    fn visit_math_inline(
        &mut self,
        state: &mut Self::State,
        math_inline: &MathInline,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Math(ir::MathType::InlineMath, math_inline.content.to_string());
        self.push(inline);

        Ok(())
    }

    fn visit_escape(
        &mut self,
        state: &mut Self::State,
        escape: &Escape,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Str(escape.content.to_string());
        self.push(inline);

        Ok(())
    }

    fn visit_word(&mut self, state: &mut Self::State, word: &Word) -> Result<(), Self::Error> {
        self.push(ir::Inline::Str(word.content.to_string()));

        Ok(())
    }

    fn visit_spacing(
        &mut self,
        state: &mut Self::State,
        _spacing: &Spacing,
    ) -> Result<(), Self::Error> {
        self.push(ir::Inline::Space);

        Ok(())
    }

    fn visit_softbreak(
        &mut self,
        state: &mut Self::State,
        _soft_break: &SoftBreak,
    ) -> Result<(), Self::Error> {
        self.push(ir::Inline::SoftBreak);

        Ok(())
    }

    fn visit_expr(&mut self, state: &mut Self::State, expr: &Expr) -> Result<(), Self::Error> {
        let mut machine = state.forge(self);

        if let Some(value) = expr.eval(&mut machine) {
            drop(machine);

            match value {
                Value::Block(block) => {
                    if self.stack_is_empty() {
                        self.add_block(block);
                    } else {
                        state.scope.error(EngineError::new(
                            *expr.span(),
                            EngineMessage::ExpectedInline,
                        ));
                    }
                }
                Value::Func(_) => todo!(),
                Value::Arg(_) => todo!(),
                Value::Inline(inline) => self.push(inline),
                Value::Bool(b) => self.push(ir::Inline::Str(format!("{b}"))),
                Value::Float(f) => self.push(ir::Inline::Str(format!("{f}"))),
                Value::Int(i) => self.push(ir::Inline::Str(format!("{i}"))),
                Value::Str(s) => self.push(ir::Inline::Str(s.to_string())),
                Value::List(l) => self.push(ir::Inline::Str(format!("{l:?}"))),
                Value::Map(m) => self.push(ir::Inline::Str(format!("{m:?}"))),
                Value::None => (),
            }
        }

        Ok(())
    }
}
