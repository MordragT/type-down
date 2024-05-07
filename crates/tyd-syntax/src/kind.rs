#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    /// The contents of a file
    Document,
    /// An invalid sequence of characters
    Error,
    /// Plain text without markup
    Text,
    /// Plain word
    Word,
    /// Whitespace
    Spacing,
    /// A line break
    SoftBreak,

    /// Raw block ``` ... ```
    Raw,
    /// A language tag ```rust ... ```
    RawLang,
    /// A block heading `= Heading`
    Heading,
    /// Introduces a heading `=`, `==`
    HeadingMarker,
    /// A pipe-style table
    Table,
    /// One row of a table `| a | b |`
    TableRow,
    /// List block
    List,
    /// An item in a list `- ...`
    ListItem,
    /// Introduces a list `-`
    ListMarker,
    /// Enum block
    Enum,
    /// An item in a enum `+ ...`
    EnumItem,
    /// Introduces a enum `+`
    EnumMarker,
    /// Term block
    Terms,
    /// An term item `> ... : ...`
    TermItem,
    /// Introduces a term `>`
    TermMarker,
    /// A paragraph block separated by by two or more newlines
    Paragraph,
    /// A plain block
    Plain,

    /// Raw inline ` ... `
    RawInline,
    /// Math inline `$ ... $`
    MathInline,
    /// A quote `" ... "`
    Quote,
    /// Stroked out `~ ... ~`
    Strikeout,
    /// Emphasized `/ ... /`
    Emphasis,
    /// Strong `* ... *`
    Strong,
    /// Subscript `H_2O`
    Subscript,
    /// Introduces a subscript `_`
    SubscriptMarker,
    /// Supscript `2^n`
    Supscript,
    /// Introduces a supscript `^`
    SupscriptMarker,
    /// A hyperlink `<https://example.com>`
    Link,
    /// A label `{label}`
    Label,
    /// A reference `@target`
    Ref,
    /// Introduces a reference `@`
    RefMarker,
    /// An escape sequence `\@`
    Escape,
    /// Introduces an escape sequence `\`
    EscapeMarker,
    /// A comment `% ...`
    Comment,
    /// Introduces a comment `%`
    CommentMarker,

    /// A code statement `# ... `
    Code,
    /// Introduces a code statement `#`
    CodeMarker,
    /// Multiline markup content `[ ... ]`
    Content,
    /// A code expression `#call()`, `#let x = 10`
    Expr,
    /// Multiple expressions
    ExprBlock,
    /// A function call `#call()`
    Call,
    /// The identifier of an function call `call`
    CallIdent,
    /// Function arguments `(a: 10, b: 20)`
    Args,
    /// One argument `a: 10`
    Arg,
    /// The identifier of an argument `a`
    ArgIdent,
    /// An variable ident
    Ident,
    /// A string `" ... "`
    Str,
    /// An integer `1234`
    Int,
    /// A float `1.234`
    Float,
    /// An boolean `true`, `false`
    Bool,

    /// `:`
    Colon,
}

// impl SyntaxKind {
//     pub fn is_leaf(&self) -> bool {
//         use SyntaxKind::*;

//         matches!(
//             self,
//             Error
//                 | Label
//                 | RawLang
//                 | RawContent
//                 | HeadingLevel
//                 | Href
//                 | CiteMarker
//                 | RawInline
//                 | MathInline
//                 | CommentMarker
//                 | CommentContent
//                 | EscapeMarker
//                 | EscapeContent
//                 | Word
//                 | Spacing
//                 | SoftBreak
//                 | Ident
//                 | CallIdent
//                 | ArgIdent
//                 | Literal
//                 | Str
//                 | Int
//                 | Bool
//         )
//     }

//     pub fn is_branch(&self) -> bool {
//         !self.is_leaf()
//     }
// }
