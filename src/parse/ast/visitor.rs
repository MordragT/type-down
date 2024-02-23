use super::*;

pub trait Visitor {
    fn visit_ast(&mut self, ast: &Ast) {
        walk_ast(self, ast)
    }

    fn visit_block(&mut self, block: &Block) {
        walk_block(self, block)
    }

    fn visit_raw(&mut self, _raw: &Raw) {}

    fn visit_heading(&mut self, heading: &Heading) {
        walk_heading(self, heading)
    }

    fn visit_list(&mut self, list: &List) {
        walk_list(self, list)
    }

    fn visit_ordered_list(&mut self, ordered_list: &OrderedList) {
        walk_ordered_list(self, ordered_list)
    }

    fn visit_table(&mut self, table: &Table) {
        walk_table(self, table)
    }

    fn visit_table_row(&mut self, table_row: &TableRow) {
        walk_table_row(self, table_row)
    }

    fn visit_blockquote(&mut self, blockquote: &Blockquote) {
        walk_blockquote(self, blockquote)
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) {
        walk_paragraph(self, paragraph)
    }

    fn visit_line(&mut self, line: &Line) {
        walk_line(self, line)
    }

    fn visit_label(&mut self, _label: &Label) {}

    fn visit_elements(&mut self, elements: &Elements) {
        walk_elements(self, elements)
    }

    fn visit_element(&mut self, element: &Element) {
        walk_element(self, element)
    }

    fn visit_inline(&mut self, inline: &Inline) {
        walk_inline(self, inline)
    }

    fn visit_quote(&mut self, quote: &Quote) {
        walk_quote(self, quote)
    }

    fn visit_strikethrough(&mut self, strikethrough: &Strikethrough) {
        walk_strikethrough(self, strikethrough)
    }

    fn visit_strong(&mut self, strong: &Strong) {
        walk_strong(self, strong)
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) {
        walk_emphasis(self, emphasis)
    }

    fn visit_enclosed(&mut self, enclosed: &Enclosed) {
        walk_enclosed(self, enclosed)
    }

    fn visit_link(&mut self, link: &Link) {
        walk_link(self, link)
    }

    fn visit_escape(&mut self, _escape: &Escape) {}

    fn visit_monospace(&mut self, _monospace: &Monospace) {}

    fn visit_sub_script(&mut self, _sub_script: &SubScript) {}

    fn visit_sup_script(&mut self, _sup_script: &SupScript) {}

    fn visit_word(&mut self, _word: &Word) {}

    fn visit_spacing(&mut self, _spacing: &Spacing) {}

    fn visit_access(&mut self, access: &Access) {
        walk_access(self, access)
    }

    fn visit_call_tail(&mut self, call_tail: &CallTail) {
        walk_call_tail(self, call_tail)
    }
}

pub fn walk_ast<V: Visitor + ?Sized>(visitor: &mut V, ast: &Ast) {
    for block in &ast.blocks {
        visitor.visit_block(block)
    }
}

pub fn walk_block<V: Visitor + ?Sized>(visitor: &mut V, block: &Block) {
    match block {
        Block::Raw(raw) => visitor.visit_raw(raw),
        Block::Heading(heading) => visitor.visit_heading(heading),
        Block::List(list) => visitor.visit_list(list),
        Block::OrderedList(ordered_list) => visitor.visit_ordered_list(ordered_list),
        Block::Table(table) => visitor.visit_table(table),
        Block::Blockquote(blockquote) => visitor.visit_blockquote(blockquote),
        Block::Paragraph(paragraph) => visitor.visit_paragraph(paragraph),
    }
}

pub fn walk_heading<V: Visitor + ?Sized>(visitor: &mut V, heading: &Heading) {
    visitor.visit_line(&heading.line)
}

pub fn walk_list<V: Visitor + ?Sized>(visitor: &mut V, list: &List) {
    for line in &list.lines {
        visitor.visit_line(line)
    }
}

pub fn walk_ordered_list<V: Visitor + ?Sized>(visitor: &mut V, ordered_list: &OrderedList) {
    for line in &ordered_list.lines {
        visitor.visit_line(line)
    }
}

pub fn walk_table<V: Visitor + ?Sized>(visitor: &mut V, table: &Table) {
    for row in &table.rows {
        visitor.visit_table_row(row)
    }
}

pub fn walk_table_row<V: Visitor + ?Sized>(visitor: &mut V, table_row: &TableRow) {
    for cell in &table_row.cells {
        visitor.visit_elements(cell)
    }
}

pub fn walk_blockquote<V: Visitor + ?Sized>(visitor: &mut V, blockquote: &Blockquote) {
    for line in &blockquote.lines {
        visitor.visit_line(line)
    }
}

pub fn walk_paragraph<V: Visitor + ?Sized>(visitor: &mut V, paragraph: &Paragraph) {
    for line in &paragraph.lines {
        visitor.visit_line(line)
    }
}

pub fn walk_line<V: Visitor + ?Sized>(visitor: &mut V, line: &Line) {
    visitor.visit_elements(&line.elements);
    if let Some(label) = &line.label {
        visitor.visit_label(label)
    }
}

pub fn walk_elements<V: Visitor + ?Sized>(visitor: &mut V, elements: &Elements) {
    for el in &elements.0 {
        visitor.visit_element(el)
    }
}

pub fn walk_element<V: Visitor + ?Sized>(visitor: &mut V, element: &Element) {
    match element {
        Element::Inline(inline) => visitor.visit_inline(inline),
        Element::Quote(quote) => visitor.visit_quote(quote),
        Element::Strikethrough(strikethrough) => visitor.visit_strikethrough(strikethrough),
        Element::Emphasis(emphasis) => visitor.visit_emphasis(emphasis),
        Element::Strong(strong) => visitor.visit_strong(strong),
        Element::Enclosed(enclosed) => visitor.visit_enclosed(enclosed),
        Element::Link(link) => visitor.visit_link(link),
        Element::Escape(escape) => visitor.visit_escape(escape),
        Element::Monospace(monospace) => visitor.visit_monospace(monospace),
        Element::Access(access) => visitor.visit_access(access),
    }
}

pub fn walk_inline<V: Visitor + ?Sized>(visitor: &mut V, inline: &Inline) {
    match inline {
        Inline::SubScript(sub_script) => visitor.visit_sub_script(sub_script),
        Inline::SupScript(sup_script) => visitor.visit_sup_script(sup_script),
        Inline::Word(word) => visitor.visit_word(word),
        Inline::Spacing(spacing) => visitor.visit_spacing(spacing),
    }
}

pub fn walk_quote<V: Visitor + ?Sized>(visitor: &mut V, quote: &Quote) {
    visitor.visit_elements(&quote.elements)
}

pub fn walk_strikethrough<V: Visitor + ?Sized>(visitor: &mut V, strikethrough: &Strikethrough) {
    visitor.visit_elements(&strikethrough.elements)
}

pub fn walk_strong<V: Visitor + ?Sized>(visitor: &mut V, strong: &Strong) {
    for inline in &strong.inlines {
        visitor.visit_inline(inline)
    }
}

pub fn walk_emphasis<V: Visitor + ?Sized>(visitor: &mut V, emphasis: &Emphasis) {
    for inline in &emphasis.inlines {
        visitor.visit_inline(inline)
    }
}

pub fn walk_enclosed<V: Visitor + ?Sized>(visitor: &mut V, enclosed: &Enclosed) {
    visitor.visit_elements(&enclosed.elements)
}

pub fn walk_link<V: Visitor + ?Sized>(visitor: &mut V, link: &Link) {
    if let Some(elements) = &link.elements {
        visitor.visit_elements(elements)
    }
}

pub fn walk_access<V: Visitor + ?Sized>(visitor: &mut V, access: &Access) {
    if let Some(tail) = &access.tail {
        visitor.visit_call_tail(tail)
    }
}

pub fn walk_call_tail<V: Visitor + ?Sized>(visitor: &mut V, call_tail: &CallTail) {
    if let Some(enclosed) = &call_tail.content {
        visitor.visit_enclosed(enclosed)
    }
}
