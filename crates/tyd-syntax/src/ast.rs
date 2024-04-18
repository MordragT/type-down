use ecow::EcoString;

use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Div(Div),
    Raw(Raw),
    Heading(Heading),
    Table(Table),
    List(List),
    Enum(Enum),
    Term(Term),
    Paragraph(Paragraph),
    Plain(Plain),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div {
    pub content: Vec<Block>,
    pub class: Option<EcoString>,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw {
    pub content: EcoString,
    pub lang: Option<EcoString>,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub content: Text,
    pub label: Option<EcoString>,
    pub span: Span,
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub label: Option<EcoString>,
    pub span: Span,
    pub col_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    pub cells: Vec<Block>,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub items: Vec<ListItem>,
    pub label: Option<EcoString>,
    pub span: Span,
}

impl From<ListItem> for List {
    fn from(value: ListItem) -> Self {
        let items = vec![value.clone()];
        let ListItem {
            content: _,
            label,
            span,
        } = value;

        Self { items, label, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub content: Vec<Block>,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub items: Vec<EnumItem>,
    pub label: Option<EcoString>,
    pub span: Span,
}
impl From<EnumItem> for Enum {
    fn from(value: EnumItem) -> Self {
        let items = vec![value.clone()];
        let EnumItem {
            content: _,
            label,
            span,
        } = value;

        Self { items, label, span }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem {
    pub content: Vec<Block>,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub content: Vec<TermItem>,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermItem {
    pub term: Text,
    pub content: Text,
    pub label: Option<EcoString>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub content: Vec<Inline>,
    pub span: Span,
}

impl From<Text> for Paragraph {
    fn from(value: Text) -> Self {
        let Text { content, span } = value;

        Self { content, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plain {
    pub content: Vec<Inline>,
    pub span: Span,
}

impl From<Text> for Plain {
    fn from(value: Text) -> Self {
        let Text { content, span } = value;

        Self { content, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Quote(Quote),
    Strikeout(Strikeout),
    Emphasis(Emphasis),
    Strong(Strong),
    Subscript(Subscript),
    Supscript(Supscript),
    Link(Link),
    Cite(Cite),
    RawInline(RawInline),
    MathInline(MathInline),
    Comment(Comment),
    Escape(Escape),
    Word(Word),
    Spacing(Spacing),
    SoftBreak,
    Code(Code),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strikeout {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emphasis {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strong {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Supscript {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub href: EcoString,
    pub content: Option<Vec<Inline>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cite {
    pub ident: EcoString,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escape {
    pub content: EcoString,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawInline {
    pub content: EcoString,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathInline {
    pub content: EcoString,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub content: EcoString,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub content: EcoString,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spacing {
    pub content: EcoString,
    pub span: Span,
}

// Code

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    pub expr: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(EcoString),
    Call(Call),
    Literal(Literal),
    Block(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub ident: EcoString,
    pub args: Vec<Arg>,
    pub content: Option<Vec<Inline>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub name: Option<EcoString>,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Boolean(bool),
    Int(i64),
    // Float(f64),
    Str(EcoString),
}
