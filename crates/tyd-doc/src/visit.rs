use crate::{Full, doc::Doc, tree};

pub trait Visitor {
    type Error;

    fn visit_error(&mut self, _error: Full<tree::Error>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_tag(&mut self, _tag: Full<tree::Tag>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_text(&mut self, _text: Full<tree::Text>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_label(&mut self, _label: Full<tree::Label>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_block(&mut self, block: Full<tree::Block>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_block(block, doc)
    }

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

    fn visit_raw(&mut self, raw: Full<tree::Raw>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_raw(raw, doc)
    }

    fn walk_raw(&mut self, raw: Full<tree::Raw>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Raw { text, lang } = *raw.0;

        self.visit_text(doc.full(text), doc)?;

        if let Some(id) = lang {
            self.visit_tag(doc.full(id), doc)?;
        }
        Ok(())
    }

    fn visit_heading(
        &mut self,
        heading: Full<tree::Heading>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_heading(heading, doc)
    }

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

    fn visit_heading_marker(
        &mut self,
        _heading_marker: Full<tree::HeadingMarker>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_table(&mut self, table: Full<tree::Table>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_table(table, doc)
    }

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

    fn visit_table_row(
        &mut self,
        table_row: Full<tree::TableRow>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_table_row(table_row, doc)
    }

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

    fn visit_list(&mut self, list: Full<tree::List>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_list(list, doc)
    }

    fn walk_list(&mut self, list: Full<tree::List>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &list.0.0 {
            self.visit_list_item(doc.full(*id), doc)?;
        }

        Ok(())
    }

    fn visit_list_item(
        &mut self,
        list_item: Full<tree::ListItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_list_item(list_item, doc)
    }

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

    fn visit_enum(&mut self, enumeration: Full<tree::Enum>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_enum(enumeration, doc)
    }

    fn walk_enum(&mut self, enumeration: Full<tree::Enum>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &enumeration.0.0 {
            self.visit_enum_item(doc.full(*id), doc)?;
        }
        Ok(())
    }

    fn visit_enum_item(
        &mut self,
        enum_item: Full<tree::EnumItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_enum_item(enum_item, doc)
    }

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

    fn visit_terms(&mut self, terms: Full<tree::Terms>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_terms(terms, doc)
    }

    fn walk_terms(&mut self, terms: Full<tree::Terms>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &terms.0.0 {
            self.visit_term_item(doc.full(*id), doc)?;
        }
        Ok(())
    }

    fn visit_term_item(
        &mut self,
        term_item: Full<tree::TermItem>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_term_item(term_item, doc)
    }

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

    fn visit_paragraph(
        &mut self,
        paragraph: Full<tree::Paragraph>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_paragraph(paragraph, doc)
    }

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

    fn visit_plain(&mut self, plain: Full<tree::Plain>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_plain(plain, doc)
    }

    fn walk_plain(&mut self, plain: Full<tree::Plain>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &plain.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }
        Ok(())
    }

    fn visit_inline(&mut self, inline: Full<tree::Inline>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_inline(inline, doc)
    }

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

    fn visit_quote(&mut self, quote: Full<tree::Quote>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_quote(quote, doc)
    }

    fn walk_quote(&mut self, quote: Full<tree::Quote>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &quote.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    fn visit_strikeout(
        &mut self,
        strikeout: Full<tree::Strikeout>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_strikeout(strikeout, doc)
    }

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

    fn visit_emphasis(
        &mut self,
        emphasis: Full<tree::Emphasis>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_emphasis(emphasis, doc)
    }

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

    fn visit_strong(&mut self, strong: Full<tree::Strong>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_strong(strong, doc)
    }

    fn walk_strong(&mut self, strong: Full<tree::Strong>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &strong.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }

    fn visit_subscript(
        &mut self,
        subscript: Full<tree::Subscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_subscript(subscript, doc)
    }

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

    fn visit_supscript(
        &mut self,
        supscript: Full<tree::Supscript>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_supscript(supscript, doc)
    }

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

    fn visit_link(&mut self, link: Full<tree::Link>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_link(link, doc)
    }

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

    fn visit_ref(&mut self, _reference: Full<tree::Ref>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_raw_inline(
        &mut self,
        _raw_inline: Full<tree::RawInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_math_inline(
        &mut self,
        _math_inline: Full<tree::MathInline>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_comment(
        &mut self,
        _comment: Full<tree::Comment>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_escape(&mut self, _escape: Full<tree::Escape>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_word(&mut self, _word: Full<tree::Word>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_spacing(
        &mut self,
        _spacing: Full<tree::Spacing>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_soft_break(
        &mut self,
        _soft_break: Full<tree::SoftBreak>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_code(&mut self, code: Full<tree::Code>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_code(code, doc)
    }

    fn walk_code(&mut self, code: Full<tree::Code>, doc: &Doc) -> Result<(), Self::Error> {
        self.visit_expr(doc.full(code.0.0), doc)
    }

    fn visit_expr(&mut self, expr: Full<tree::Expr>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_expr(expr, doc)
    }

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

    fn visit_let(&mut self, let_: Full<tree::Let>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_let(let_, doc)
    }

    fn walk_let(&mut self, let_: Full<tree::Let>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &let_.0.0 {
            self.visit_bind(doc.full(*id), doc)?;
        }
        Ok(())
    }

    fn visit_bind(&mut self, bind: Full<tree::Bind>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_bind(bind, doc)
    }

    fn walk_bind(&mut self, bind: Full<tree::Bind>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Bind { name, value } = bind.0;

        self.visit_ident(doc.full(*name), doc)?;
        self.visit_expr(doc.full(*value), doc)?;
        Ok(())
    }

    fn visit_if(&mut self, if_: Full<tree::If>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_if(if_, doc)
    }

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

    fn visit_for(&mut self, for_: Full<tree::For>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_for(for_, doc)
    }

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

    fn visit_call(&mut self, call: Full<tree::Call>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_call(call, doc)
    }

    fn walk_call(&mut self, call: Full<tree::Call>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Call { ident, args } = call.0;

        self.visit_ident(doc.full(*ident), doc)?;
        self.visit_args(doc.full(*args), doc)?;

        Ok(())
    }

    fn visit_args(&mut self, args: Full<tree::Args>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_args(args, doc)
    }

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

    fn visit_arg(&mut self, arg: Full<tree::Arg>, doc: &Doc) -> Result<(), Self::Error> {
        self.walk_arg(arg, doc)
    }

    fn walk_arg(&mut self, arg: Full<tree::Arg>, doc: &Doc) -> Result<(), Self::Error> {
        let tree::Arg { name: key, value } = arg.0;

        if let Some(id) = *key {
            self.visit_ident(doc.full(id), doc)?;
        }

        self.visit_expr(doc.full(*value), doc)?;

        Ok(())
    }

    fn visit_literal(
        &mut self,
        _literal: Full<tree::Literal>,
        _doc: &Doc,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_ident(&mut self, _ident: Full<tree::Ident>, _doc: &Doc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_content(
        &mut self,
        content: Full<tree::Content>,
        doc: &Doc,
    ) -> Result<(), Self::Error> {
        self.walk_content(content, doc)
    }

    fn walk_content(&mut self, content: Full<tree::Content>, doc: &Doc) -> Result<(), Self::Error> {
        for id in &content.0.0 {
            self.visit_inline(doc.full(*id), doc)?;
        }

        Ok(())
    }
}
