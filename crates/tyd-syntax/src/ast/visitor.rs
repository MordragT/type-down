use miette::Diagnostic;

use super::*;

pub trait Visitor {
    type Error: Diagnostic;

    fn visit_ast(&mut self, ast: &Ast) -> Result<(), Self::Error> {
        walk_ast(self, ast)
    }

    fn visit_blocks(&mut self, blocks: &Blocks) -> Result<(), Self::Error> {
        walk_blocks(self, blocks)
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), Self::Error> {
        walk_block(self, block)
    }

    fn visit_raw(&mut self, _raw: &Raw) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_heading(&mut self, heading: &Heading) -> Result<(), Self::Error> {
        walk_heading(self, heading)
    }

    fn visit_bullet_list(&mut self, list: &BulletList) -> Result<(), Self::Error> {
        walk_bullet_list(self, list)
    }

    fn visit_ordered_list(&mut self, ordered_list: &OrderedList) -> Result<(), Self::Error> {
        walk_ordered_list(self, ordered_list)
    }

    fn visit_table(&mut self, table: &Table) -> Result<(), Self::Error> {
        walk_table(self, table)
    }

    fn visit_table_row(&mut self, table_row: &TableRow) -> Result<(), Self::Error> {
        walk_table_row(self, table_row)
    }

    fn visit_block_quote(&mut self, block_quote: &BlockQuote) -> Result<(), Self::Error> {
        walk_block_quote(self, block_quote)
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<(), Self::Error> {
        walk_paragraph(self, paragraph)
    }

    fn visit_image(&mut self, _image: &Image) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_line(&mut self, line: &Line) -> Result<(), Self::Error> {
        walk_line(self, line)
    }

    fn visit_label(&mut self, _label: &Label) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_elements(&mut self, elements: &Elements) -> Result<(), Self::Error> {
        walk_elements(self, elements)
    }

    fn visit_element(&mut self, element: &Element) -> Result<(), Self::Error> {
        walk_element(self, element)
    }

    fn visit_quote(&mut self, quote: &Quote) -> Result<(), Self::Error> {
        walk_quote(self, quote)
    }

    fn visit_strikeout(&mut self, strikeout: &Strikeout) -> Result<(), Self::Error> {
        walk_strikeout(self, strikeout)
    }

    fn visit_strong(&mut self, strong: &Strong) -> Result<(), Self::Error> {
        walk_strong(self, strong)
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> Result<(), Self::Error> {
        walk_emphasis(self, emphasis)
    }

    fn visit_enclosed(&mut self, enclosed: &Enclosed) -> Result<(), Self::Error> {
        walk_enclosed(self, enclosed)
    }

    fn visit_link(&mut self, link: &Link) -> Result<(), Self::Error> {
        walk_link(self, link)
    }

    fn visit_escape(&mut self, _escape: &Escape) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_raw_inline(&mut self, _raw_inline: &RawInline) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_sub_script(&mut self, _sub_script: &SubScript) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_sup_script(&mut self, _sup_script: &SupScript) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_word(&mut self, _word: &Word) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_spacing(&mut self, _spacing: &Spacing) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_access(&mut self, access: &Access) -> Result<(), Self::Error> {
        walk_access(self, access)
    }

    fn visit_call_tail(&mut self, call_tail: &CallTail) -> Result<(), Self::Error> {
        walk_call_tail(self, call_tail)
    }
}

pub fn walk_ast<V: Visitor + ?Sized>(visitor: &mut V, ast: &Ast) -> Result<(), V::Error> {
    visitor.visit_blocks(&ast.blocks)
}

pub fn walk_blocks<V: Visitor + ?Sized>(visitor: &mut V, blocks: &Blocks) -> Result<(), V::Error> {
    for block in &blocks.0 {
        visitor.visit_block(block)?;
    }
    Ok(())
}

pub fn walk_block<V: Visitor + ?Sized>(visitor: &mut V, block: &Block) -> Result<(), V::Error> {
    match block {
        Block::Raw(raw) => visitor.visit_raw(raw),
        Block::Heading(heading) => visitor.visit_heading(heading),
        Block::BulletList(bullet) => visitor.visit_bullet_list(bullet),
        Block::OrderedList(ordered_list) => visitor.visit_ordered_list(ordered_list),
        Block::Table(table) => visitor.visit_table(table),
        Block::BlockQuote(block_quote) => visitor.visit_block_quote(block_quote),
        Block::Paragraph(paragraph) => visitor.visit_paragraph(paragraph),
        Block::Image(image) => visitor.visit_image(image),
    }
}

pub fn walk_heading<V: Visitor + ?Sized>(
    visitor: &mut V,
    heading: &Heading,
) -> Result<(), V::Error> {
    visitor.visit_line(&heading.line)
}

pub fn walk_bullet_list<V: Visitor + ?Sized>(
    visitor: &mut V,
    list: &BulletList,
) -> Result<(), V::Error> {
    for line in &list.lines {
        visitor.visit_line(line)?;
    }
    Ok(())
}

pub fn walk_ordered_list<V: Visitor + ?Sized>(
    visitor: &mut V,
    ordered_list: &OrderedList,
) -> Result<(), V::Error> {
    for line in &ordered_list.lines {
        visitor.visit_line(line)?;
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
        visitor.visit_elements(cell)?;
    }

    Ok(())
}

pub fn walk_block_quote<V: Visitor + ?Sized>(
    visitor: &mut V,
    block_quote: &BlockQuote,
) -> Result<(), V::Error> {
    for line in &block_quote.lines {
        visitor.visit_line(line)?;
    }

    Ok(())
}

pub fn walk_paragraph<V: Visitor + ?Sized>(
    visitor: &mut V,
    paragraph: &Paragraph,
) -> Result<(), V::Error> {
    for line in &paragraph.lines {
        visitor.visit_line(line)?;
    }

    Ok(())
}

pub fn walk_line<V: Visitor + ?Sized>(visitor: &mut V, line: &Line) -> Result<(), V::Error> {
    visitor.visit_elements(&line.elements)?;
    if let Some(label) = &line.label {
        visitor.visit_label(label)?;
    }

    Ok(())
}

pub fn walk_elements<V: Visitor + ?Sized>(
    visitor: &mut V,
    elements: &Elements,
) -> Result<(), V::Error> {
    for el in &elements.0 {
        visitor.visit_element(el)?;
    }

    Ok(())
}

pub fn walk_element<V: Visitor + ?Sized>(
    visitor: &mut V,
    element: &Element,
) -> Result<(), V::Error> {
    match element {
        Element::Access(access) => visitor.visit_access(access),
        Element::Quote(quote) => visitor.visit_quote(quote),
        Element::Strikeout(strikeout) => visitor.visit_strikeout(strikeout),
        Element::Emphasis(emphasis) => visitor.visit_emphasis(emphasis),
        Element::Strong(strong) => visitor.visit_strong(strong),
        Element::Enclosed(enclosed) => visitor.visit_enclosed(enclosed),
        Element::Link(link) => visitor.visit_link(link),
        Element::Escape(escape) => visitor.visit_escape(escape),
        Element::RawInline(raw_inline) => visitor.visit_raw_inline(raw_inline),
        Element::SubScript(sub_script) => visitor.visit_sub_script(sub_script),
        Element::SupScript(sup_script) => visitor.visit_sup_script(sup_script),
        Element::Word(word) => visitor.visit_word(word),
        Element::Spacing(spacing) => visitor.visit_spacing(spacing),
    }
}

pub fn walk_quote<V: Visitor + ?Sized>(visitor: &mut V, quote: &Quote) -> Result<(), V::Error> {
    visitor.visit_elements(&quote.elements)
}

pub fn walk_strikeout<V: Visitor + ?Sized>(
    visitor: &mut V,
    strikeout: &Strikeout,
) -> Result<(), V::Error> {
    visitor.visit_elements(&strikeout.elements)
}

pub fn walk_strong<V: Visitor + ?Sized>(visitor: &mut V, strong: &Strong) -> Result<(), V::Error> {
    visitor.visit_elements(&strong.elements)
}

pub fn walk_emphasis<V: Visitor + ?Sized>(
    visitor: &mut V,
    emphasis: &Emphasis,
) -> Result<(), V::Error> {
    visitor.visit_elements(&emphasis.elements)
}

pub fn walk_enclosed<V: Visitor + ?Sized>(
    visitor: &mut V,
    enclosed: &Enclosed,
) -> Result<(), V::Error> {
    visitor.visit_elements(&enclosed.elements)
}

pub fn walk_link<V: Visitor + ?Sized>(visitor: &mut V, link: &Link) -> Result<(), V::Error> {
    if let Some(elements) = &link.elements {
        visitor.visit_elements(elements)?;
    }

    Ok(())
}

pub fn walk_access<V: Visitor + ?Sized>(visitor: &mut V, access: &Access) -> Result<(), V::Error> {
    if let Some(tail) = &access.tail {
        visitor.visit_call_tail(tail)?;
    }

    Ok(())
}

pub fn walk_call_tail<V: Visitor + ?Sized>(
    visitor: &mut V,
    call_tail: &CallTail,
) -> Result<(), V::Error> {
    if let Some(enclosed) = &call_tail.content {
        visitor.visit_enclosed(enclosed)?;
    }

    Ok(())
}
