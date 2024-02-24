use parasite::chumsky::chain::Chain;
use std::collections::BTreeMap;

use super::cst;

pub mod visitor;

pub use cst::terminal::Word;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ast {
    pub blocks: Blocks,
}

impl From<cst::Cst> for Ast {
    fn from(value: cst::Cst) -> Self {
        let blocks = Blocks(
            value
                .0
                 .0
                .into_iter()
                .map(|(block, _)| block.into())
                .collect(),
        );

        Self { blocks }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blocks(pub Vec<Block>);

// TODO merge Block and MarkBlock to one

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Block {
    Raw(Raw),
    Heading(Heading),
    BulletList(BulletList),
    OrderedList(OrderedList),
    Table(Table),
    BlockQuote(BlockQuote),
    Paragraph(Paragraph),
    // Expr(Expr),
    // Math(Math),
}

impl From<cst::Block> for Block {
    fn from(value: cst::Block) -> Self {
        match value {
            cst::Block::Raw(raw_block) => Self::Raw(raw_block.into()),
            cst::Block::Heading(heading) => Self::Heading(heading.into()),
            cst::Block::BulletList(bullet) => Self::BulletList(bullet.into()),
            cst::Block::OrderedList(ordered) => Self::OrderedList(ordered.into()),
            cst::Block::Table(table) => Self::Table(table.into()),
            cst::Block::BlockQuote(block_quote) => Self::BlockQuote(block_quote.into()),
            cst::Block::Paragraph(paragraph) => Self::Paragraph(paragraph.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Raw {
    pub lang: Option<String>,
    pub content: String,
}

impl From<cst::Raw> for Raw {
    fn from(value: cst::Raw) -> Self {
        let cst::Raw(_, lang, _, content, _, _) = value;

        let lang = lang.map(|lang| lang.0 .0);

        Self {
            lang,
            content: content.0,
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
pub struct BulletList {
    pub lines: Vec<Line>,
}

impl From<cst::BulletList> for BulletList {
    fn from(value: cst::BulletList) -> Self {
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
    pub cells: Vec<Elements>,
}

impl From<cst::TableRow> for TableRow {
    fn from(value: cst::TableRow) -> Self {
        let cells = value
            .1
             .0
            .into_iter()
            .map(|(elements, _)| elements.into())
            .collect();

        Self { cells }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockQuote {
    pub lines: Vec<Line>,
}

impl From<cst::BlockQuote> for BlockQuote {
    fn from(value: cst::BlockQuote) -> Self {
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
        let elements = value.0 .0.into_iter().map(Into::into).collect();

        Self(elements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    Access(Access),
    Quote(Quote),
    Strikeout(Strikeout),
    Emphasis(Emphasis),
    Strong(Strong),
    Enclosed(Enclosed),
    Link(Link),
    Escape(Escape),
    RawInline(RawInline),
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
}

impl From<cst::Element> for Element {
    fn from(value: cst::Element) -> Self {
        match value {
            cst::Element::Access(access) => Self::Access(access.into()),
            cst::Element::Quote(quote) => Self::Quote(quote.into()),
            cst::Element::Strikeout(strikeout) => Self::Strikeout(strikeout.into()),
            cst::Element::Emphasis(emphasis) => Self::Emphasis(emphasis.into()),
            cst::Element::Strong(strong) => Self::Strong(strong.into()),
            cst::Element::Link(link) => Self::Link(link.into()),
            cst::Element::Enclosed(enclosed) => Self::Enclosed(enclosed.into()),
            cst::Element::Escape(escape) => Self::Escape(escape.into()),
            cst::Element::RawInline(raw_inline) => Self::RawInline(raw_inline.into()),
            cst::Element::SubScript(script) => Self::SubScript(script.into()),
            cst::Element::SupScript(script) => Self::SupScript(script.into()),
            cst::Element::Spacing(spacing) => Self::Spacing(spacing.into()),
            cst::Element::Word(word) => Self::Word(word),
        }
    }
}

impl From<cst::StrikeoutElement> for Element {
    fn from(value: cst::StrikeoutElement) -> Self {
        match value {
            cst::StrikeoutElement::Access(access) => Self::Access(access.into()),
            cst::StrikeoutElement::Emphasis(emphasis) => Self::Emphasis(emphasis.into()),
            cst::StrikeoutElement::Strong(strong) => Self::Strong(strong.into()),
            cst::StrikeoutElement::SubScript(script) => Self::SubScript(script.into()),
            cst::StrikeoutElement::SupScript(script) => Self::SupScript(script.into()),
            cst::StrikeoutElement::Spacing(spacing) => Self::Spacing(spacing.into()),
            cst::StrikeoutElement::Word(word) => Self::Word(word),
        }
    }
}

impl From<cst::QuoteElement> for Element {
    fn from(value: cst::QuoteElement) -> Self {
        match value {
            cst::QuoteElement::Access(access) => Self::Access(access.into()),
            cst::QuoteElement::Strikeout(strikeout) => Self::Strikeout(strikeout.into()),
            cst::QuoteElement::Emphasis(emphasis) => Self::Emphasis(emphasis.into()),
            cst::QuoteElement::Strong(strong) => Self::Strong(strong.into()),
            cst::QuoteElement::SubScript(script) => Self::SubScript(script.into()),
            cst::QuoteElement::SupScript(script) => Self::SupScript(script.into()),
            cst::QuoteElement::Spacing(spacing) => Self::Spacing(spacing.into()),
            cst::QuoteElement::Word(word) => Self::Word(word),
        }
    }
}

impl From<cst::EmphasizedElement> for Element {
    fn from(value: cst::EmphasizedElement) -> Self {
        match value {
            cst::EmphasizedElement::Access(access) => Self::Access(access.into()),
            cst::EmphasizedElement::SubScript(script) => Self::SubScript(script.into()),
            cst::EmphasizedElement::SupScript(script) => Self::SupScript(script.into()),
            cst::EmphasizedElement::Spacing(spacing) => Self::Spacing(spacing.into()),
            cst::EmphasizedElement::Word(word) => Self::Word(word),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubScript(pub char);

impl From<cst::SubScript> for SubScript {
    fn from(value: cst::SubScript) -> Self {
        Self(value.1 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SupScript(pub char);

impl From<cst::SupScript> for SupScript {
    fn from(value: cst::SupScript) -> Self {
        Self(value.1 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spacing(pub usize);

impl From<cst::Spacing> for Spacing {
    fn from(value: cst::Spacing) -> Self {
        Self(value.0 .0.len())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote {
    pub elements: Elements,
}

impl From<cst::Quote> for Quote {
    fn from(value: cst::Quote) -> Self {
        let elements = Elements(value.1 .0.into_iter().map(Into::into).collect());

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strikeout {
    pub elements: Elements,
}

impl From<cst::Strikeout> for Strikeout {
    fn from(value: cst::Strikeout) -> Self {
        let elements = Elements(value.1 .0.into_iter().map(Into::into).collect());

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Emphasis {
    pub elements: Elements,
}

impl From<cst::Emphasis> for Emphasis {
    fn from(value: cst::Emphasis) -> Self {
        let elements = Elements(value.1 .0.into_iter().map(Into::into).collect());

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strong {
    pub elements: Elements,
}

impl From<cst::Strong> for Strong {
    fn from(value: cst::Strong) -> Self {
        let elements = Elements(value.1 .0.into_iter().map(Into::into).collect());

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Enclosed {
    pub elements: Elements,
}

impl From<cst::Enclosed> for Enclosed {
    fn from(value: cst::Enclosed) -> Self {
        let elements = Elements::from(*value.1 .0);

        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub link: String,
    pub elements: Option<Elements>,
}

impl From<cst::Link> for Link {
    fn from(value: cst::Link) -> Self {
        let cst::Link(_, content, _, enclosed) = value;

        let link = content.0;
        let elements = enclosed.map(|enclosed| Elements::from(*enclosed.1 .0));

        Self { link, elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawInline(pub String);

impl From<cst::RawInline> for RawInline {
    fn from(value: cst::RawInline) -> Self {
        RawInline(value.1 .0)
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
pub struct Access {
    pub ident: String,
    pub tail: Option<CallTail>,
}

impl From<cst::Access> for Access {
    fn from(value: cst::Access) -> Self {
        let cst::Access(_, ident, tail) = value;

        let ident = ident.0;
        let tail = tail.map(Into::into);

        Self { ident, tail }
    }
}

pub type Args = BTreeMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallTail {
    pub args: BTreeMap<String, Value>,
    pub content: Option<Enclosed>,
}

impl From<cst::CallTail> for CallTail {
    fn from(value: cst::CallTail) -> Self {
        let cst::CallTail(_, args, _, enclosed) = value;

        let args = args
            .0
             .0
            .into_iter()
            .map(|cst::Arg(ident, _, value)| (ident.0, value.into()))
            .collect();
        let content = enclosed.map(Into::into);

        Self { args, content }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Identifier(String),
    String(String),
}

impl From<cst::Value> for Value {
    fn from(value: cst::Value) -> Self {
        match value {
            cst::Value::Identifier(ident) => Self::Identifier(ident.0),
            cst::Value::String(s) => Self::String(s.1 .0),
        }
    }
}
