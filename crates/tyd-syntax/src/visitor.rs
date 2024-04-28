use std::fmt::Debug;

use crate::{ast::*, Span};

pub trait Visitor: Debug {
    type State;

    fn visit_ast(&self, state: &mut Self::State, ast: &Ast) {
        walk_ast(self, state, ast)
    }

    fn visit_block(&self, state: &mut Self::State, block: &Block) {
        walk_block(self, state, block)
    }

    fn visit_raw(&self, _state: &mut Self::State, _raw: &Raw) {}

    fn visit_heading(&self, state: &mut Self::State, heading: &Heading) {
        walk_heading(self, state, heading)
    }

    fn visit_list(&self, state: &mut Self::State, list: &List) {
        walk_list(self, state, list)
    }

    fn visit_list_item(&self, state: &mut Self::State, item: &ListItem) {
        walk_list_item(self, state, item)
    }

    fn visit_enum(&self, state: &mut Self::State, enumeration: &Enum) {
        walk_enum(self, state, enumeration)
    }

    fn visit_enum_item(&self, state: &mut Self::State, item: &EnumItem) {
        walk_enum_item(self, state, item)
    }

    fn visit_table(&self, state: &mut Self::State, table: &Table) {
        walk_table(self, state, table)
    }

    fn visit_table_row(&self, state: &mut Self::State, table_row: &TableRow) {
        walk_table_row(self, state, table_row)
    }

    fn visit_term(&self, state: &mut Self::State, term: &Terms) {
        walk_term(self, state, term)
    }

    fn visit_term_item(&self, state: &mut Self::State, item: &TermItem) {
        walk_term_item(self, state, item)
    }

    fn visit_paragraph(&self, state: &mut Self::State, paragraph: &Paragraph) {
        walk_paragraph(self, state, paragraph)
    }

    fn visit_plain(&self, state: &mut Self::State, plain: &Plain) {
        walk_plain(self, state, plain)
    }

    fn visit_text(&self, state: &mut Self::State, text: &Vec<Inline>) {
        walk_text(self, state, text)
    }

    fn visit_inline(&self, state: &mut Self::State, inline: &Inline) {
        walk_inline(self, state, inline)
    }

    fn visit_quote(&self, state: &mut Self::State, quote: &Quote) {
        walk_quote(self, state, quote)
    }

    fn visit_strikeout(&self, state: &mut Self::State, strikeout: &Strikeout) {
        walk_strikeout(self, state, strikeout)
    }

    fn visit_emphasis(&self, state: &mut Self::State, emphasis: &Emphasis) {
        walk_emphasis(self, state, emphasis)
    }

    fn visit_strong(&self, state: &mut Self::State, strong: &Strong) {
        walk_strong(self, state, strong)
    }

    fn visit_subscript(&self, state: &mut Self::State, subscript: &Subscript) {
        walk_subscript(self, state, subscript)
    }

    fn visit_supscript(&self, state: &mut Self::State, supscript: &Supscript) {
        walk_supscript(self, state, supscript)
    }

    fn visit_link(&self, state: &mut Self::State, link: &Link) {
        walk_link(self, state, link)
    }

    fn visit_cite(&self, _state: &mut Self::State, _cite: &Cite) {}

    fn visit_raw_inline(&self, _state: &mut Self::State, _raw_inline: &RawInline) {}

    fn visit_math_inline(&self, _state: &mut Self::State, _math_inline: &MathInline) {}

    fn visit_comment(&self, _state: &mut Self::State, _comment: &Comment) {}

    fn visit_escape(&self, _state: &mut Self::State, _escape: &Escape) {}

    fn visit_word(&self, _state: &mut Self::State, _word: &Word) {}

    fn visit_spacing(&self, _state: &mut Self::State, _spacing: &Spacing) {}

    fn visit_softbreak(&self, _state: &mut Self::State, _soft_break: &SoftBreak) {}

    fn visit_code(&self, state: &mut Self::State, code: &Code) {
        walk_code(self, state, code)
    }

    fn visit_expr(&self, state: &mut Self::State, expr: &Expr) {
        walk_expr(self, state, expr)
    }

    fn visit_ident_expr(&self, _state: &mut Self::State, _ident: &Ident) {}

    fn visit_literal_expr(&self, _state: &mut Self::State, _literal: &Literal) {}

    fn visit_block_expr(&self, state: &mut Self::State, block: &Vec<Expr>) {
        walk_block_expr(self, state, block)
    }

    fn visit_call_expr(&self, _state: &mut Self::State, _call: &Call) {}

    fn visit_content_expr(&self, _state: &mut Self::State, _content: &Content) {}

    fn visit_error(&self, _state: &mut Self::State, _span: &Span) {}
}

pub fn walk_ast<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, ast: &Ast) {
    for block in &ast.blocks {
        visitor.visit_block(state, block);
    }
}

pub fn walk_block<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, block: &Block) {
    match block {
        Block::Raw(raw) => visitor.visit_raw(state, raw),
        Block::Table(table) => visitor.visit_table(state, table),
        Block::List(list) => visitor.visit_list(state, list),
        Block::Enum(enumeration) => visitor.visit_enum(state, enumeration),
        Block::Terms(block_quote) => visitor.visit_term(state, block_quote),
        Block::Heading(heading) => visitor.visit_heading(state, heading),
        Block::Paragraph(paragraph) => visitor.visit_paragraph(state, paragraph),
        Block::Plain(plain) => visitor.visit_plain(state, plain),
        Block::Error(span) => visitor.visit_error(state, span),
    }
}

pub fn walk_heading<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, heading: &Heading) {
    visitor.visit_text(state, &heading.content)
}

pub fn walk_list<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, list: &List) {
    for item in &list.items {
        visitor.visit_list_item(state, item);
    }
}

pub fn walk_list_item<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, item: &ListItem) {
    for block in &item.content {
        visitor.visit_block(state, block);
    }
}

pub fn walk_enum<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, enumeration: &Enum) {
    for item in &enumeration.items {
        visitor.visit_enum_item(state, item);
    }
}

pub fn walk_enum_item<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, item: &EnumItem) {
    for block in &item.content {
        visitor.visit_block(state, block);
    }
}

pub fn walk_table<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, table: &Table) {
    for row in &table.rows {
        visitor.visit_table_row(state, row);
    }
}

pub fn walk_table_row<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    table_row: &TableRow,
) {
    for cell in &table_row.cells {
        visitor.visit_block(state, cell);
    }
}

pub fn walk_term<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, block_quote: &Terms) {
    for item in &block_quote.content {
        visitor.visit_term_item(state, item);
    }
}

pub fn walk_term_item<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, item: &TermItem) {
    visitor.visit_text(state, &item.term);
    visitor.visit_text(state, &item.content)
}

pub fn walk_paragraph<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    paragraph: &Paragraph,
) {
    for inline in &paragraph.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_plain<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, plain: &Plain) {
    for inline in &plain.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_text<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, text: &Vec<Inline>) {
    for inline in text {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_inline<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, inline: &Inline) {
    match inline {
        Inline::Quote(quote) => visitor.visit_quote(state, quote),
        Inline::Strikeout(strikeout) => visitor.visit_strikeout(state, strikeout),
        Inline::Emphasis(emphasis) => visitor.visit_emphasis(state, emphasis),
        Inline::Strong(strong) => visitor.visit_strong(state, strong),
        Inline::Subscript(sub_script) => visitor.visit_subscript(state, sub_script),
        Inline::Supscript(sup_script) => visitor.visit_supscript(state, sup_script),
        Inline::Link(link) => visitor.visit_link(state, link),
        Inline::Cite(cite) => visitor.visit_cite(state, cite),
        Inline::RawInline(raw_inline) => visitor.visit_raw_inline(state, raw_inline),
        Inline::MathInline(math_inline) => visitor.visit_math_inline(state, math_inline),
        Inline::Comment(comment) => visitor.visit_comment(state, comment),
        Inline::Escape(escape) => visitor.visit_escape(state, escape),
        Inline::Word(word) => visitor.visit_word(state, word),
        Inline::Spacing(spacing) => visitor.visit_spacing(state, spacing),
        Inline::SoftBreak(soft_break) => visitor.visit_softbreak(state, soft_break),
        Inline::Code(code) => visitor.visit_code(state, code),
        Inline::Error(span) => visitor.visit_error(state, span),
    }
}

pub fn walk_quote<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, quote: &Quote) {
    for inline in &quote.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_strikeout<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    strikeout: &Strikeout,
) {
    for inline in &strikeout.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_emphasis<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, emphasis: &Emphasis) {
    for inline in &emphasis.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_strong<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, strong: &Strong) {
    for inline in &strong.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_subscript<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    subscript: &Subscript,
) {
    for inline in &subscript.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_supscript<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    supscript: &Supscript,
) {
    for inline in &supscript.content {
        visitor.visit_inline(state, inline);
    }
}

pub fn walk_link<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, link: &Link) {
    if let Some(content) = &link.content {
        for inline in content {
            visitor.visit_inline(state, inline);
        }
    }
}

pub fn walk_code<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, code: &Code) {
    visitor.visit_expr(state, &code.expr)
}

pub fn walk_expr<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, expr: &Expr) {
    match expr {
        Expr::Block(block, _) => visitor.visit_block_expr(state, block),
        Expr::Call(call) => visitor.visit_call_expr(state, call),
        Expr::Ident(ident) => visitor.visit_ident_expr(state, ident),
        Expr::Literal(literal, _) => visitor.visit_literal_expr(state, literal),
        Expr::Content(content) => visitor.visit_content_expr(state, content),
    }
}

pub fn walk_block_expr<V: Visitor + ?Sized>(visitor: &V, state: &mut V::State, block: &Vec<Expr>) {
    for expr in block {
        visitor.visit_expr(state, expr);
    }
}
