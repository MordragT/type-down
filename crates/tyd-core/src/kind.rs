/// Defines the different types of nodes in the abstract syntax tree.
///
/// The enum is organized into several logical groups:
/// - General purpose nodes
/// - Block-level nodes
/// - Inline-level nodes
/// - Code-related nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeKind {
    /// Represents an error node
    Error,
    /// Represents a tag node
    Tag,
    /// Represents a text node
    Text,
    /// Represents a label node
    Label,

    // Block
    /// Represents a block container node
    Block,
    /// Represents a raw content block
    Raw,
    /// Represents a heading element
    Heading,
    /// Represents the marker for a heading (e.g., =, ==)
    HeadingMarker,
    /// Represents a table element
    Table,
    /// Represents a row within a table
    TableRow,
    /// Represents an unordered list
    List,
    /// Represents an item within an unordered list
    ListItem,
    /// Represents an enumerated (ordered) list
    Enum,
    /// Represents an item within an enumerated list
    EnumItem,
    /// Represents a definition terms list
    Terms,
    /// Represents an item within a terms list
    TermItem,
    /// Represents a paragraph block
    Paragraph,
    /// Represents a plain text block
    Plain,

    // Inline
    /// Represents an inline container element
    Inline,
    /// Represents quoted text
    Quote,
    /// Represents struck-out text
    Strikeout,
    /// Represents emphasized text (often italic)
    Emphasis,
    /// Represents strongly emphasized text (often bold)
    Strong,
    /// Represents subscript text
    Subscript,
    /// Represents superscript text
    Supscript,
    /// Represents a hyperlink
    Link,
    /// Represents a reference to another element
    Ref,
    /// Represents raw inline content
    RawInline,
    /// Represents inline mathematical notation
    MathInline,
    /// Represents a comment
    Comment,
    /// Represents an escaped character
    Escape,
    /// Represents a word
    Word,
    /// Represents spacing between elements
    Spacing,
    /// Represents a soft line break
    SoftBreak,

    // Code
    /// Represents a code section
    Code,
    /// Represents an expression in code
    Expr,
    /// Represents a let statement/binding
    Let,
    /// Represents a binding operation
    Bind,
    /// Represents an if conditional statement
    If,
    /// Represents a for loop
    For,
    /// Represents a function or method call
    Call,
    /// Represents function arguments
    Args,
    /// Represents a single argument
    Arg,
    /// Represents a literal value
    Literal,
    /// Represents an identifier
    Ident,
    /// Represents content within code
    Content,
}
