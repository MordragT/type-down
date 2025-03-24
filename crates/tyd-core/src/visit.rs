use crate::{Full, doc::Doc, tree};

/// A visitor trait for traversing and processing document tree nodes.
///
/// This trait provides a comprehensive set of methods for visiting different
/// types of nodes in a document tree. It follows a visitor pattern where
/// each node type has a corresponding `visit_*` method that can be overridden
/// to provide custom behavior.
///
/// Each `visit_*` method has a default implementation that calls the corresponding
/// `walk_*` method, which recursively visits child nodes. This allows for easily
/// implementing custom behavior for specific node types while letting the default
/// traversal handle the rest.
///
/// # Type Parameters
///
/// * `Error` - The error type that can be returned by visitor methods
///
/// # Examples
///
/// ```
/// struct MyVisitor;
///
/// impl Visitor for MyVisitor {
///     type Error = String;
///
///     fn visit_heading(&mut self, heading: Full<tree::Heading>, doc: &Doc) -> Result<(), Self::Error> {
///         println!("Found a heading!");
///         self.walk_heading(heading, doc)
///     }
/// }
/// ```
pub trait Visitor {
    /// The error type that can be returned by visitor methods.
    type Error;

    /// Visit an error node in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_error(&mut self, _error: Full<tree::Error>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a tag node in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_tag(&mut self, _tag: Full<tree::Tag>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a text node in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_text(&mut self, _text: Full<tree::Text>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a label node in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_label(&mut self, _label: Full<tree::Label>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a block node in the document tree.
    ///
    /// Default implementation calls walk_block to traverse the block's children.
    fn visit_block(&mut self, block: Full<tree::Block>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_block(block, doc)
    }

    /// Walk through a block node in the document tree and visit its children.
    ///
    /// Dispatches to the appropriate visit method based on the block type.
    fn walk_block(&mut self, block: Full<tree::Block>, doc: &Doc) -> Result<(), Self::Error> {
        match *block.0 {
            tree::Block::Raw(id) => self.visit_raw(doc.full(id), doc),
            tree::Block::Heading(id) => self.visit_heading(doc.full(id), doc),
            tree::Block::Table(id) => self.visit_table(doc.full(id), doc),
            tree::Block::List(id) => self.visit_list(doc.full(id), doc),
            tree::Block::Enum(id) => self.visit_enum(doc.full(id), doc),
            tree::Block::Terms(id) => self.visit_terms(doc.full(id), doc),
            tree::Block::Paragraph(id) => self.visit_paragraph(doc.full(id), doc),
            tree::Block::Plain(id) => self.visit_plain(doc.full(id), doc),
        }
    }

    /// Visit a raw block in the document tree.
    ///
    /// Default implementation calls walk_raw to traverse its children.
    fn visit_raw(&mut self, raw: Full<tree::Raw>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_raw(raw, doc)
    }

    /// Walk through a raw block and visit its children.
    ///
    /// Visits the text content and optional language tag.
    fn walk_raw(&mut self, raw: Full<tree::Raw>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Raw { text, lang } = *raw.0;

        self.visit_text(doc.full(text), doc)?;

        if let Some(id) = lang {
            self.visit_tag(doc.full(id), doc)?;
        }
        Ok(())
    }

    /// Visit a heading block in the document tree.
    ///
    /// Default implementation calls walk_heading to traverse its children.
    fn visit_heading(
        &mut self,
        heading: Full<tree::Heading>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_heading(heading, doc)
    }

    /// Walk through a heading block and visit its children.
    ///
    /// Visits the heading marker, content, and optional label.
    fn walk_heading(&mut self, heading: Full<tree::Heading>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Heading {
            marker,
            content,
            label,
        } = heading.0;

        self.visit_heading_marker(doc.full(*marker), doc)?;

        for id in content {
            self.visit_inline(doc.full(*id), doc)?;
        }

        if let Some(id) = *label {
            self.visit_label(doc.full(id), doc)?;
        }

        Ok(())
    }

    /// Visit a heading marker in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_heading_marker(
        &mut self,
        _heading_marker: Full<tree::HeadingMarker>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a table block in the document tree.
    ///
    /// Default implementation calls walk_table to traverse its children.
    fn visit_table(&mut self, table: Full<tree::Table>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_table(table, doc)
    }

    /// Walk through a table block and visit its children.
    ///
    /// Visits each table row and optional label.
    fn walk_table(&mut self, table: Full<tree::Table>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Table {
            rows,
            columns: _,
            label,
        } = table.0;

        for id in rows {
            self.visit_table_row(doc.full(*id), doc)?;
        }

        if let Some(id) = *label {
            self.visit_label(doc.full(id), doc)?;
        }
        Ok(())
    }

    /// Visit a table row in the document tree.
    ///
    /// Default implementation calls walk_table_row to traverse its children.
    fn visit_table_row(
        &mut self,
        table_row: Full<tree::TableRow>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_table_row(table_row, doc)
    }

    /// Walk through a table row and visit its cells (blocks).
    fn walk_table_row(
        &mut self,
        table_row: Full<tree::TableRow>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &table_row.0.0 {
            self.visit_block(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a list block in the document tree.
    ///
    /// Default implementation calls walk_list to traverse its items.
    fn visit_list(&mut self, list: Full<tree::List>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_list(list, doc)
    }

    /// Walk through a list block and visit each list item.
    fn walk_list(&mut self, list: Full<tree::List>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &list.0.0 {
            self.visit_list_item(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a list item in the document tree.
    ///
    /// Default implementation calls walk_list_item to traverse its children.
    fn visit_list_item(
        &mut self,
        list_item: Full<tree::ListItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_list_item(list_item, doc)
    }

    /// Walk through a list item and visit its children (blocks).
    fn walk_list_item(
        &mut self,
        list_item: Full<tree::ListItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &list_item.0.0 {
            self.visit_block(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit an enumeration block in the document tree.
    ///
    /// Default implementation calls walk_enum to traverse its items.
    fn visit_enum(&mut self, enumeration: Full<tree::Enum>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_enum(enumeration, doc)
    }

    /// Walk through an enumeration block and visit each enum item.
    fn walk_enum(&mut self, enumeration: Full<tree::Enum>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &enumeration.0.0 {
            self.visit_enum_item(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit an enumeration item in the document tree.
    ///
    /// Default implementation calls walk_enum_item to traverse its children.
    fn visit_enum_item(
        &mut self,
        enum_item: Full<tree::EnumItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_enum_item(enum_item, doc)
    }

    /// Walk through an enum item and visit its children (blocks).
    fn walk_enum_item(
        &mut self,
        enum_item: Full<tree::EnumItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &enum_item.0.0 {
            self.visit_block(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit a terms block in the document tree.
    ///
    /// Default implementation calls walk_terms to traverse its items.
    fn visit_terms(&mut self, terms: Full<tree::Terms>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_terms(terms, doc)
    }

    /// Walk through a terms block and visit each term item.
    fn walk_terms(&mut self, terms: Full<tree::Terms>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &terms.0.0 {
            self.visit_term_item(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit a term item in the document tree.
    ///
    /// Default implementation calls walk_term_item to traverse its children.
    fn visit_term_item(
        &mut self,
        term_item: Full<tree::TermItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_term_item(term_item, doc)
    }

    /// Walk through a term item and visit its term and description parts.
    fn walk_term_item(
        &mut self,
        term_item: Full<tree::TermItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        let tree::TermItem { term, desc } = term_item.0;

        for id in term {
            self.visit_inline(doc.full(*id), doc)?;
        }

        for id in desc {
            self.visit_inline(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit a paragraph block in the document tree.
    ///
    /// Default implementation calls walk_paragraph to traverse its children.
    fn visit_paragraph(
        &mut self,
        paragraph: Full<tree::Paragraph>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_paragraph(paragraph, doc)
    }

    /// Walk through a paragraph block and visit its inline elements.
    fn walk_paragraph(
        &mut self,
        paragraph: Full<tree::Paragraph>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &paragraph.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit a plain block in the document tree.
    ///
    /// Default implementation calls walk_plain to traverse its children.
    fn visit_plain(&mut self, plain: Full<tree::Plain>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_plain(plain, doc)
    }

    /// Walk through a plain block and visit its inline elements.
    fn walk_plain(&mut self, plain: Full<tree::Plain>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &plain.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit an inline element in the document tree.
    ///
    /// Default implementation calls walk_inline to traverse based on element type.
    fn visit_inline(&mut self, inline: Full<tree::Inline>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_inline(inline, doc)
    }

    /// Walk through an inline element, dispatching to the appropriate visitor method
    /// based on the specific type of inline element.
    fn walk_inline(&mut self, inline: Full<tree::Inline>, doc: &Doc) -> Result<(), Self::Error> {
        match inline.0 {
            tree::Inline::Error(id) => self.visit_error(doc.full(*id), doc),
            tree::Inline::Quote(id) => self.visit_quote(doc.full(*id), doc),
            tree::Inline::Strikeout(id) => self.visit_strikeout(doc.full(*id), doc),
            tree::Inline::Emphasis(id) => self.visit_emphasis(doc.full(*id), doc),
            tree::Inline::Strong(id) => self.visit_strong(doc.full(*id), doc),
            tree::Inline::Subscript(id) => self.visit_subscript(doc.full(*id), doc),
            tree::Inline::Supscript(id) => self.visit_supscript(doc.full(*id), doc),
            tree::Inline::Link(id) => self.visit_link(doc.full(*id), doc),
            tree::Inline::Ref(id) => self.visit_ref(doc.full(*id), doc),
            tree::Inline::RawInline(id) => self.visit_raw_inline(doc.full(*id), doc),
            tree::Inline::MathInline(id) => self.visit_math_inline(doc.full(*id), doc),
            tree::Inline::Comment(id) => self.visit_comment(doc.full(*id), doc),
            tree::Inline::Escape(id) => self.visit_escape(doc.full(*id), doc),
            tree::Inline::Word(id) => self.visit_word(doc.full(*id), doc),
            tree::Inline::Spacing(id) => self.visit_spacing(doc.full(*id), doc),
            tree::Inline::SoftBreak(id) => self.visit_soft_break(doc.full(*id), doc),
            tree::Inline::Code(id) => self.visit_code(doc.full(*id), doc),
        }
    }

    /// Visit a quote element in the document tree.
    ///
    /// Default implementation calls walk_quote to traverse its children.
    fn visit_quote(&mut self, quote: Full<tree::Quote>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_quote(quote, doc)
    }

    /// Walk through a quote element and visit its inline children.
    fn walk_quote(&mut self, quote: Full<tree::Quote>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &quote.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a strikeout element in the document tree.
    ///
    /// Default implementation calls walk_strikeout to traverse its children.
    fn visit_strikeout(
        &mut self,
        strikeout: Full<tree::Strikeout>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_strikeout(strikeout, doc)
    }

    /// Walk through a strikeout element and visit its inline children.
    fn walk_strikeout(
        &mut self,
        strikeout: Full<tree::Strikeout>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &strikeout.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit an emphasis element in the document tree.
    ///
    /// Default implementation calls walk_emphasis to traverse its children.
    fn visit_emphasis(
        &mut self,
        emphasis: Full<tree::Emphasis>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_emphasis(emphasis, doc)
    }

    /// Walk through an emphasis element and visit its inline children.
    fn walk_emphasis(
        &mut self,
        emphasis: Full<tree::Emphasis>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &emphasis.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a strong element in the document tree.
    ///
    /// Default implementation calls walk_strong to traverse its children.
    fn visit_strong(&mut self, strong: Full<tree::Strong>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_strong(strong, doc)
    }

    /// Walk through a strong element and visit its inline children.
    fn walk_strong(&mut self, strong: Full<tree::Strong>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &strong.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a subscript element in the document tree.
    ///
    /// Default implementation calls walk_subscript to traverse its children.
    fn visit_subscript(
        &mut self,
        subscript: Full<tree::Subscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_subscript(subscript, doc)
    }

    /// Walk through a subscript element and visit its inline children.
    fn walk_subscript(
        &mut self,
        subscript: Full<tree::Subscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &subscript.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a superscript element in the document tree.
    ///
    /// Default implementation calls walk_supscript to traverse its children.
    fn visit_supscript(
        &mut self,
        supscript: Full<tree::Supscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_supscript(supscript, doc)
    }

    /// Walk through a superscript element and visit its inline children.
    fn walk_supscript(
        &mut self,
        supscript: Full<tree::Supscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        for id in &supscript.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    /// Visit a link element in the document tree.
    ///
    /// Default implementation calls walk_link to traverse its href and content.
    fn visit_link(&mut self, link: Full<tree::Link>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_link(link, doc)
    }

    /// Walk through a link element and visit its href text and optional content.
    fn walk_link(&mut self, link: Full<tree::Link>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Link { href, content } = link.0;

        self.visit_text(doc.full(*href), doc)?;

        if let Some(content) = content {
            for id in content {
                self.visit_inline(doc.full(*id), doc)?;
            }
        }

        Ok(())
    }

    /// Visit a reference element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_ref(&mut self, _reference: Full<tree::Ref>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a raw inline element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_raw_inline(
        &mut self,
        _raw_inline: Full<tree::RawInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a math inline element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_math_inline(
        &mut self,
        _math_inline: Full<tree::MathInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a comment element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_comment(
        &mut self,
        _comment: Full<tree::Comment>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit an escape element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_escape(&mut self, _escape: Full<tree::Escape>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a word element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_word(&mut self, _word: Full<tree::Word>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a spacing element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_spacing(
        &mut self,
        _spacing: Full<tree::Spacing>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a soft break element in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_soft_break(
        &mut self,
        _soft_break: Full<tree::SoftBreak>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a code element in the document tree.
    ///
    /// Default implementation calls walk_code to traverse its expression.
    fn visit_code(&mut self, code: Full<tree::Code>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_code(code, doc)
    }

    /// Walk through a code element and visit its expression.
    fn walk_code(&mut self, code: Full<tree::Code>, doc: &Doc) -> Result<(), Self::Error> {
        self.visit_expr(doc.full(code.0.0), doc)
    }

    /// Visit an expression element in the document tree.
    ///
    /// Default implementation calls walk_expr to traverse based on expression type.
    fn visit_expr(&mut self, expr: Full<tree::Expr>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_expr(expr, doc)
    }

    /// Walk through an expression and dispatch to the appropriate visitor method
    /// based on the specific type of expression.
    fn walk_expr(&mut self, expr: Full<tree::Expr>, doc: &Doc) -> Result<(), Self::Error> {
        match *expr.0 {
            tree::Expr::Let(id) => self.visit_let(doc.full(id), doc),
            tree::Expr::If(id) => self.visit_if(doc.full(id), doc),
            tree::Expr::For(id) => self.visit_for(doc.full(id), doc),
            tree::Expr::Call(id) => self.visit_call(doc.full(id), doc),
            tree::Expr::Literal(id) => self.visit_literal(doc.full(id), doc),
            tree::Expr::Ident(id) => self.visit_ident(doc.full(id), doc),
            tree::Expr::Content(id) => self.visit_content(doc.full(id), doc),
        }
    }

    /// Visit a let expression in the document tree.
    ///
    /// Default implementation calls walk_let to traverse its bindings.
    fn visit_let(&mut self, let_: Full<tree::Let>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_let(let_, doc)
    }

    /// Walk through a let expression and visit its bindings.
    fn walk_let(&mut self, let_: Full<tree::Let>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &let_.0.0 {
            self.visit_bind(doc.full(*id), doc)?;
        }
        Ok(())
    }

    /// Visit a bind element in a let expression.
    ///
    /// Default implementation calls walk_bind to traverse its components.
    fn visit_bind(&mut self, bind: Full<tree::Bind>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_bind(bind, doc)
    }

    /// Walk through a bind element and visit its name identifier and value expression.
    fn walk_bind(&mut self, bind: Full<tree::Bind>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Bind { name, value } = bind.0;

        self.visit_ident(doc.full(*name), doc)?;
        self.visit_expr(doc.full(*value), doc)?;
        Ok(())
    }

    /// Visit an if expression in the document tree.
    ///
    /// Default implementation calls walk_if to traverse its components.
    fn visit_if(&mut self, if_: Full<tree::If>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_if(if_, doc)
    }

    /// Walk through an if expression and visit its predicate, then, and or branches.
    fn walk_if(&mut self, if_: Full<tree::If>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::If {
            predicate,
            then,
            or,
        } = if_.0;

        self.visit_expr(doc.full(*predicate), doc)?;
        self.visit_content(doc.full(*then), doc)?;
        self.visit_content(doc.full(*or), doc)?;

        Ok(())
    }

    /// Visit a for expression in the document tree.
    ///
    /// Default implementation calls walk_for to traverse its components.
    fn visit_for(&mut self, for_: Full<tree::For>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_for(for_, doc)
    }

    /// Walk through a for expression and visit its element, iterable, and content.
    fn walk_for(&mut self, for_: Full<tree::For>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::For {
            el,
            inside,
            content,
        } = for_.0;

        self.visit_ident(doc.full(*el), doc)?;
        self.visit_expr(doc.full(*inside), doc)?;
        self.visit_content(doc.full(*content), doc)?;

        Ok(())
    }

    /// Visit a function call expression in the document tree.
    ///
    /// Default implementation calls walk_call to traverse its components.
    fn visit_call(&mut self, call: Full<tree::Call>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_call(call, doc)
    }

    /// Walk through a call expression and visit its identifier and arguments.
    fn walk_call(&mut self, call: Full<tree::Call>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Call { ident, args } = call.0;

        self.visit_ident(doc.full(*ident), doc)?;
        self.visit_args(doc.full(*args), doc)?;

        Ok(())
    }

    /// Visit an arguments list in the document tree.
    ///
    /// Default implementation calls walk_args to traverse its components.
    fn visit_args(&mut self, args: Full<tree::Args>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_args(args, doc)
    }

    /// Walk through an arguments list and visit each argument and optional content.
    fn walk_args(&mut self, args: Full<tree::Args>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Args { args, content } = args.0;

        for id in args {
            self.visit_arg(doc.full(*id), doc)?;
        }

        if let Some(id) = *content {
            self.visit_content(doc.full(id), doc)?;
        }

        Ok(())
    }

    /// Visit an argument in a function call.
    ///
    /// Default implementation calls walk_arg to traverse its components.
    fn visit_arg(&mut self, arg: Full<tree::Arg>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_arg(arg, doc)
    }

    /// Walk through an argument and visit its optional name and value expression.
    fn walk_arg(&mut self, arg: Full<tree::Arg>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Arg { name: key, value } = arg.0;

        if let Some(id) = *key {
            self.visit_ident(doc.full(id), doc)?;
        }

        self.visit_expr(doc.full(*value), doc)?;

        Ok(())
    }

    /// Visit a literal value in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_literal(
        &mut self,
        _literal: Full<tree::Literal>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit an identifier in the document tree.
    ///
    /// Default implementation does nothing and returns Ok.
    fn visit_ident(&mut self, _ident: Full<tree::Ident>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a content block in the document tree.
    ///
    /// Default implementation calls walk_content to traverse its inline elements.
    fn visit_content(
        &mut self,
        content: Full<tree::Content>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_content(content, doc)
    }

    /// Walk through a content block and visit each of its inline elements.
    fn walk_content(&mut self, content: Full<tree::Content>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &content.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }
}
