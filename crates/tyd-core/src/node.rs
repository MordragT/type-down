use derive_more::From;

use crate::{TryAsMut, TryAsRef, impl_try_as, kind::NodeKind, tree::*};

/// Represents a node in the abstract syntax tree (AST).
///
/// The `Node` enum encompasses all possible elements that can exist in the document structure,
/// organized into several categories:
/// - General nodes (Error, Tag, Text, Label)
/// - Block-level elements (Block, Raw, Heading, etc.)
/// - Inline elements (Inline, Quote, Emphasis, etc.)
/// - Code elements (Code, Expr, Let, etc.)
#[derive(Clone, Debug, PartialEq, From)]
pub enum Node {
    /// Represents an error in the document
    Error(Error),
    /// A tag element
    Tag(Tag),
    /// Plain text content
    Text(Text),
    /// A label element
    Label(Label),

    // Block
    /// A block-level container
    Block(Block),
    /// Raw block content
    Raw(Raw),
    /// A heading element (like h1, h2, etc.)
    Heading(Heading),
    /// The marker for a heading (e.g., '=', '==')
    HeadingMarker(HeadingMarker),
    /// A table element
    Table(Table),
    /// A row within a table
    TableRow(TableRow),
    /// An unordered list
    List(List),
    /// An item within a list
    ListItem(ListItem),
    /// An enumerated (ordered) list
    Enum(Enum),
    /// An item within an enumerated list
    EnumItem(EnumItem),
    /// A definition list/terms list
    Terms(Terms),
    /// An item within a terms list
    TermItem(TermItem),
    /// A paragraph element
    Paragraph(Paragraph),
    /// Plain content without specific formatting
    Plain(Plain),

    // Inline
    /// An inline container element
    Inline(Inline),
    /// Quoted text
    Quote(Quote),
    /// Text with strikethrough formatting
    Strikeout(Strikeout),
    /// Emphasized text (typically italic)
    Emphasis(Emphasis),
    /// Strongly emphasized text (typically bold)
    Strong(Strong),
    /// Subscript text
    Subscript(Subscript),
    /// Superscript text
    Supscript(Supscript),
    /// A hyperlink
    Link(Link),
    /// A reference to another element
    Ref(Ref),
    /// Raw inline content
    RawInline(RawInline),
    /// Inline mathematical notation
    MathInline(MathInline),
    /// A comment
    Comment(Comment),
    /// An escaped character
    Escape(Escape),
    /// A word unit
    Word(Word),
    /// Whitespace or other spacing
    Spacing(Spacing),
    /// A soft line break
    SoftBreak(SoftBreak),

    // Code
    /// A code section
    Code(Code),
    /// An expression in code
    Expr(Expr),
    /// A let binding in code
    Let(Let),
    /// A binding in code
    Bind(Bind),
    /// An if statement/expression
    If(If),
    /// A for loop
    For(For),
    /// A function or method call
    Call(Call),
    /// Arguments to a function call
    Args(Args),
    /// A single argument
    Arg(Arg),
    /// A literal value (number, string, etc.)
    Literal(Literal),
    /// An identifier in code
    Ident(Ident),
    /// Content within a code structure
    Content(Content),
}

impl_try_as!(
    Node,
    Error(Error),
    Tag(Tag),
    Text(Text),
    Label(Label),
    // Block
    Block(Block),
    Raw(Raw),
    Heading(Heading),
    HeadingMarker(HeadingMarker),
    Table(Table),
    TableRow(TableRow),
    List(List),
    ListItem(ListItem),
    Enum(Enum),
    EnumItem(EnumItem),
    Terms(Terms),
    TermItem(TermItem),
    Paragraph(Paragraph),
    Plain(Plain),
    // Inline
    Inline(Inline),
    Quote(Quote),
    Strikeout(Strikeout),
    Emphasis(Emphasis),
    Strong(Strong),
    Subscript(Subscript),
    Supscript(Supscript),
    Link(Link),
    Ref(Ref),
    RawInline(RawInline),
    MathInline(MathInline),
    Comment(Comment),
    Escape(Escape),
    Word(Word),
    Spacing(Spacing),
    SoftBreak(SoftBreak),
    // Code
    Code(Code),
    Expr(Expr),
    Let(Let),
    Bind(Bind),
    If(If),
    For(For),
    Call(Call),
    Args(Args),
    Arg(Arg),
    Literal(Literal),
    Ident(Ident),
    Content(Content)
);

impl Node {
    /// Returns the kind of this node as a `NodeKind` enum value.
    ///
    /// This method provides a way to determine the type of node without
    /// pattern matching on the entire enum.
    pub fn kind(&self) -> NodeKind {
        match self {
            Self::Error(_) => NodeKind::Error,
            Self::Tag(_) => NodeKind::Tag,
            Self::Text(_) => NodeKind::Text,
            Self::Label(_) => NodeKind::Label,

            // Block
            Self::Block(_) => NodeKind::Block,
            Self::Raw(_) => NodeKind::Raw,
            Self::Heading(_) => NodeKind::Heading,
            Self::HeadingMarker(_) => NodeKind::HeadingMarker,
            Self::Table(_) => NodeKind::Table,
            Self::TableRow(_) => NodeKind::TableRow,
            Self::List(_) => NodeKind::List,
            Self::ListItem(_) => NodeKind::ListItem,
            Self::Enum(_) => NodeKind::Enum,
            Self::EnumItem(_) => NodeKind::EnumItem,
            Self::Terms(_) => NodeKind::Terms,
            Self::TermItem(_) => NodeKind::TermItem,
            Self::Paragraph(_) => NodeKind::Paragraph,
            Self::Plain(_) => NodeKind::Plain,

            // Inline
            Self::Inline(_) => NodeKind::Inline,
            Self::Quote(_) => NodeKind::Quote,
            Self::Strikeout(_) => NodeKind::Strikeout,
            Self::Emphasis(_) => NodeKind::Emphasis,
            Self::Strong(_) => NodeKind::Strong,
            Self::Subscript(_) => NodeKind::Subscript,
            Self::Supscript(_) => NodeKind::Supscript,
            Self::Link(_) => NodeKind::Link,
            Self::Ref(_) => NodeKind::Ref,
            Self::RawInline(_) => NodeKind::RawInline,
            Self::MathInline(_) => NodeKind::MathInline,
            Self::Comment(_) => NodeKind::Comment,
            Self::Escape(_) => NodeKind::Escape,
            Self::Word(_) => NodeKind::Word,
            Self::Spacing(_) => NodeKind::Spacing,
            Self::SoftBreak(_) => NodeKind::SoftBreak,

            // Code
            Self::Code(_) => NodeKind::Code,
            Self::Expr(_) => NodeKind::Expr,
            Self::Let(_) => NodeKind::Let,
            Self::Bind(_) => NodeKind::Bind,
            Self::If(_) => NodeKind::If,
            Self::For(_) => NodeKind::For,
            Self::Call(_) => NodeKind::Call,
            Self::Args(_) => NodeKind::Args,
            Self::Arg(_) => NodeKind::Arg,
            Self::Literal(_) => NodeKind::Literal,
            Self::Ident(_) => NodeKind::Ident,
            Self::Content(_) => NodeKind::Content,
        }
    }

    /// Determines if this node is a block-level element.
    ///
    /// Block-level elements include Block, Raw, Heading, Table, List, Enum,
    /// Terms, Paragraph, and Plain nodes.
    pub fn is_block(&self) -> bool {
        matches!(
            self,
            Self::Block(_)
                | Self::Raw(_)
                | Self::Heading(_)
                | Self::Table(_)
                | Self::List(_)
                | Self::Enum(_)
                | Self::Terms(_)
                | Self::Paragraph(_)
                | Self::Plain(_)
        )
    }

    /// Determines if this node is an inline element.
    ///
    /// Inline elements include Inline, Quote, Strikeout, Emphasis, Strong, Subscript,
    /// Supscript, Link, Ref, RawInline, MathInline, Comment, Escape, Word, Spacing,
    /// SoftBreak, and Code nodes.
    pub fn is_inline(&self) -> bool {
        matches!(
            self,
            Self::Inline(_)
                | Self::Quote(_)
                | Self::Strikeout(_)
                | Self::Emphasis(_)
                | Self::Strong(_)
                | Self::Subscript(_)
                | Self::Supscript(_)
                | Self::Link(_)
                | Self::Ref(_)
                | Self::RawInline(_)
                | Self::MathInline(_)
                | Self::Comment(_)
                | Self::Escape(_)
                | Self::Word(_)
                | Self::Spacing(_)
                | Self::SoftBreak(_)
                | Self::Code(_)
        )
    }

    /// Determines if this node is a code element.
    ///
    /// Code elements include Code, Expr, Let, Bind, If, For, Call, Args, Arg,
    /// Literal, Ident, and Content nodes.
    pub fn is_code(&self) -> bool {
        matches!(
            self,
            Self::Code(_)
                | Self::Expr(_)
                | Self::Let(_)
                | Self::Bind(_)
                | Self::If(_)
                | Self::For(_)
                | Self::Call(_)
                | Self::Args(_)
                | Self::Arg(_)
                | Self::Literal(_)
                | Self::Ident(_)
                | Self::Content(_)
        )
    }
}
