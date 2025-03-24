use derive_more::From;
use ecow::EcoString;

use super::{Error, Text, code::Code};
use crate::id::NodeId;

/// Represents all inline elements in the document structure.
///
/// This enum contains all possible inline nodes that can be used within text content.
/// Each variant holds a reference to its underlying node type through a `NodeId`.
#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Inline {
    /// An error element
    Error(NodeId<Error>),
    /// Quoted text
    Quote(NodeId<Quote>),
    /// Text with strikethrough formatting
    Strikeout(NodeId<Strikeout>),
    /// Emphasized text (typically italicized)
    Emphasis(NodeId<Emphasis>),
    /// Strongly emphasized text (typically bold)
    Strong(NodeId<Strong>),
    /// Text formatted as subscript
    Subscript(NodeId<Subscript>),
    /// Text formatted as superscript
    Supscript(NodeId<Supscript>),
    /// A hyperlink element
    Link(NodeId<Link>),
    /// A reference to another element
    Ref(NodeId<Ref>),
    /// Raw inline content that should be included as-is
    RawInline(NodeId<RawInline>),
    /// Mathematical notation in inline form
    MathInline(NodeId<MathInline>),
    /// A comment that is not rendered in the final output
    Comment(NodeId<Comment>),
    /// An escaped character
    Escape(NodeId<Escape>),
    /// A single word
    Word(NodeId<Word>),
    /// Whitespace between elements
    Spacing(NodeId<Spacing>),
    /// A soft line break
    SoftBreak(NodeId<SoftBreak>),
    /// Inline code snippet
    Code(NodeId<Code>),
}

/// Represents quoted text content.
///
/// Contains a vector of inline elements that form the quoted content.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quote(pub Vec<NodeId<Inline>>);

/// Represents text with strikethrough formatting.
///
/// Contains a vector of inline elements that are displayed with a line through them.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Strikeout(pub Vec<NodeId<Inline>>);

/// Represents emphasized text, typically displayed in italic.
///
/// Contains a vector of inline elements that are emphasized.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Emphasis(pub Vec<NodeId<Inline>>);

/// Represents strongly emphasized text, typically displayed in bold.
///
/// Contains a vector of inline elements that are strongly emphasized.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Strong(pub Vec<NodeId<Inline>>);

/// Represents text displayed as subscript.
///
/// Contains a vector of inline elements that are formatted as subscript.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subscript(pub Vec<NodeId<Inline>>);

/// Represents text displayed as superscript.
///
/// Contains a vector of inline elements that are formatted as superscript.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Supscript(pub Vec<NodeId<Inline>>);

/// Represents a hyperlink.
///
/// Contains both the link target and optional display content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link {
    /// The URL or path that this link points to
    pub href: NodeId<Text>,
    /// Optional content to display instead of the raw URL
    pub content: Option<Vec<NodeId<Inline>>>,
}

/// Represents a reference to another element in the document.
///
/// Contains a string identifier for the referenced element.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Ref(pub EcoString);

/// Represents raw inline content that should be included verbatim.
///
/// Contains the raw string content to be included without processing.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct RawInline(pub EcoString);

/// Represents mathematical notation in inline form.
///
/// Contains the string representation of the mathematical expression.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct MathInline(pub EcoString);

/// Represents a comment that is not rendered in the final output.
///
/// Contains the string content of the comment.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Comment(pub EcoString);

/// Represents an escaped character or string.
///
/// Contains the string representation of the escaped content.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Escape(pub EcoString);

/// Represents a single word in the text.
///
/// Contains the string content of the word.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Word(pub EcoString);

/// Represents whitespace between elements.
///
/// A marker struct with no additional data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Spacing;

/// Represents a soft line break in the text.
///
/// A marker struct with no additional data that indicates a line break
/// which may be treated differently than hard breaks.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoftBreak;
