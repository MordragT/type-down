use parasite::chumsky::chain::Chain;

use crate::cst;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ast {
    pub blocks: Vec<Block>,
}

impl From<cst::Cst> for Ast {
    fn from(value: cst::Cst) -> Self {
        let blocks = value
            .0
             .0
            .into_iter()
            .map(|(block, _)| block.into())
            .collect();

        Self { blocks }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Block {
    Mark(MarkBlock),
    // Raw(RawBlock),
    // Code(CodeBlock),
    // Math(MathBlock),
}

impl From<cst::Block> for Block {
    fn from(value: cst::Block) -> Self {
        match value {
            cst::Block::Mark(mark_block) => Self::Mark(mark_block.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkBlock {
    Heading(Heading),
    List(List),
    OrderedList(OrderedList),
    Table(Table),
    Blockquote(Blockquote),
    Paragraph(Paragraph),
}

impl From<cst::MarkBlock> for MarkBlock {
    fn from(value: cst::MarkBlock) -> Self {
        match value {
            cst::MarkBlock::Heading(heading) => Self::Heading(heading.into()),
            cst::MarkBlock::List(list) => Self::List(list.into()),
            cst::MarkBlock::OrderedList(ordered) => Self::OrderedList(ordered.into()),
            cst::MarkBlock::Table(table) => Self::Table(table.into()),
            cst::MarkBlock::Blockquote(blockquote) => Self::Blockquote(blockquote.into()),
            cst::MarkBlock::Paragraph(paragraph) => Self::Paragraph(paragraph.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heading {
    pub level: u8,
    pub line: Line,
}

impl From<cst::Heading> for Heading {
    fn from(value: cst::Heading) -> Self {
        let cst::Heading(level, line) = value;

        let level = level.0.len() as u8;

        Self {
            level,
            line: line.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct List {
    pub lines: Vec<Line>,
}

impl From<cst::List> for List {
    fn from(value: cst::List) -> Self {
        let lines = value
            .0
             .0
            .into_iter()
            .map(|(_, line)| line.into())
            .collect();

        Self { lines }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedList {
    pub lines: Vec<Line>,
}

impl From<cst::OrderedList> for OrderedList {
    fn from(value: cst::OrderedList) -> Self {
        let lines = value
            .0
             .0
            .into_iter()
            .map(|(_, line)| line.into())
            .collect();

        Self { lines }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    pub rows: Vec<TableRow>,
}

impl From<cst::Table> for Table {
    fn from(value: cst::Table) -> Self {
        let rows = value.0 .0.into_iter().map(Into::into).collect();

        Self { rows }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableRow {
    pub elements: Vec<Elements>,
}

impl From<cst::TableRow> for TableRow {
    fn from(value: cst::TableRow) -> Self {
        let elements = value
            .1
             .0
            .into_iter()
            .map(|(elements, _)| elements.into())
            .collect();

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blockquote {
    pub lines: Vec<Line>,
}

impl From<cst::Blockquote> for Blockquote {
    fn from(value: cst::Blockquote) -> Self {
        let lines = value
            .0
             .0
            .into_iter()
            .map(|(_, line)| line.into())
            .collect();

        Self { lines }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paragraph {
    pub lines: Vec<Line>,
}

impl From<cst::Paragraph> for Paragraph {
    fn from(value: cst::Paragraph) -> Self {
        let lines = value.0 .0.into_iter().map(Into::into).collect();

        Self { lines }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub elements: Elements,
    pub label: Option<Label>,
}

impl From<cst::Line> for Line {
    fn from(value: cst::Line) -> Self {
        let cst::Line(elements, label, _) = value;

        let label = label.map(Into::into);

        Self {
            elements: elements.into(),
            label,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(pub String);

impl From<cst::Label> for Label {
    fn from(value: cst::Label) -> Self {
        Label(value.1 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elements(pub Vec<Element>);

impl From<cst::Elements> for Elements {
    fn from(value: cst::Elements) -> Self {
        let elements = value.0 .0 .0.into_iter().map(Into::into).collect();

        Self(elements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    Quote(Quote),
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Link(Link),
    Escape(Escape),
    Monospace(Monospace),
    // Enclosed((LeftBracket, Vec<Element>, RightBracket)),
    Script(Script),
}

impl From<cst::Element> for Element {
    fn from(value: cst::Element) -> Self {
        match value {
            cst::Element::Quote(quote) => Self::Quote(quote.into()),
            cst::Element::Strikethrough(strikethrough) => Self::Strikethrough(strikethrough.into()),
            cst::Element::Emphasis(emphasis) => Self::Emphasis(emphasis.into()),
            cst::Element::Strong(strong) => Self::Strong(strong.into()),
            cst::Element::Link(link) => Self::Link(link.into()),
            cst::Element::Escape(escape) => Self::Escape(escape.into()),
            cst::Element::Monospace(monospace) => Self::Monospace(monospace.into()),
            cst::Element::Script(script) => Self::Script(script.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote {
    pub elements: Vec<QuoteElement>,
}

impl From<cst::Quote> for Quote {
    fn from(value: cst::Quote) -> Self {
        let elements = value.1 .0 .0.into_iter().map(Into::into).collect();

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuoteElement {
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Script(Script),
}

impl From<cst::QuoteElement> for QuoteElement {
    fn from(value: cst::QuoteElement) -> Self {
        match value {
            cst::QuoteElement::Strikethrough(strikethrough) => {
                Self::Strikethrough(strikethrough.into())
            }
            cst::QuoteElement::Emphasis(emphasis) => Self::Emphasis(emphasis.into()),
            cst::QuoteElement::Strong(strong) => Self::Strong(strong.into()),
            cst::QuoteElement::Script(script) => Self::Script(script.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strikethrough {
    pub elements: Vec<StrikethroughElement>,
}

impl From<cst::Strikethrough> for Strikethrough {
    fn from(value: cst::Strikethrough) -> Self {
        let elements = value.1 .0 .0.into_iter().map(Into::into).collect();

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrikethroughElement {
    Emphasis(Emphasis),
    Strong(Strong),
    Script(Script),
}

impl From<cst::StrikethroughElement> for StrikethroughElement {
    fn from(value: cst::StrikethroughElement) -> Self {
        match value {
            cst::StrikethroughElement::Emphasis(emphasis) => Self::Emphasis(emphasis.into()),
            cst::StrikethroughElement::Strong(strong) => Self::Strong(strong.into()),
            cst::StrikethroughElement::Script(script) => Self::Script(script.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Emphasis {
    pub scripts: Vec<Script>,
}

impl From<cst::Emphasis> for Emphasis {
    fn from(value: cst::Emphasis) -> Self {
        let scripts = value.1 .0 .0.into_iter().map(Into::into).collect();

        Self { scripts }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strong {
    pub scripts: Vec<Script>,
}

impl From<cst::Strong> for Strong {
    fn from(value: cst::Strong) -> Self {
        let scripts = value.1 .0 .0.into_iter().map(Into::into).collect();

        Self { scripts }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link(pub String);

impl From<cst::Link> for Link {
    fn from(value: cst::Link) -> Self {
        Link(value.1 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monospace(pub String);

impl From<cst::Monospace> for Monospace {
    fn from(value: cst::Monospace) -> Self {
        Monospace(value.1 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Escape(pub char);

impl From<cst::Escape> for Escape {
    fn from(value: cst::Escape) -> Self {
        Escape(value.1 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Script(pub String, pub ScriptTail);

impl From<cst::Script> for Script {
    fn from(value: cst::Script) -> Self {
        let cst::Script(word, tail) = value;

        Self(word.0, tail.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptTail {
    Sub(char, Box<Script>),
    Sup(char, Box<Script>),
    None,
}

impl From<Option<cst::ScriptTail>> for ScriptTail {
    fn from(value: Option<cst::ScriptTail>) -> Self {
        match value {
            Some(cst::ScriptTail::Sup(_, c, script)) => {
                ScriptTail::Sup(c.0, Box::new((*script.0).into()))
            }
            Some(cst::ScriptTail::Sub(_, c, script)) => {
                ScriptTail::Sub(c.0, Box::new((*script.0).into()))
            }
            None => ScriptTail::None,
        }
    }
}
