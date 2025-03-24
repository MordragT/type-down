use ecow::EcoString;
use std::{collections::BTreeMap, fmt::Debug, mem};
use tyd_core::prelude::*;
use tyd_syntax::{source::Source, SpanMetadata};

use crate::{
    error::{EngineError, SymbolError, TypeError},
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{Type, TypeCast, Value},
};

/// Result of running the document processing engine
///
/// Contains the processed Pandoc document and a tracer with potential errors
#[derive(Debug)]
pub struct EngineResult {
    /// The optional Pandoc document output
    /// Will be None if errors occurred during processing
    pub pandoc: Option<ir::Pandoc>,

    /// Tracer containing any errors that occurred during processing
    pub tracer: Tracer,
}

/// Document processing engine that transforms a TypeDown document into Pandoc format
///
/// The engine implements the Visitor pattern to traverse the document and build
/// the Pandoc representation with appropriate error handling.
#[derive(Debug)]
pub struct Engine {
    /// Inline elements being constructed
    inlines: Vec<ir::Inline>,

    /// Block elements being constructed
    blocks: Vec<ir::Block>,

    /// Table rows being constructed
    rows: Vec<ir::Row>,

    /// Definition list items being constructed
    definitions: Vec<ir::Definition>,

    /// Bullet list items being constructed
    bullet_list: Vec<Vec<ir::Block>>,

    /// Ordered list items being constructed
    ordered_list: Vec<Vec<ir::Block>>,

    /// Value stack for expression evaluation
    stack: Stack,

    /// Variable scope for document processing
    scope: Scope,

    /// Error tracking and reporting
    tracer: Tracer,

    /// Document source reference
    source: Source,

    /// Span metadata for error reporting
    spans: SpanMetadata,
}

impl Engine {
    /// Creates a new document processing engine
    ///
    /// # Arguments
    /// * `global_scope` - The global variable scope to use for document processing
    /// * `tracer` - Error tracker for collecting and reporting errors
    pub fn new(global_scope: Scope, tracer: Tracer) -> Self {
        Self {
            inlines: Vec::new(),
            blocks: Vec::new(),
            rows: Vec::new(),
            definitions: Vec::new(),
            bullet_list: Vec::new(),
            ordered_list: Vec::new(),
            stack: Stack::new(),
            scope: Scope::new(global_scope),
            source: tracer.source.clone(),
            spans: tracer.spans.clone(),
            tracer,
        }
    }

    /// Processes a document and produces a result
    ///
    /// Visits all nodes in the document, applies transformations, and collects
    /// metadata to produce a Pandoc document.
    ///
    /// # Arguments
    /// * `doc` - The document to process
    ///
    /// # Returns
    /// An `EngineResult` containing either the Pandoc document or error information
    pub fn run(mut self, doc: Doc) -> EngineResult {
        if let Err(tracer) = doc.visit_by(&mut self) {
            return EngineResult {
                pandoc: None,
                tracer,
            };
        }

        let Self {
            blocks,
            inlines,
            rows,
            definitions,
            bullet_list,
            ordered_list,
            stack,
            scope,
            mut tracer,
            source: _,
            spans: _,
        } = self;

        assert!(inlines.is_empty());
        assert!(rows.is_empty());
        assert!(definitions.is_empty());
        assert!(bullet_list.is_empty());
        assert!(ordered_list.is_empty());
        assert!(stack.is_empty());

        if tracer.has_errors() {
            return EngineResult {
                pandoc: None,
                tracer,
            };
        }

        let mut meta = BTreeMap::new();

        if let Some(title) = scope.try_get::<EcoString>("title") {
            match title {
                Ok(tittle) => {
                    meta.insert(
                        "title".to_owned(),
                        ir::MetaValue::MetaString(tittle.to_string()),
                    );
                }
                Err(got) => tracer.error(TypeError::WrongType {
                    got,
                    expected: Type::Str,
                }),
            };
        }

        let pandoc = ir::Pandoc {
            pandoc_api_version: vec![1, 23, 1],
            meta,
            blocks,
        };

        EngineResult {
            pandoc: Some(pandoc),
            tracer,
        }
    }

    /// Takes all accumulated blocks, leaving an empty collection
    fn take_blocks(&mut self) -> Vec<ir::Block> {
        mem::take(&mut self.blocks)
    }

    /// Takes all accumulated inlines, leaving an empty collection
    fn take_inlines(&mut self) -> Vec<ir::Inline> {
        mem::take(&mut self.inlines)
    }

    /// Takes all accumulated table rows, leaving an empty collection
    fn take_rows(&mut self) -> Vec<ir::Row> {
        mem::take(&mut self.rows)
    }

    /// Takes all accumulated definition items, leaving an empty collection
    fn take_definitions(&mut self) -> Vec<ir::Definition> {
        mem::take(&mut self.definitions)
    }

    /// Takes all accumulated bullet list items, leaving an empty collection
    fn take_bullet_list(&mut self) -> Vec<Vec<ir::Block>> {
        mem::take(&mut self.bullet_list)
    }

    /// Takes all accumulated ordered list items, leaving an empty collection
    fn take_ordered_list(&mut self) -> Vec<Vec<ir::Block>> {
        mem::take(&mut self.ordered_list)
    }

    /// Replaces blocks with a new collection and returns the old one
    fn replace_blocks(&mut self, src: Vec<ir::Block>) -> Vec<ir::Block> {
        mem::replace(&mut self.blocks, src)
    }

    /// Replaces inlines with a new collection and returns the old one
    fn replace_inlines(&mut self, src: Vec<ir::Inline>) -> Vec<ir::Inline> {
        mem::replace(&mut self.inlines, src)
    }

    /// Replaces rows with a new collection and returns the old one
    #[allow(unused)]
    fn replace_rows(&mut self, src: Vec<ir::Row>) -> Vec<ir::Row> {
        mem::replace(&mut self.rows, src)
    }

    /// Replaces definitions with a new collection and returns the old one
    #[allow(unused)]
    fn replace_definitions(&mut self, src: Vec<ir::Definition>) -> Vec<ir::Definition> {
        mem::replace(&mut self.definitions, src)
    }

    /// Replaces bullet list items with a new collection and returns the old one
    fn replace_bullet_list(&mut self, src: Vec<Vec<ir::Block>>) -> Vec<Vec<ir::Block>> {
        mem::replace(&mut self.bullet_list, src)
    }

    /// Replaces ordered list items with a new collection and returns the old one
    fn replace_ordered_list(&mut self, src: Vec<Vec<ir::Block>>) -> Vec<Vec<ir::Block>> {
        mem::replace(&mut self.ordered_list, src)
    }
}

impl Visitor for Engine {
    type Error = Tracer;

    /// Handles error nodes in the document
    fn visit_error(
        &mut self,
        (error, id): Full<tree::Error>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.tracer.node_error(id, error);
        Ok(())
    }

    /// Processes raw code blocks
    fn visit_raw(&mut self, raw: Full<tree::Raw>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Raw { lang, text } = raw.0;

        let lang = lang.map(|id| doc.node(id).0.to_string());
        let attr = ir::AttrBuilder::new().class_opt(lang).build();

        let block = ir::Block::CodeBlock(attr, doc.node(*text).0.to_string());
        self.blocks.push(block);

        Ok(())
    }

    /// Processes heading elements
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

    /// Processes table elements
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

    /// Processes table rows
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

    /// Processes unordered lists
    fn visit_list(&mut self, list: Full<tree::List>, doc: &Doc) -> Result<(), Self::Error> {
        let bullet_list = self.take_bullet_list();

        self.walk_list(list, doc)?;

        let bullet_list = self.replace_bullet_list(bullet_list);
        let block = ir::Block::BulletList(bullet_list);
        self.blocks.push(block);
        Ok(())
    }

    /// Processes unordered list items
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

    /// Processes ordered lists (enumerations)
    fn visit_enum(&mut self, enumeration: Full<tree::Enum>, doc: &Doc) -> Result<(), Self::Error> {
        let ordered_list = self.take_ordered_list();

        self.walk_enum(enumeration, doc)?;

        let ordered_list = self.replace_ordered_list(ordered_list);
        let attrs = (1, ir::ListNumberStyle::Decimal, ir::ListNumberDelim::Period);
        let block = ir::Block::OrderedList(attrs, ordered_list);
        self.blocks.push(block);
        Ok(())
    }

    /// Processes ordered list items
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

    /// Processes definition term lists
    fn visit_terms(&mut self, terms: Full<tree::Terms>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_terms(terms, doc)?;

        let definition_list = self.take_definitions();
        let block = ir::Block::DefinitionList(definition_list);
        self.blocks.push(block);

        Ok(())
    }

    /// Processes individual definition term items
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

    /// Processes paragraph blocks
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

    /// Processes plain text blocks
    fn visit_plain(&mut self, plain: Full<tree::Plain>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_plain(plain, doc)?;

        let block = ir::Block::Plain(self.take_inlines());
        self.blocks.push(block);
        Ok(())
    }

    /// Processes quoted text
    fn visit_quote(&mut self, quote: Full<tree::Quote>, doc: &Doc) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_quote(quote, doc)?;

        let inline = ir::Inline::Quoted(ir::QuoteType::DoubleQuote, self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    /// Processes strikeout text formatting
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

    /// Processes emphasized text formatting
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

    /// Processes strong (bold) text formatting
    fn visit_strong(&mut self, strong: Full<tree::Strong>, doc: &Doc) -> Result<(), Self::Error> {
        let inlines = self.take_inlines();

        self.walk_strong(strong, doc)?;

        let inline = ir::Inline::Strong(self.replace_inlines(inlines));
        self.inlines.push(inline);

        Ok(())
    }

    /// Processes subscript text formatting
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

    /// Processes superscript text formatting
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

    /// Processes hyperlinks
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

    /// Processes references to document elements
    fn visit_ref(&mut self, reference: Full<tree::Ref>, _doc: &Doc) -> Result<(), Self::Error> {
        let href = format!("#{}", reference.0 .0);
        let content = vec![ir::Inline::Str(reference.0 .0.to_string())];

        let inline = ir::Inline::Link(ir::AttrBuilder::empty(), content, (href, String::new()));
        self.inlines.push(inline);
        Ok(())
    }

    /// Processes raw inline code
    fn visit_raw_inline(
        &mut self,
        raw_inline: Full<tree::RawInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Code(ir::AttrBuilder::empty(), raw_inline.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    /// Processes inline math elements
    fn visit_math_inline(
        &mut self,
        math_inline: Full<tree::MathInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        let inline = ir::Inline::Math(ir::MathType::InlineMath, math_inline.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    /// Processes escaped characters
    fn visit_escape(&mut self, escape: Full<tree::Escape>, _doc: &Doc) -> Result<(), Self::Error> {
        let inline = ir::Inline::Str(escape.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    /// Processes word elements
    fn visit_word(&mut self, word: Full<tree::Word>, _doc: &Doc) -> Result<(), Self::Error> {
        let inline = ir::Inline::Str(word.0 .0.to_string());
        self.inlines.push(inline);
        Ok(())
    }

    /// Processes spacing elements
    fn visit_spacing(
        &mut self,
        _spacing: Full<tree::Spacing>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.inlines.push(ir::Inline::Space);
        Ok(())
    }

    /// Processes soft break elements
    fn visit_soft_break(
        &mut self,
        _soft_break: Full<tree::SoftBreak>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.inlines.push(ir::Inline::SoftBreak);
        Ok(())
    }

    /// Processes code execution blocks
    fn visit_code(&mut self, (code, id): Full<tree::Code>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_code((code, id), doc)?;

        let inline = match self.stack.pop().unwrap() {
            Value::Block(block) => {
                if self.inlines.is_empty() {
                    self.blocks.push(block);
                } else {
                    self.tracer.node_error(id, EngineError::ExpectedInline);
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

    /// Processes variable declarations
    fn visit_let(&mut self, let_: Full<tree::Let>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &let_.0 .0 {
            self.visit_bind(doc.full(*id), doc)?;
        }

        self.stack.push_none();
        Ok(())
    }

    /// Processes variable bindings
    fn visit_bind(&mut self, bind: Full<tree::Bind>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Bind { name, value } = bind.0;

        let name = doc.node(*name).0.clone();

        self.visit_expr(doc.full(*value), doc)?;

        let value = self.stack.pop().unwrap();

        self.scope.insert(name, value);

        Ok(())
    }

    /// Processes conditional statements
    fn visit_if(&mut self, (if_, id): Full<tree::If>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::If {
            predicate,
            then,
            or,
        } = if_;

        self.visit_expr(doc.full(*predicate), doc)?;

        let pred = match self.stack.try_pop::<bool>().unwrap() {
            Ok(p) => p,
            Err(got) => {
                self.stack.push_none();
                self.tracer.node_error(
                    id,
                    TypeError::WrongType {
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

    /// Processes for loop constructs
    fn visit_for(&mut self, (for_, id): Full<tree::For>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::For {
            el,
            inside,
            content,
        } = for_;

        self.visit_expr(doc.full(*inside), doc)?;

        // TODO allow maps

        let collection = match self.stack.try_pop::<ir::List>().unwrap() {
            Ok(list) => list,
            Err(got) => {
                self.stack.push_none();
                self.tracer.node_error(
                    id,
                    TypeError::WrongType {
                        got,
                        expected: Type::list(Type::Any),
                    },
                );
                return Ok(());
            }
        };

        let stack = self.stack.take();

        let name = doc.node(*el).0.clone();
        for value in collection {
            self.scope.insert(name.clone(), value);

            self.visit_content(doc.full(*content), doc)?;
        }

        let content = self
            .stack
            .replace(stack)
            .into_iter()
            .flat_map(|val| ir::Content::try_downcast(val).unwrap())
            .collect();
        self.stack.push(Value::Content(content));

        Ok(())
    }

    /// Processes function calls
    fn visit_call(&mut self, (call, id): Full<tree::Call>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Call { ident, args } = call;

        let ident = &doc.node(*ident).0;

        match self.scope.try_get::<ir::Func>(ident) {
            Some(Ok(f)) => {
                let stack = self.stack.take();

                self.scope.enter();
                self.visit_args(doc.full(*args), doc)?;

                let result = f(
                    self.stack.take(),
                    self.scope.exit(),
                    self.source.clone(),
                    self.spans.get(id).inner_copied(),
                    &mut self.tracer,
                );

                self.stack.replace(stack);
                self.stack.push(result);
            }
            Some(Err(got)) => {
                self.tracer.node_error(
                    id,
                    TypeError::WrongType {
                        got,
                        expected: Type::Func,
                    },
                );
                self.stack.push_none();
            }
            None => {
                self.stack.push_none();
                self.tracer
                    .node_error(id, SymbolError::NotFound(ident.clone()));
            }
        }

        Ok(())
    }

    /// Processes function arguments
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

    /// Processes literal values
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

    /// Processes identifier references
    fn visit_ident(
        &mut self,
        (ident, id): Full<tree::Ident>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        match self.scope.get(&ident.0) {
            Some(v) => self.stack.push(v),
            None => {
                self.stack.push_none();
                self.tracer
                    .node_error(id, SymbolError::NotFound(ident.0.clone()));
            }
        }

        Ok(())
    }

    /// Processes content blocks
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
