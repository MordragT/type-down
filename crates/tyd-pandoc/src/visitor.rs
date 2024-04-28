use miette::Result;
use pandoc_ast as ir;
use tyd_eval::{
    error::{EngineError, EngineErrors, EngineMessage},
    eval::{Engine, Eval},
    value::{Cast, Value},
    world::World,
};
use tyd_syntax::prelude::*;

use crate::{attr::AttrBuilder, engine::PandocEngine, error::PandocError};

#[derive(Debug, Clone)]
pub struct PandocVisitor {}

impl Visitor for PandocVisitor {
    type Error = PandocError;
    type State = PandocEngine;

    fn visit_raw(&self, state: &mut Self::State, raw: &Raw) -> Result<(), Self::Error> {
        let attr = AttrBuilder::new().class_opt(raw.lang.as_ref()).build();

        let block = ir::Block::CodeBlock(attr, raw.content.to_string());
        state.add_block(block);

        Ok(())
    }

    fn visit_heading(&self, state: &mut Self::State, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, state, heading)?;

        let attr = AttrBuilder::new().ident_opt(heading.label.as_ref()).build();

        let block = ir::Block::Header(heading.level.level as i64, attr, state.take_stack());
        state.add_block(block);

        Ok(())
    }

    fn visit_list(&self, state: &mut Self::State, list: &List) -> Result<(), Self::Error> {
        let mut bullet_list = Vec::new();

        for item in &list.items {
            let mut bullet_point = Vec::new();
            for block in &item.content {
                self.visit_block(state, block)?;
                let block = state.pop_block();
                bullet_point.push(block);
            }
            bullet_list.push(bullet_point);
        }

        let block = ir::Block::BulletList(bullet_list);
        state.add_block(block);

        Ok(())
    }

    fn visit_enum(&self, state: &mut Self::State, enumeration: &Enum) -> Result<(), Self::Error> {
        let mut ordered_list = Vec::new();

        for item in &enumeration.items {
            let mut ordered_point = Vec::new();
            for block in &item.content {
                self.visit_block(state, block)?;
                let block = state.pop_block();
                ordered_point.push(block);
            }
            ordered_list.push(ordered_point);
        }
        let attrs = (1, ir::ListNumberStyle::Decimal, ir::ListNumberDelim::Period);
        let block = ir::Block::OrderedList(attrs, ordered_list);
        state.add_block(block);

        Ok(())
    }

    fn visit_table(&self, state: &mut Self::State, table: &Table) -> Result<(), Self::Error> {
        let col_count = table.col_count;

        let mut rows: Vec<ir::Row> = Vec::new();

        for tr in &table.rows {
            let mut cells: Vec<ir::Cell> = Vec::new();

            for td in &tr.cells {
                self.visit_block(state, td)?;

                let blocks = vec![state.pop_block()];

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
        state.add_block(block);
        Ok(())
    }

    fn visit_term(&self, state: &mut Self::State, term: &Terms) -> Result<(), Self::Error> {
        let mut definition_list = Vec::new();

        for item in &term.content {
            let start = state.start();
            self.visit_text(state, &item.term)?;
            let definition = state.end(start).collect();

            let start = state.start();
            self.visit_text(state, &item.content)?;
            let body = vec![vec![ir::Block::Plain(state.end(start).collect())]];

            definition_list.push((definition, body));
        }

        let block = ir::Block::DefinitionList(definition_list);
        state.add_block(block);

        Ok(())
    }

    fn visit_paragraph(
        &self,
        state: &mut Self::State,
        paragraph: &Paragraph,
    ) -> Result<(), Self::Error> {
        walk_paragraph(self, state, paragraph)?;

        let block = ir::Block::Para(state.take_stack());
        state.add_block(block);

        Ok(())
    }

    fn visit_plain(
        &self,
        state: &mut Self::State,
        plain: &Plain,
    ) -> std::prelude::v1::Result<(), Self::Error> {
        walk_plain(self, state, plain)?;

        let block = ir::Block::Plain(state.take_stack());
        state.add_block(block);

        Ok(())
    }

    fn visit_quote(&self, state: &mut Self::State, quote: &Quote) -> Result<(), Self::Error> {
        let pos = state.start();

        walk_quote(self, state, quote)?;

        let inline = ir::Inline::Quoted(ir::QuoteType::DoubleQuote, state.end(pos).collect());
        state.push(inline);

        Ok(())
    }

    fn visit_strikeout(
        &self,
        state: &mut Self::State,
        strikeout: &Strikeout,
    ) -> Result<(), Self::Error> {
        let pos = state.start();

        walk_strikeout(self, state, strikeout)?;

        let inline = ir::Inline::Strikeout(state.end(pos).collect());
        state.push(inline);

        Ok(())
    }

    fn visit_emphasis(
        &self,
        state: &mut Self::State,
        emphasis: &Emphasis,
    ) -> Result<(), Self::Error> {
        let pos = state.start();

        walk_emphasis(self, state, emphasis)?;

        let inline = ir::Inline::Emph(state.end(pos).collect());
        state.push(inline);

        Ok(())
    }

    fn visit_strong(&self, state: &mut Self::State, strong: &Strong) -> Result<(), Self::Error> {
        let pos = state.start();

        walk_strong(self, state, strong)?;

        let inline = ir::Inline::Strong(state.end(pos).collect());
        state.push(inline);

        Ok(())
    }

    fn visit_subscript(
        &self,
        state: &mut Self::State,
        subscript: &Subscript,
    ) -> Result<(), Self::Error> {
        let start = state.start();

        walk_subscript(self, state, subscript)?;

        let content = state.end(start).collect();
        let inline = ir::Inline::Subscript(content);
        state.push(inline);

        Ok(())
    }

    fn visit_supscript(
        &self,
        state: &mut Self::State,
        supscript: &Supscript,
    ) -> Result<(), Self::Error> {
        let start = state.start();

        walk_supscript(self, state, supscript)?;

        let content = state.end(start).collect();
        let inline = ir::Inline::Superscript(content);
        state.push(inline);

        Ok(())
    }

    fn visit_link(&self, state: &mut Self::State, link: &Link) -> Result<(), Self::Error> {
        let pos = state.start();

        walk_link(self, state, link)?;

        let href = link.href.to_string();
        let mut content = state.end(pos).collect::<Vec<_>>();

        if content.is_empty() {
            content.push(ir::Inline::Str(href.clone()));
        }

        let inline = ir::Inline::Link(AttrBuilder::empty(), content, (href, String::new()));
        state.push(inline);

        Ok(())
    }

    fn visit_cite(&self, state: &mut Self::State, cite: &Cite) -> Result<(), Self::Error> {
        let href = format!("#{}", cite.ident);
        let content = vec![ir::Inline::Str(cite.ident.to_string())];

        let inline = ir::Inline::Link(AttrBuilder::empty(), content, (href, String::new()));
        state.push(inline);
        Ok(())
    }

    fn visit_raw_inline(
        &self,
        state: &mut Self::State,
        raw_inline: &RawInline,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Code(AttrBuilder::empty(), raw_inline.content.to_string());
        state.push(inline);

        Ok(())
    }

    fn visit_math_inline(
        &self,
        state: &mut Self::State,
        math_inline: &MathInline,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Math(ir::MathType::InlineMath, math_inline.content.to_string());
        state.push(inline);

        Ok(())
    }

    fn visit_escape(&self, state: &mut Self::State, escape: &Escape) -> Result<(), Self::Error> {
        let inline = ir::Inline::Str(escape.content.to_string());
        state.push(inline);

        Ok(())
    }

    fn visit_word(&self, state: &mut Self::State, word: &Word) -> Result<(), Self::Error> {
        state.push(ir::Inline::Str(word.content.to_string()));

        Ok(())
    }

    fn visit_spacing(
        &self,
        state: &mut Self::State,
        _spacing: &Spacing,
    ) -> Result<(), Self::Error> {
        state.push(ir::Inline::Space);

        Ok(())
    }

    fn visit_softbreak(
        &self,
        state: &mut Self::State,
        _soft_break: &SoftBreak,
    ) -> Result<(), Self::Error> {
        state.push(ir::Inline::SoftBreak);

        Ok(())
    }

    fn visit_expr(&self, state: &mut Self::State, expr: &Expr) -> Result<(), Self::Error> {
        if let Some(value) = expr.eval(state, self) {
            match value {
                Value::Block(block) => {
                    if state.stack_is_empty() {
                        state.add_block(block);
                    } else {
                        state.tracer_mut().error(EngineError::new(
                            *expr.span(),
                            EngineMessage::ExpectedInline,
                        ));
                    }
                }
                Value::Func(_) => todo!(),
                Value::Inline(inline) => state.push(inline),
                Value::Bool(b) => state.push(ir::Inline::Str(format!("{b}"))),
                Value::Float(f) => state.push(ir::Inline::Str(format!("{f}"))),
                Value::Int(i) => state.push(ir::Inline::Str(format!("{i}"))),
                Value::Str(s) => state.push(ir::Inline::Str(s.to_string())),
                Value::List(l) => state.push(ir::Inline::Str(format!("{l:?}"))),
                Value::Map(m) => state.push(ir::Inline::Str(format!("{m:?}"))),
                Value::None => (),
            }
        }

        Ok(())
    }
}
