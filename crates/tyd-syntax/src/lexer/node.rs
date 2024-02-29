use crate::ast::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'src> {
    ContentStart,
    ContentEnd,
    Raw(Raw<'src>),
    Heading(Heading<'src>),
    TableRow(TableRow<'src>),
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    TermItem(TermItem<'src>),
    Text(Text<'src>),
    // Label(&'src str),
    LineBreak,
    // HardBreak,
    Indent,
    Dedent,
}

impl<'src> fmt::Display for Node<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentStart => f.write_str("ContentStart"),
            Self::ContentEnd => f.write_str("ContentEnd"),
            Self::Raw(_) => f.write_str("Raw"),
            Self::Heading(_) => f.write_str("Heading"),
            Self::TableRow(_) => f.write_str("TableRow"),
            Self::ListItem(_) => f.write_str("ListItem"),
            Self::EnumItem(_) => f.write_str("EnumItem"),
            Self::TermItem(_) => f.write_str("BlockQuoteItem"),
            Self::Text(_) => f.write_str("Text"),
            Self::LineBreak => f.write_str("LineBreak"),
            // Self::HardBreak => f.write_str("HardBreak"),
            Self::Indent => f.write_str("Indent"),
            Self::Dedent => f.write_str("Dedent"),
        }
    }
}
