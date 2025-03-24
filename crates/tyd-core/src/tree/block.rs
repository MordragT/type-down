use derive_more::From;

use super::{Label, Tag, Text, inline::Inline};
use crate::{id::NodeId, kind::NodeKind};

/// Represents different types of block-level elements in a document.
///
/// Block elements form the structural components of a document, such as
/// paragraphs, headings, lists, and tables.
#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Block {
    /// A block of raw, unprocessed text with optional language tag
    Raw(NodeId<Raw>),
    /// A section heading of varying levels
    Heading(NodeId<Heading>),
    /// A table with rows and columns
    Table(NodeId<Table>),
    /// An unordered list of items
    List(NodeId<List>),
    /// An ordered (enumerated) list of items
    Enum(NodeId<Enum>),
    /// A definition list with terms and descriptions
    Terms(NodeId<Terms>),
    /// A standard paragraph of text
    Paragraph(NodeId<Paragraph>),
    /// Plain text content without paragraph formatting
    Plain(NodeId<Plain>),
}

impl Block {
    /// Returns the kind of node this block represents.
    ///
    /// This method provides a way to determine the type of block
    /// without having to match on the enum variant.
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

/// Represents a block of raw, unprocessed text.
///
/// Raw blocks typically contain code or other content that should not be
/// parsed as markup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Raw {
    /// The text content of the raw block
    pub text: NodeId<Text>,
    /// Optional language tag for syntax highlighting
    pub lang: Option<NodeId<Tag>>,
}

/// Represents a section heading in a document.
///
/// Headings provide structure to a document by denoting sections and subsections.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Heading {
    /// Indicates the level of the heading (e.g., h1, h2, h3)
    pub marker: NodeId<HeadingMarker>,
    /// The text content of the heading
    pub content: Vec<NodeId<Inline>>,
    /// Optional label for cross-referencing
    pub label: Option<NodeId<Label>>,
}

/// Marker that indicates the level of a heading.
///
/// The value typically ranges from 1 to 6, corresponding to HTML h1-h6 elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeadingMarker(pub u8);

/// Represents a table structure with rows and columns.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Table {
    /// The rows of the table
    pub rows: Vec<NodeId<TableRow>>,
    /// The number of columns in the table
    pub columns: usize,
    /// Optional label for cross-referencing
    pub label: Option<NodeId<Label>>,
}

/// Represents a single row in a table.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableRow(pub Vec<NodeId<Block>>);

/// Represents an unordered list of items.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List(pub Vec<NodeId<ListItem>>);

/// Represents a single item in an unordered list.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListItem(pub Vec<NodeId<Block>>);

/// Represents an ordered (enumerated) list of items.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enum(pub Vec<NodeId<EnumItem>>);

/// Represents a single item in an ordered list.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumItem(pub Vec<NodeId<Block>>);

/// Represents a definition list with terms and descriptions.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Terms(pub Vec<NodeId<TermItem>>);

/// Represents a term-description pair in a definition list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TermItem {
    /// The term being defined
    pub term: Vec<NodeId<Inline>>,
    /// The description or definition of the term
    pub desc: Vec<NodeId<Inline>>,
}

/// Represents a standard paragraph of text.
///
/// Paragraphs are the basic unit of text organization in a document.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Paragraph(pub Vec<NodeId<Inline>>);

/// Represents plain text content without paragraph formatting.
///
/// Plain blocks are similar to paragraphs but may be used in contexts
/// where paragraph semantics are not appropriate.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plain(pub Vec<NodeId<Inline>>);
