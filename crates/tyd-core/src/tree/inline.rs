use derive_more::From;
use ecow::EcoString;

use super::{Error, Text, code::Code};
use crate::id::NodeId;

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Inline {
    Error(NodeId<Error>),
    Quote(NodeId<Quote>),
    Strikeout(NodeId<Strikeout>),
    Emphasis(NodeId<Emphasis>),
    Strong(NodeId<Strong>),
    Subscript(NodeId<Subscript>),
    Supscript(NodeId<Supscript>),
    Link(NodeId<Link>),
    Ref(NodeId<Ref>),
    RawInline(NodeId<RawInline>),
    MathInline(NodeId<MathInline>),
    Comment(NodeId<Comment>),
    Escape(NodeId<Escape>),
    Word(NodeId<Word>),
    Spacing(NodeId<Spacing>),
    SoftBreak(NodeId<SoftBreak>),
    Code(NodeId<Code>),
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quote(pub Vec<NodeId<Inline>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Strikeout(pub Vec<NodeId<Inline>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Emphasis(pub Vec<NodeId<Inline>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Strong(pub Vec<NodeId<Inline>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subscript(pub Vec<NodeId<Inline>>);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Supscript(pub Vec<NodeId<Inline>>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link {
    pub href: NodeId<Text>,
    pub content: Option<Vec<NodeId<Inline>>>,
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Ref(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct RawInline(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct MathInline(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Comment(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Escape(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Word(pub EcoString);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Spacing;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoftBreak;
