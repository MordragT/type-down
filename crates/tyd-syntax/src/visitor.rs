use miette::Diagnostic;

use crate::ast::*;

pub trait Visitor {
    type Error: Diagnostic;

    fn visit_ast(&mut self, ast: &Ast) -> Result<(), Self::Error> {
        walk_ast(self, ast)
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), Self::Error> {
        walk_block(self, block)
    }

    fn visit_raw(&mut self, _raw: &Raw) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_div(&mut self, div: &Div) -> Result<(), Self::Error> {
        walk_div(self, div)
    }

    fn visit_heading(&mut self, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, heading)
    }

    fn visit_list(&mut self, list: &List) -> Result<(), Self::Error> {
        walk_list(self, list)
    }

    fn visit_list_item(&mut self, item: &ListItem) -> Result<(), Self::Error> {
        walk_list_item(self, item)
    }

    fn visit_enum(&mut self, enumeration: &Enum) -> Result<(), Self::Error> {
        walk_enum(self, enumeration)
    }

    fn visit_enum_item(&mut self, item: &EnumItem) -> Result<(), Self::Error> {
        walk_enum_item(self, item)
    }

    fn visit_table(&mut self, table: &Table) -> Result<(), Self::Error> {
        walk_table(self, table)
    }

    fn visit_table_row(&mut self, table_row: &TableRow) -> Result<(), Self::Error> {
        walk_table_row(self, table_row)
    }

    fn visit_term(&mut self, term: &Term) -> Result<(), Self::Error> {
        walk_term(self, term)
    }

    fn visit_term_item(&mut self, item: &TermItem) -> Result<(), Self::Error> {
        walk_term_item(self, item)
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<(), Self::Error> {
        walk_paragraph(self, paragraph)
    }

    fn visit_plain(&mut self, plain: &Plain) -> Result<(), Self::Error> {
        walk_plain(self, plain)
    }

    fn visit_text(&mut self, text: &Text) -> Result<(), Self::Error> {
        walk_text(self, text)
    }

    fn visit_inline(&mut self, inline: &Inline) -> Result<(), Self::Error> {
        walk_inline(self, inline)
    }

    fn visit_quote(&mut self, quote: &Quote) -> Result<(), Self::Error> {
        walk_quote(self, quote)
    }

    fn visit_strikeout(&mut self, strikeout: &Strikeout) -> Result<(), Self::Error> {
        walk_strikeout(self, strikeout)
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> Result<(), Self::Error> {
        walk_emphasis(self, emphasis)
    }

    fn visit_strong(&mut self, strong: &Strong) -> Result<(), Self::Error> {
        walk_strong(self, strong)
    }

    fn visit_subscript(&mut self, subscript: &Subscript) -> Result<(), Self::Error> {
        walk_subscript(self, subscript)
    }

    fn visit_supscript(&mut self, supscript: &Supscript) -> Result<(), Self::Error> {
        walk_supscript(self, supscript)
    }

    fn visit_link(&mut self, link: &Link) -> Result<(), Self::Error> {
        walk_link(self, link)
    }

    fn visit_cite(&mut self, _cite: &Cite) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_raw_inline(&mut self, _raw_inline: &RawInline) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_comment(&mut self, _comment: &Comment) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_escape(&mut self, _escape: &Escape) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_word(&mut self, _word: &Word) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_spacing(&mut self, _spacing: &Spacing) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_softbreak(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_code(&mut self, code: &Code) -> Result<(), Self::Error> {
        walk_code(self, code)
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<(), Self::Error> {
        walk_expr(self, expr)
    }

    fn visit_ident_expr(&mut self, _ident: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_literal_expr(&mut self, _literal: &Literal) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_block_expr(&mut self, block: &Vec<Expr>) -> Result<(), Self::Error> {
        walk_block_expr(self, block)
    }

    fn visit_call_expr(&mut self, _call: &Call) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn walk_ast<V: Visitor + ?Sized>(visitor: &mut V, ast: &Ast) -> Result<(), V::Error> {
    for block in &ast.blocks {
        visitor.visit_block(block)?;
    }
    Ok(())
}

pub fn walk_block<V: Visitor + ?Sized>(visitor: &mut V, block: &Block) -> Result<(), V::Error> {
    match block {
        Block::Div(div) => visitor.visit_div(div),
        Block::Raw(raw) => visitor.visit_raw(raw),
        Block::Table(table) => visitor.visit_table(table),
        Block::List(list) => visitor.visit_list(list),
        Block::Enum(enumeration) => visitor.visit_enum(enumeration),
        Block::Term(block_quote) => visitor.visit_term(block_quote),
        Block::Heading(heading) => visitor.visit_heading(heading),
        Block::Paragraph(paragraph) => visitor.visit_paragraph(paragraph),
        Block::Plain(plain) => visitor.visit_plain(plain),
    }
}

pub fn walk_div<V: Visitor + ?Sized>(visitor: &mut V, div: &Div) -> Result<(), V::Error> {
    for block in &div.content {
        visitor.visit_block(block)?;
    }

    Ok(())
}

pub fn walk_heading<V: Visitor + ?Sized>(
    visitor: &mut V,
    heading: &Heading,
) -> Result<(), V::Error> {
    visitor.visit_text(&heading.content)
}

pub fn walk_list<V: Visitor + ?Sized>(visitor: &mut V, list: &List) -> Result<(), V::Error> {
    for item in &list.items {
        visitor.visit_list_item(item)?;
    }

    Ok(())
}

pub fn walk_list_item<V: Visitor + ?Sized>(
    visitor: &mut V,
    item: &ListItem,
) -> Result<(), V::Error> {
    for block in &item.content {
        visitor.visit_block(block)?;
    }
    Ok(())
}

pub fn walk_enum<V: Visitor + ?Sized>(visitor: &mut V, enumeration: &Enum) -> Result<(), V::Error> {
    for item in &enumeration.items {
        visitor.visit_enum_item(item)?;
    }

    Ok(())
}

pub fn walk_enum_item<V: Visitor + ?Sized>(
    visitor: &mut V,
    item: &EnumItem,
) -> Result<(), V::Error> {
    for block in &item.content {
        visitor.visit_block(block)?;
    }
    Ok(())
}

pub fn walk_table<V: Visitor + ?Sized>(visitor: &mut V, table: &Table) -> Result<(), V::Error> {
    for row in &table.rows {
        visitor.visit_table_row(row)?;
    }

    Ok(())
}

pub fn walk_table_row<V: Visitor + ?Sized>(
    visitor: &mut V,
    table_row: &TableRow,
) -> Result<(), V::Error> {
    for cell in &table_row.cells {
        visitor.visit_block(cell)?;
    }

    Ok(())
}

pub fn walk_term<V: Visitor + ?Sized>(visitor: &mut V, block_quote: &Term) -> Result<(), V::Error> {
    for item in &block_quote.content {
        visitor.visit_term_item(item)?;
    }

    Ok(())
}

pub fn walk_term_item<V: Visitor + ?Sized>(
    visitor: &mut V,
    item: &TermItem,
) -> Result<(), V::Error> {
    visitor.visit_text(&item.term)?;
    visitor.visit_text(&item.content)
}

pub fn walk_paragraph<V: Visitor + ?Sized>(
    visitor: &mut V,
    paragraph: &Paragraph,
) -> Result<(), V::Error> {
    for inline in &paragraph.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_plain<V: Visitor + ?Sized>(visitor: &mut V, plain: &Plain) -> Result<(), V::Error> {
    for inline in &plain.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_text<V: Visitor + ?Sized>(visitor: &mut V, text: &Text) -> Result<(), V::Error> {
    for inline in &text.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_inline<V: Visitor + ?Sized>(visitor: &mut V, inline: &Inline) -> Result<(), V::Error> {
    match inline {
        Inline::Quote(quote) => visitor.visit_quote(quote),
        Inline::Strikeout(strikeout) => visitor.visit_strikeout(strikeout),
        Inline::Emphasis(emphasis) => visitor.visit_emphasis(emphasis),
        Inline::Strong(strong) => visitor.visit_strong(strong),
        Inline::Subscript(sub_script) => visitor.visit_subscript(sub_script),
        Inline::Supscript(sup_script) => visitor.visit_supscript(sup_script),
        Inline::Link(link) => visitor.visit_link(link),
        Inline::Cite(cite) => visitor.visit_cite(cite),
        Inline::RawInline(raw_inline) => visitor.visit_raw_inline(raw_inline),
        Inline::Comment(comment) => visitor.visit_comment(comment),
        Inline::Escape(escape) => visitor.visit_escape(escape),
        Inline::Word(word) => visitor.visit_word(word),
        Inline::Spacing(spacing) => visitor.visit_spacing(spacing),
        Inline::SoftBreak => visitor.visit_softbreak(),
        Inline::Code(code) => visitor.visit_code(code),
    }
}

pub fn walk_quote<V: Visitor + ?Sized>(visitor: &mut V, quote: &Quote) -> Result<(), V::Error> {
    for inline in &quote.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_strikeout<V: Visitor + ?Sized>(
    visitor: &mut V,
    strikeout: &Strikeout,
) -> Result<(), V::Error> {
    for inline in &strikeout.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_emphasis<V: Visitor + ?Sized>(
    visitor: &mut V,
    emphasis: &Emphasis,
) -> Result<(), V::Error> {
    for inline in &emphasis.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_strong<V: Visitor + ?Sized>(visitor: &mut V, strong: &Strong) -> Result<(), V::Error> {
    for inline in &strong.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_subscript<V: Visitor + ?Sized>(
    visitor: &mut V,
    subscript: &Subscript,
) -> Result<(), V::Error> {
    for inline in &subscript.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_supscript<V: Visitor + ?Sized>(
    visitor: &mut V,
    supscript: &Supscript,
) -> Result<(), V::Error> {
    for inline in &supscript.content {
        visitor.visit_inline(inline)?;
    }

    Ok(())
}

pub fn walk_link<V: Visitor + ?Sized>(visitor: &mut V, link: &Link) -> Result<(), V::Error> {
    if let Some(content) = &link.content {
        for inline in content {
            visitor.visit_inline(inline)?;
        }
    }
    Ok(())
}

pub fn walk_code<V: Visitor + ?Sized>(visitor: &mut V, code: &Code) -> Result<(), V::Error> {
    visitor.visit_expr(&code.expr)
}

pub fn walk_expr<V: Visitor + ?Sized>(visitor: &mut V, expr: &Expr) -> Result<(), V::Error> {
    match expr {
        Expr::Block(block) => visitor.visit_block_expr(block),
        Expr::Call(call) => visitor.visit_call_expr(call),
        Expr::Ident(ident) => visitor.visit_ident_expr(ident),
        Expr::Literal(literal) => visitor.visit_literal_expr(literal),
    }
}

pub fn walk_block_expr<V: Visitor + ?Sized>(
    visitor: &mut V,
    block: &Vec<Expr>,
) -> Result<(), V::Error> {
    for expr in block {
        visitor.visit_expr(expr)?;
    }

    Ok(())
}
