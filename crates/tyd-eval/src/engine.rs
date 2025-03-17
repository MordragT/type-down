use ecow::EcoString;
use std::{collections::BTreeMap, fmt::Debug, mem};
use tyd_doc::prelude::*;
use tyd_syntax::{source::Source, SpanMetadata};

use crate::{
    error::{ArgumentError, EngineErrors, EngineMessage},
    ir,
    scope::Scope,
    tracer::Tracer,
    ty::Type,
    value::Value,
};

/// The core component, responsible for typesetting
#[derive(Debug, Clone)]
pub struct Engine {
    pub inlines: Vec<ir::Inline>,
    pub blocks: Vec<ir::Block>,
    pub rows: Vec<ir::Row>,
    pub definitions: Vec<ir::Definition>,
    pub bullet_list: Vec<Vec<ir::Block>>,
    pub ordered_list: Vec<Vec<ir::Block>>,
    pub stack: Vec<Value>,
    pub scope: Scope,
    pub tracer: Tracer,
    pub source: Source,
}

impl Engine {
    pub fn new(global_scope: Scope, spans: SpanMetadata, source: Source) -> Self {
        Self {
            inlines: Vec::new(),
            blocks: Vec::new(),
            rows: Vec::new(),
            definitions: Vec::new(),
            bullet_list: Vec::new(),
            ordered_list: Vec::new(),
            stack: Vec::new(),
            scope: Scope::new(global_scope),
            tracer: Tracer::new(spans),
            source,
        }
    }

    pub fn run(mut self, doc: Doc) -> Result<ir::Pandoc, EngineErrors> {
        doc.visit_by(&mut self)?;

        let Self {
            blocks,
            inlines,
            rows,
            definitions,
            bullet_list,
            ordered_list,
            stack,
            scope,
            tracer,
            source,
        } = self;

        assert!(inlines.is_empty());
        assert!(rows.is_empty());
        assert!(definitions.is_empty());
        assert!(bullet_list.is_empty());
        assert!(ordered_list.is_empty());
        assert!(stack.is_empty());

        if tracer.has_errors() {
            return Err(EngineErrors {
                src: source,
                related: tracer.into_errors(),
            })?;
        }

        let mut meta = BTreeMap::new();

        if let Some(title) = scope.lookup_str(&EcoString::from("title")) {
            meta.insert(
                "title".to_owned(),
                ir::MetaValue::MetaString(title.to_string()),
            );
        }

        Ok(ir::Pandoc {
            pandoc_api_version: vec![1, 23, 1],
            meta,
            blocks,
        })
    }

    pub fn take_blocks(&mut self) -> Vec<ir::Block> {
        mem::take(&mut self.blocks)
    }

    pub fn take_inlines(&mut self) -> Vec<ir::Inline> {
        mem::take(&mut self.inlines)
    }

    pub fn take_rows(&mut self) -> Vec<ir::Row> {
        mem::take(&mut self.rows)
    }

    pub fn take_definitions(&mut self) -> Vec<ir::Definition> {
        mem::take(&mut self.definitions)
    }

    pub fn take_bullet_list(&mut self) -> Vec<Vec<ir::Block>> {
        mem::take(&mut self.bullet_list)
    }

    pub fn take_ordered_list(&mut self) -> Vec<Vec<ir::Block>> {
        mem::take(&mut self.ordered_list)
    }

    pub fn take_stack(&mut self) -> Vec<Value> {
        mem::take(&mut self.stack)
    }

    pub fn replace_blocks(&mut self, src: Vec<ir::Block>) -> Vec<ir::Block> {
        mem::replace(&mut self.blocks, src)
    }

    pub fn replace_inlines(&mut self, src: Vec<ir::Inline>) -> Vec<ir::Inline> {
        mem::replace(&mut self.inlines, src)
    }

    pub fn replace_rows(&mut self, src: Vec<ir::Row>) -> Vec<ir::Row> {
        mem::replace(&mut self.rows, src)
    }

    pub fn replace_definitions(&mut self, src: Vec<ir::Definition>) -> Vec<ir::Definition> {
        mem::replace(&mut self.definitions, src)
    }

    pub fn replace_bullet_list(&mut self, src: Vec<Vec<ir::Block>>) -> Vec<Vec<ir::Block>> {
        mem::replace(&mut self.bullet_list, src)
    }

    pub fn replace_ordered_list(&mut self, src: Vec<Vec<ir::Block>>) -> Vec<Vec<ir::Block>> {
        mem::replace(&mut self.ordered_list, src)
    }

    pub fn replace_stack(&mut self, src: Vec<Value>) -> Vec<Value> {
        mem::replace(&mut self.stack, src)
    }
}

impl Visitor for Engine {
    type Error = EngineErrors;

    fn visit_error(
        &mut self,
        (error, id): Full<tree::Error>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.tracer
            .node_error(id, EngineMessage::Doc(error.clone()));
        Ok(())
    }

    fn visit_raw(&mut self, raw: Full<tree::Raw>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Raw { lang, text } = raw.0;

        let lang = lang.map(|id| doc.node(id).0.to_string());
        let attr = ir::AttrBuilder::new().class_opt(lang).build();

        let block = ir::Block::CodeBlock(attr, doc.node(*text).0.to_string());
        self.blocks.push(block);

        Ok(())
    }

    fn visit_heading(
        &mut self,
        heading: Full<tree::Heading>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let tree::Heading {
            marker,
            content,
            label,
        } = heading.0;

        let attr = ir::AttrBuilder::new()
            .ident_opt(label.map(|id| doc.node(id).0.to_string()))
            .build();

        for id in content {
            self.visit_inline(doc.full(*id), doc)?;
        }

        let block = ir::Block::Header(doc.node(*marker).0 as i64, attr, self.take_inlines());
        self.blocks.push(block);

        Ok(())
    }

    fn visit_table(&mut self, table: Full<tree::Table>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Table {
            rows,
            columns,
            label,
        } = table.0;

        for id in rows {
            self.visit_table_row(doc.full(*id), doc)?;
        }

        let rows = self.take_rows();

        let attr = ir::AttrBuilder::new()
            .ident_opt(label.map(|id| doc.node(id).0.to_string()))
            .build();
        let caption = (None, Vec::new());
        let col_spec = (ir::Alignment::AlignCenter, ir::ColWidth::ColWidthDefault);
        let col_specs = vec![col_spec; *columns];
        let head = (ir::AttrBuilder::empty(), Vec::new());
        let body = vec![(ir::AttrBuilder::empty(), *columns as i64, rows, Vec::new())];
        let foot = (ir::AttrBuilder::empty(), Vec::new());

        let block = ir::Block::Table(attr, caption, col_specs, head, body, foot);
        self.blocks.push(block);
        Ok(())
    }

    fn visit_table_row(
        &mut self,
        table_row: Full<tree::TableRow>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let blocks = self.take_blocks();

        for id in &table_row.0 .0 {
            self.visit_block(doc.full(*id), doc)?;
        }

        let cells = self
            .replace_blocks(blocks)
            .into_iter()
            .map(|block| {
                (
                    ir::AttrBuilder::empty(),
                    ir::Alignment::AlignCenter,
                    1,
                    1,
                    vec![block],
                )
            })
            .collect();

        self.rows.push((ir::AttrBuilder::empty(), cells));
        Ok(())
    }

    fn visit_list(&mut self, list: Full<tree::List>, doc: &Doc) -> Result<(), Self::Error> {
        let bullet_list = self.take_bullet_list();

        self.walk_list(list, doc)?;

        let bullet_list = self.replace_bullet_list(bullet_list);
        let block = ir::Block::BulletList(bullet_list);
        self.blocks.push(block);
        Ok(())
    }

    fn visit_list_item(
        &mut self,
        list_item: Full<tree::ListItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let blocks = self.take_blocks();

        for id in &list_item.0 .0 {
            self.visit_block(doc.full(*id), doc)?;
        }

        let bullet_point = self.replace_blocks(blocks);

        self.bullet_list.push(bullet_point);
        Ok(())
    }

    fn visit_enum(&mut self, enumeration: Full<tree::Enum>, doc: &Doc) -> Result<(), Self::Error> {
        let ordered_list = self.take_ordered_list();

        self.walk_enum(enumeration, doc)?;

        let ordered_list = self.replace_ordered_list(ordered_list);
        let attrs = (1, ir::ListNumberStyle::Decimal, ir::ListNumberDelim::Period);
        let block = ir::Block::OrderedList(attrs, ordered_list);
        self.blocks.push(block);
        Ok(())
    }

    fn visit_enum_item(
        &mut self,
        enum_item: Full<tree::EnumItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let blocks = self.take_blocks();

        for id in &enum_item.0 .0 {
            self.visit_block(doc.full(*id), doc)?;
        }

        let ordered_point = self.replace_blocks(blocks);

        self.ordered_list.push(ordered_point);
        Ok(())
    }

    fn visit_terms(&mut self, terms: Full<tree::Terms>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_terms(terms, doc)?;

        let definition_list = self.take_definitions();
        let block = ir::Block::DefinitionList(definition_list);
        self.blocks.push(block);

        Ok(())
    }

    fn visit_term_item(
        &mut self,
        term_item: Full<tree::TermItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        let tree::TermItem { term, desc } = term_item.0;

        for id in term {
            self.visit_inline(doc.full(*id), doc)?;
        }

        let definition = self.take_inlines();

        for id in desc {
            self.visit_inline(doc.full(*id), doc)?;
        }

        let body = self.replace_inlines(inlines);

        self.definitions
            .push((definition, vec![vec![ir::Block::Plain(body)]]));
        Ok(())
    }

    fn visit_paragraph(
        &mut self,
        paragraph: Full<tree::Paragraph>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_paragraph(paragraph, doc)?;

        let block = ir::Block::Para(self.take_inlines());
        self.blocks.push(block);
        Ok(())
    }

    fn visit_plain(&mut self, plain: Full<tree::Plain>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_plain(plain, doc)?;

        let block = ir::Block::Plain(self.take_inlines());
        self.blocks.push(block);
        Ok(())
    }

    fn visit_quote(&mut self, quote: Full<tree::Quote>, doc: &Doc) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_quote(quote, doc)?;

        let inline = ir::Inline::Quoted(ir::QuoteType::DoubleQuote, self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    fn visit_strikeout(
        &mut self,
        strikeout: Full<tree::Strikeout>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_strikeout(strikeout, doc)?;

        let inline = ir::Inline::Strikeout(self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    fn visit_emphasis(
        &mut self,
        emphasis: Full<tree::Emphasis>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_emphasis(emphasis, doc)?;

        let inline = ir::Inline::Emph(self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    fn visit_strong(&mut self, strong: Full<tree::Strong>, doc: &Doc) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_strong(strong, doc)?;

        let inline = ir::Inline::Strong(self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    fn visit_subscript(
        &mut self,
        subscript: Full<tree::Subscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_subscript(subscript, doc)?;

        let inline = ir::Inline::Subscript(self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    fn visit_supscript(
        &mut self,
        supscript: Full<tree::Supscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_supscript(supscript, doc)?;

        let inline = ir::Inline::Superscript(self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    fn visit_link(&mut self, link: Full<tree::Link>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Link { href, content } = link.0;

        let href = doc.node(*href).0.to_string();

        let inlines = self.take_inlines();

        if let Some(content) = content {
            for id in content {
                self.visit_inline(doc.full(*id), doc)?;
            }
        }

        let mut content = self.replace_inlines(inlines);

        if content.is_empty() {
            content.push(ir::Inline::Str(href.clone()));
        }

        let inline = ir::Inline::Link(ir::AttrBuilder::empty(), content, (href, String::new()));
        self.inlines.push(inline);
        Ok(())
    }

    fn visit_ref(&mut self, reference: Full<tree::Ref>, _doc: &Doc) -> Result<(), Self::Error> {
        let href = format!("#{}", reference.0 .0);
        let content = vec![ir::Inline::Str(reference.0 .0.to_string())];

        let inline = ir::Inline::Link(ir::AttrBuilder::empty(), content, (href, String::new()));
        self.inlines.push(inline);
        Ok(())
    }

    fn visit_raw_inline(
        &mut self,
        raw_inline: Full<tree::RawInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Code(ir::AttrBuilder::empty(), raw_inline.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    fn visit_math_inline(
        &mut self,
        math_inline: Full<tree::MathInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Math(ir::MathType::InlineMath, math_inline.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    fn visit_escape(&mut self, escape: Full<tree::Escape>, _doc: &Doc) -> Result<(), Self::Error> {
        let inline = ir::Inline::Str(escape.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    fn visit_word(&mut self, word: Full<tree::Word>, _doc: &Doc) -> Result<(), Self::Error> {
        let inline = ir::Inline::Str(word.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    fn visit_spacing(
        &mut self,
        _spacing: Full<tree::Spacing>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.inlines.push(ir::Inline::Space);
        Ok(())
    }

    fn visit_soft_break(
        &mut self,
        _soft_break: Full<tree::SoftBreak>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.inlines.push(ir::Inline::SoftBreak);
        Ok(())
    }

    fn visit_code(&mut self, (code, id): Full<tree::Code>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_code((code, id), doc)?;

        let inline = match self.stack.pop().unwrap() {
            Value::Block(block) => {
                if self.inlines.is_empty() {
                    self.blocks.push(block);
                } else {
                    self.tracer.node_error(id, EngineMessage::ExpectedInline);
                }

                return Ok(());
            }
            Value::Inline(inline) => inline,
            Value::Content(content) => ir::Inline::Span(ir::AttrBuilder::empty(), content),
            Value::Bool(b) => ir::Inline::Str(format!("{b}")),
            Value::Float(f) => ir::Inline::Str(format!("{f}")),
            Value::Int(i) => ir::Inline::Str(format!("{i}")),
            Value::Str(s) => ir::Inline::Str(s.to_string()),
            Value::List(l) => ir::Inline::Str(format!("{l:?}")),
            Value::Map(m) => ir::Inline::Str(format!("{m:?}")),
            Value::None => ir::Inline::Str(format!("{:?}", None::<()>)),
            Value::Func(f) => ir::Inline::Str(format!("{f:?}")),
        };

        self.inlines.push(inline);

        Ok(())
    }

    fn visit_let(&mut self, let_: Full<tree::Let>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &let_.0 .0 {
            self.visit_bind(doc.full(*id), doc)?;
        }

        self.stack.push(Value::None);
        Ok(())
    }

    fn visit_bind(&mut self, bind: Full<tree::Bind>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Bind { name, value } = bind.0;

        let name = doc.node(*name).0.clone();

        self.visit_expr(doc.full(*value), doc)?;

        let value = self.stack.pop().unwrap();

        self.scope.insert(name, value);

        Ok(())
    }

    fn visit_if(&mut self, (if_, id): Full<tree::If>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::If {
            predicate,
            then,
            or,
        } = if_;

        self.visit_expr(doc.full(*predicate), doc)?;

        let pred = self.stack.pop().unwrap();
        let got = pred.ty();

        let pred = match pred.into_bool() {
            Some(p) => p,
            None => {
                self.stack.push(Value::None);
                self.tracer.node_error(
                    id,
                    ArgumentError::WrongType {
                        got,
                        expected: Type::Bool,
                    },
                );
                return Ok(());
            }
        };

        if pred {
            self.visit_content(doc.full(*then), doc)?;
        } else {
            self.visit_content(doc.full(*or), doc)?;
        }

        Ok(())
    }

    fn visit_for(&mut self, (for_, id): Full<tree::For>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::For {
            el,
            inside,
            content,
        } = for_;

        self.visit_expr(doc.full(*inside), doc)?;

        // TODO allow maps

        let collection = self.stack.pop().unwrap();
        let got = collection.ty();

        let collection = match collection.into_list() {
            Some(list) => list,
            None => {
                self.stack.push(Value::None);
                self.tracer.node_error(
                    id,
                    ArgumentError::WrongType {
                        got,
                        expected: Type::list(Type::Any),
                    },
                );
                return Ok(());
            }
        };

        let stack = self.take_stack();

        let name = doc.node(*el).0.clone();
        for value in collection {
            self.scope.insert(name.clone(), value);

            self.visit_content(doc.full(*content), doc)?;
        }

        let content = self
            .replace_stack(stack)
            .into_iter()
            .flat_map(|val| val.into_content().unwrap())
            .collect();
        self.stack.push(Value::Content(content));

        Ok(())
    }

    fn visit_call(&mut self, (call, id): Full<tree::Call>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Call { ident, args } = call;

        let ident = &doc.node(*ident).0;

        match self.scope.lookup::<ir::Func>(ident) {
            Some(f) => {
                let stack = self.take_stack();

                self.scope.enter();
                self.visit_args(doc.full(*args), doc)?;
                let args = ir::Arguments {
                    named: self.scope.exit().scope,
                    positional: self.take_stack(),
                    span: self.tracer.span(id),
                    source: self.source.clone(),
                };

                let result = f(args, &mut self.tracer);

                self.replace_stack(stack);
                self.stack.push(result);
            }
            None => {
                self.stack.push(Value::None);
                self.tracer
                    .node_error(id, EngineMessage::FunctionNotFound(ident.clone()));
            }
        }

        Ok(())
    }

    fn visit_arg(&mut self, arg: Full<tree::Arg>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Arg { name: key, value } = arg.0;

        self.visit_expr(doc.full(*value), doc)?;

        if let Some(id) = key {
            let name = doc.node(*id).0.clone();
            let value = self.stack.pop().unwrap();

            self.scope.insert(name, value);
        }

        Ok(())
    }

    fn visit_literal(
        &mut self,
        literal: Full<tree::Literal>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        let value = match literal.0.clone() {
            tree::Literal::Bool(b) => Value::Bool(b),
            tree::Literal::Str(s) => Value::Str(s),
            tree::Literal::Int(i) => Value::Int(i),
            tree::Literal::Float(f) => Value::Float(f),
        };

        self.stack.push(value);
        Ok(())
    }

    fn visit_ident(
        &mut self,
        (ident, id): Full<tree::Ident>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        match self.scope.lookup(&ident.0) {
            Some(v) => self.stack.push(v),
            None => {
                self.stack.push(Value::None);
                self.tracer
                    .node_error(id, EngineMessage::SymbolNotFound(ident.0.clone()));
            }
        }

        Ok(())
    }

    fn visit_content(
        &mut self,
        content: Full<tree::Content>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.scope.enter();

        let inlines = self.take_inlines();

        for id in &content.0 .0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        let content = self.replace_inlines(inlines);
        self.stack.push(Value::Content(content));

        self.scope.exit();

        Ok(())
    }
}
