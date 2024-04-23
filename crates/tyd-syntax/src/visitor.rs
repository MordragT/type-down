use miette::Diagnostic;

use crate::ast::*;

pub trait Visitor {
    type Error: Diagnostic;
    type State;

    fn visit_ast(&self, state: &mut Self::State, ast: &Ast) -> Result<(), Self::Error> {
        walk_ast(self, state, ast)
    }

    fn visit_block(&self, state: &mut Self::State, block: &Block) -> Result<(), Self::Error> {
        walk_block(self, state, block)
    }

    fn visit_raw(&self, _state: &mut Self::State, _raw: &Raw) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_heading(&self, state: &mut Self::State, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, state, heading)
    }

    fn visit_list(&self, state: &mut Self::State, list: &List) -> Result<(), Self::Error> {
        walk_list(self, state, list)
    }

    fn visit_list_item(&self, state: &mut Self::State, item: &ListItem) -> Result<(), Self::Error> {
        walk_list_item(self, state, item)
    }

    fn visit_enum(&self, state: &mut Self::State, enumeration: &Enum) -> Result<(), Self::Error> {
        walk_enum(self, state, enumeration)
    }

    fn visit_enum_item(&self, state: &mut Self::State, item: &EnumItem) -> Result<(), Self::Error> {
        walk_enum_item(self, state, item)
    }

    fn visit_table(&self, state: &mut Self::State, table: &Table) -> Result<(), Self::Error> {
        walk_table(self, state, table)
    }

    fn visit_table_row(
        &self,
        state: &mut Self::State,
        table_row: &TableRow,
    ) -> Result<(), Self::Error> {
        walk_table_row(self, state, table_row)
    }

    fn visit_term(&self, state: &mut Self::State, term: &Terms) -> Result<(), Self::Error> {
        walk_term(self, state, term)
    }

    fn visit_term_item(&self, state: &mut Self::State, item: &TermItem) -> Result<(), Self::Error> {
        walk_term_item(self, state, item)
    }

    fn visit_paragraph(
        &self,
        state: &mut Self::State,
        paragraph: &Paragraph,
    ) -> Result<(), Self::Error> {
        walk_paragraph(self, state, paragraph)
    }

    fn visit_plain(&self, state: &mut Self::State, plain: &Plain) -> Result<(), Self::Error> {
        walk_plain(self, state, plain)
    }

    fn visit_text(&self, state: &mut Self::State, text: &Vec<Inline>) -> Result<(), Self::Error> {
        walk_text(self, state, text)
    }

    fn visit_inline(&self, state: &mut Self::State, inline: &Inline) -> Result<(), Self::Error> {
        walk_inline(self, state, inline)
    }

    fn visit_quote(&self, state: &mut Self::State, quote: &Quote) -> Result<(), Self::Error> {
        walk_quote(self, state, quote)
    }

    fn visit_strikeout(
        &self,
        state: &mut Self::State,
        strikeout: &Strikeout,
    ) -> Result<(), Self::Error> {
        walk_strikeout(self, state, strikeout)
    }

    fn visit_emphasis(
        &self,
        state: &mut Self::State,
        emphasis: &Emphasis,
    ) -> Result<(), Self::Error> {
        walk_emphasis(self, state, emphasis)
    }

    fn visit_strong(&self, state: &mut Self::State, strong: &Strong) -> Result<(), Self::Error> {
        walk_strong(self, state, strong)
    }

    fn visit_subscript(
        &self,
        state: &mut Self::State,
        subscript: &Subscript,
    ) -> Result<(), Self::Error> {
        walk_subscript(self, state, subscript)
    }

    fn visit_supscript(
        &self,
        state: &mut Self::State,
        supscript: &Supscript,
    ) -> Result<(), Self::Error> {
        walk_supscript(self, state, supscript)
    }

    fn visit_link(&self, state: &mut Self::State, link: &Link) -> Result<(), Self::Error> {
        walk_link(self, state, link)
    }

    fn visit_cite(&self, _state: &mut Self::State, _cite: &Cite) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_raw_inline(
        &self,
        _state: &mut Self::State,
        _raw_inline: &RawInline,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_math_inline(
        &self,
        _state: &mut Self::State,
        _math_inline: &MathInline,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_comment(
        &self,
        _state: &mut Self::State,
        _comment: &Comment,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_escape(&self, _state: &mut Self::State, _escape: &Escape) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_word(&self, _state: &mut Self::State, _word: &Word) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_spacing(
        &self,
        _state: &mut Self::State,
        _spacing: &Spacing,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_softbreak(
        &self,
        _state: &mut Self::State,
        _soft_break: &SoftBreak,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_code(&self, state: &mut Self::State, code: &Code) -> Result<(), Self::Error> {
        walk_code(self, state, code)
    }

    fn visit_expr(&self, state: &mut Self::State, expr: &Expr) -> Result<(), Self::Error> {
        walk_expr(self, state, expr)
    }

    fn visit_ident_expr(
        &self,
        _state: &mut Self::State,
        _ident: &Ident,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_literal_expr(
        &self,
        _state: &mut Self::State,
        _literal: &Literal,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_block_expr(
        &self,
        state: &mut Self::State,
        block: &Vec<Expr>,
    ) -> Result<(), Self::Error> {
        walk_block_expr(self, state, block)
    }

    fn visit_call_expr(&self, _state: &mut Self::State, _call: &Call) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_content_expr(
        &self,
        _state: &mut Self::State,
        _content: &Content,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn walk_ast<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    ast: &Ast,
) -> Result<(), V::Error> {
    for block in &ast.blocks {
        visitor.visit_block(state, block)?;
    }
    Ok(())
}

pub fn walk_block<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    block: &Block,
) -> Result<(), V::Error> {
    match block {
        Block::Raw(raw) => visitor.visit_raw(state, raw),
        Block::Table(table) => visitor.visit_table(state, table),
        Block::List(list) => visitor.visit_list(state, list),
        Block::Enum(enumeration) => visitor.visit_enum(state, enumeration),
        Block::Terms(block_quote) => visitor.visit_term(state, block_quote),
        Block::Heading(heading) => visitor.visit_heading(state, heading),
        Block::Paragraph(paragraph) => visitor.visit_paragraph(state, paragraph),
        Block::Plain(plain) => visitor.visit_plain(state, plain),
    }
}

pub fn walk_heading<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    heading: &Heading,
) -> Result<(), V::Error> {
    visitor.visit_text(state, &heading.content)
}

pub fn walk_list<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    list: &List,
) -> Result<(), V::Error> {
    for item in &list.items {
        visitor.visit_list_item(state, item)?;
    }

    Ok(())
}

pub fn walk_list_item<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    item: &ListItem,
) -> Result<(), V::Error> {
    for block in &item.content {
        visitor.visit_block(state, block)?;
    }
    Ok(())
}

pub fn walk_enum<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    enumeration: &Enum,
) -> Result<(), V::Error> {
    for item in &enumeration.items {
        visitor.visit_enum_item(state, item)?;
    }

    Ok(())
}

pub fn walk_enum_item<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    item: &EnumItem,
) -> Result<(), V::Error> {
    for block in &item.content {
        visitor.visit_block(state, block)?;
    }
    Ok(())
}

pub fn walk_table<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    table: &Table,
) -> Result<(), V::Error> {
    for row in &table.rows {
        visitor.visit_table_row(state, row)?;
    }

    Ok(())
}

pub fn walk_table_row<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    table_row: &TableRow,
) -> Result<(), V::Error> {
    for cell in &table_row.cells {
        visitor.visit_block(state, cell)?;
    }

    Ok(())
}

pub fn walk_term<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    block_quote: &Terms,
) -> Result<(), V::Error> {
    for item in &block_quote.content {
        visitor.visit_term_item(state, item)?;
    }

    Ok(())
}

pub fn walk_term_item<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    item: &TermItem,
) -> Result<(), V::Error> {
    visitor.visit_text(state, &item.term)?;
    visitor.visit_text(state, &item.content)
}

pub fn walk_paragraph<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    paragraph: &Paragraph,
) -> Result<(), V::Error> {
    for inline in &paragraph.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_plain<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    plain: &Plain,
) -> Result<(), V::Error> {
    for inline in &plain.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_text<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    text: &Vec<Inline>,
) -> Result<(), V::Error> {
    for inline in text {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_inline<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    inline: &Inline,
) -> Result<(), V::Error> {
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
    }
}

pub fn walk_quote<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    quote: &Quote,
) -> Result<(), V::Error> {
    for inline in &quote.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_strikeout<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    strikeout: &Strikeout,
) -> Result<(), V::Error> {
    for inline in &strikeout.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_emphasis<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    emphasis: &Emphasis,
) -> Result<(), V::Error> {
    for inline in &emphasis.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_strong<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    strong: &Strong,
) -> Result<(), V::Error> {
    for inline in &strong.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_subscript<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    subscript: &Subscript,
) -> Result<(), V::Error> {
    for inline in &subscript.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_supscript<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    supscript: &Supscript,
) -> Result<(), V::Error> {
    for inline in &supscript.content {
        visitor.visit_inline(state, inline)?;
    }

    Ok(())
}

pub fn walk_link<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    link: &Link,
) -> Result<(), V::Error> {
    if let Some(content) = &link.content {
        for inline in content {
            visitor.visit_inline(state, inline)?;
        }
    }
    Ok(())
}

pub fn walk_code<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    code: &Code,
) -> Result<(), V::Error> {
    visitor.visit_expr(state, &code.expr)
}

pub fn walk_expr<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    expr: &Expr,
) -> Result<(), V::Error> {
    match expr {
        Expr::Block(block, _) => visitor.visit_block_expr(state, block),
        Expr::Call(call) => visitor.visit_call_expr(state, call),
        Expr::Ident(ident) => visitor.visit_ident_expr(state, ident),
        Expr::Literal(literal, _) => visitor.visit_literal_expr(state, literal),
        Expr::Content(content) => visitor.visit_content_expr(state, content),
    }
}

pub fn walk_block_expr<V: Visitor + ?Sized>(
    visitor: &V,
    state: &mut V::State,
    block: &Vec<Expr>,
) -> Result<(), V::Error> {
    for expr in block {
        visitor.visit_expr(state, expr)?;
    }

    Ok(())
}
