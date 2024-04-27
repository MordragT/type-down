use ecow::EcoString;

use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub blocks: Vec<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Raw(Raw),
    Heading(Heading),
    Table(Table),
    List(List),
    Enum(Enum),
    Terms(Terms),
    Paragraph(Paragraph),
    Plain(Plain),
    Error(Span),
}

impl Block {
    pub fn span(&self) -> &Span {
        match self {
            Self::Raw(raw) => &raw.span,
            Self::Heading(heading) => &heading.span,
            Self::Table(table) => &table.span,
            Self::List(list) => &list.span,
            Self::Enum(enumeration) => &enumeration.span,
            Self::Terms(term) => &term.span,
            Self::Paragraph(p) => &p.span,
            Self::Plain(plain) => &plain.span,
            Self::Error(span) => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub label: EcoString,
    pub span: Span,
}

impl ToString for Label {
    fn to_string(&self) -> String {
        self.label.to_string()
    }
}

impl Into<String> for Label {
    fn into(self) -> String {
        self.label.into()
    }
}

impl Into<String> for &Label {
    fn into(self) -> String {
        self.label.as_ref().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw {
    pub content: RawContent,
    pub lang: Option<RawLang>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawContent {
    pub content: EcoString,
    pub span: Span,
}

impl ToString for RawContent {
    fn to_string(&self) -> String {
        self.content.to_string()
    }
}

impl Into<String> for RawContent {
    fn into(self) -> String {
        self.content.into()
    }
}

impl Into<String> for &RawContent {
    fn into(self) -> String {
        self.content.as_ref().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawLang {
    pub lang: EcoString,
    pub span: Span,
}

impl ToString for RawLang {
    fn to_string(&self) -> String {
        self.lang.to_string()
    }
}

impl Into<String> for RawLang {
    fn into(self) -> String {
        self.lang.into()
    }
}

impl Into<String> for &RawLang {
    fn into(self) -> String {
        self.lang.as_ref().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: HeadingLevel,
    pub content: Vec<Inline>,
    pub label: Option<Label>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingLevel {
    pub level: u8,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub label: Option<Label>,
    pub span: Span,
    pub col_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    pub cells: Vec<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub items: Vec<ListItem>,
    pub span: Span,
}

impl From<ListItem> for List {
    fn from(value: ListItem) -> Self {
        let items = vec![value.clone()];
        let ListItem { content: _, span } = value;

        Self { items, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub content: Vec<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub items: Vec<EnumItem>,
    pub span: Span,
}
impl From<EnumItem> for Enum {
    fn from(value: EnumItem) -> Self {
        let items = vec![value.clone()];
        let EnumItem { content: _, span } = value;

        Self { items, span }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem {
    pub content: Vec<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terms {
    pub content: Vec<TermItem>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermItem {
    pub term: Vec<Inline>,
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plain {
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
    SoftBreak(SoftBreak),
    Code(Code),
    Error(Span),
}

impl Inline {
    pub fn span(&self) -> &Span {
        match self {
            Self::Quote(q) => &q.span,
            Self::Strikeout(s) => &s.span,
            Self::Emphasis(e) => &e.span,
            Self::Strong(s) => &s.span,
            Self::Subscript(s) => &s.span,
            Self::Supscript(s) => &s.span,
            Self::Link(l) => &l.span,
            Self::Cite(c) => &c.span,
            Self::RawInline(r) => &r.span,
            Self::MathInline(m) => &m.span,
            Self::Comment(c) => &c.span,
            Self::Escape(e) => &e.span,
            Self::Word(w) => &w.span,
            Self::Spacing(s) => &s.span,
            Self::SoftBreak(s) => &s.span,
            Self::Code(c) => &c.span,
            Self::Error(span) => span,
        }
    }
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
    pub href: Href,
    pub content: Option<Vec<Inline>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Href {
    pub href: EcoString,
    pub span: Span,
}

impl ToString for Href {
    fn to_string(&self) -> String {
        self.href.to_string()
    }
}

impl Into<String> for Href {
    fn into(self) -> String {
        self.href.into()
    }
}

impl Into<String> for &Href {
    fn into(self) -> String {
        self.href.as_ref().into()
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftBreak {
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
    Ident(Ident),
    Call(Call),
    Literal(Literal, Span),
    Block(Vec<Expr>, Span),
    Content(Content),
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Self::Ident(i) => &i.span,
            Self::Call(c) => &c.span,
            Self::Literal(_, span) => span,
            Self::Block(_, span) => span,
            Self::Content(c) => &c.span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub ident: EcoString,
    pub span: Span,
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.ident.to_string()
    }
}

impl Into<String> for Ident {
    fn into(self) -> String {
        self.ident.into()
    }
}

impl Into<String> for &Ident {
    fn into(self) -> String {
        self.ident.as_ref().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub ident: Ident,
    pub args: Args,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub args: Vec<Arg>,
    pub content: Option<Content>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    pub content: Vec<Inline>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub name: Option<Ident>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Boolean(bool),
    Int(i64),
    // Float(f64),
    Str(EcoString),
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(b) => b.to_string(),
            Self::Int(i) => i.to_string(),
            Self::Str(s) => s.to_string(),
        }
    }
}
