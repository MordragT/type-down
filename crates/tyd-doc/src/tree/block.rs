use derive_more::From;

use super::{Label, Tag, Text, inline::Inline};
use crate::{id::NodeId, kind::NodeKind};

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Block {
    Raw(NodeId<Raw>),
    Heading(NodeId<Heading>),
    Table(NodeId<Table>),
    List(NodeId<List>),
    Enum(NodeId<Enum>),
    Terms(NodeId<Terms>),
    Paragraph(NodeId<Paragraph>),
    Plain(NodeId<Plain>),
}

impl Block {
    pub fn kind(&self) -> NodeKind {
        match self {
            Self::Raw(_) => NodeKind::Raw,
            Self::Heading(_) => NodeKind::Heading,
            Self::Table(_) => NodeKind::Table,
            Self::List(_) => NodeKind::List,
            Self::Enum(_) => NodeKind::Enum,
            Self::Terms(_) => NodeKind::Terms,
            Self::Paragraph(_) => NodeKind::Paragraph,
            Self::Plain(_) => NodeKind::Plain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Raw {
    pub text: NodeId<Text>,
    pub lang: Option<NodeId<Tag>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Heading {
    pub marker: NodeId<HeadingMarker>,
    pub content: Vec<NodeId<Inline>>,
    pub label: Option<NodeId<Label>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeadingMarker(pub u8);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Table {
    pub rows: Vec<NodeId<TableRow>>,
    pub columns: usize,
    pub label: Option<NodeId<Label>>,
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableRow(pub Vec<NodeId<Block>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List(pub Vec<NodeId<ListItem>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListItem(pub Vec<NodeId<Block>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enum(pub Vec<NodeId<EnumItem>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumItem(pub Vec<NodeId<Block>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Terms(pub Vec<NodeId<TermItem>>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TermItem {
    pub term: Vec<NodeId<Inline>>,
    pub desc: Vec<NodeId<Inline>>,
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Paragraph(pub Vec<NodeId<Inline>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plain(pub Vec<NodeId<Inline>>);
