use std::fmt;

use crate::{prelude::Block, Span};

use super::code::Code;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'src> {
    Heading(Heading<'src>),
    Div(Div<'src>),
    Raw(Raw<'src>),
    TableRow(TableRow<'src>),
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    BlockQuoteItem(BlockQuoteItem<'src>),
    Text(Text<'src>),
    // Label(&'src str),
    LineBreak,
    Indentation,
}

impl<'src> fmt::Display for Node<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Heading(_) => f.write_str("Heading"),
            Self::Div(_) => f.write_str("Div"),
            Self::Raw(_) => f.write_str("Raw"),
            Self::TableRow(_) => f.write_str("TableRow"),
            Self::ListItem(_) => f.write_str("ListItem"),
            Self::EnumItem(_) => f.write_str("EnumItem"),
            Self::BlockQuoteItem(_) => f.write_str("BlockQuoteItem"),
            Self::Text(_) => f.write_str("Text"),
            Self::LineBreak => f.write_str("LineBreak"),
            Self::Indentation => f.write_str("Indentation"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div<'src> {
    pub content: Text<'src>,
    pub class: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw<'src> {
    pub content: &'src str,
    pub lang: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading<'src> {
    pub level: u8,
    pub content: Text<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow<'src> {
    pub cells: Vec<Block<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteItem<'src> {
    pub level: u8,
    pub item: Block<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'src> {
    pub content: Text<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem<'src> {
    pub content: Text<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum BlockQuoteItem<'src> {
//     ListItem(ListItem<'src>),
//     EnumItem(EnumItem<'src>),
//     Text(Text<'src>),
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline<'src> {
    Quote(Quote<'src>),
    Strikeout(Strikeout<'src>),
    Emphasis(Emphasis<'src>),
    Strong(Strong<'src>),
    Subscript(Subscript<'src>),
    Supscript(Supscript<'src>),
    Link(Link<'src>),
    Cite(Cite<'src>),
    RawInline(RawInline<'src>),
    Comment(Comment<'src>),
    Escape(Escape<'src>),
    Word(Word<'src>),
    Spacing(Spacing<'src>),
    // not used in cst stage but in ast
    SoftBreak,
    Code(Code<'src>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strikeout<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emphasis<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strong<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Supscript<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<'src> {
    pub href: &'src str,
    pub content: Option<Vec<Block<'src>>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cite<'src> {
    pub ident: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escape<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawInline<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spacing<'src> {
    pub content: &'src str,
    pub span: Span,
}
