use ecow::EcoString;
use std::fmt;
use tyd_syntax::Span;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub text: EcoString,
    pub kind: TokenKind,
    pub span: Span,
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TokenKind {
    // MARKUP
    /// # ...
    CodeMarker,
    /// $ ...
    MathStart,
    /// ...
    MathContent,
    /// ... $
    MathEnd,
    /// ```
    RawStart,
    /// ```rust ...
    RawLang,
    /// ...
    RawContent,
    /// ... ```
    RawEnd,
    /// == ...
    HeadingMarker(u8),
    // /// | ...
    // TableRowStart,
    /// ... | ...
    TableRowSep,
    // /// ... |
    // TableRowEnd,
    /// - ...
    ListItemMarker,
    /// + ...
    EnumItemMarker,
    /// > ...
    TermItemMarker,
    /// > ...:
    TermItemSep,
    /// "
    QuoteMarker,
    /// ~
    StrikeoutMarker,
    /// /
    EmphasisMarker,
    /// *
    StrongMarker,
    /// _
    SubscriptMarker,
    /// ^
    SupscriptMarker,
    /// < ...
    LinkStart,
    /// ...
    LinkContent,
    /// ... >
    LinkEnd,
    /// @ ...
    RefMarker,
    /// { ...
    LabelStart,
    /// ... }
    LabelEnd,
    /// ` ...
    RawInlineStart,
    /// ...
    RawInlineContent,
    /// ... `
    RawInlineEnd,
    // /// $ ...
    // MathInlineStart,
    // /// ...
    // MathInlineContent,
    // /// ... $
    // MathInlineEnd,
    /// none_of(special)*
    Word,

    // CODE
    /// [
    ContentStart,
    /// ]
    ContentEnd,
    /// { ...
    BlockStart,
    /// ... }
    BlockEnd,
    /// ( ...
    ArgsStart,
    /// ... )
    ArgsEnd,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// ,
    Comma,
    /// " ...
    StringStart,
    /// ... "
    StringEnd,
    /// 0o24 0xA1 123 1.0
    Number,
    /// true
    KwTrue,
    /// false
    KwFalse,

    // SHARED
    /// \.
    EscapeMarker,
    /// .
    EscapeContent,
    /// [a-zA-Z]...
    Ident,
    Text,
    /// \n (\n)+
    HardBreak,
    /// ... \n ...
    SoftBreak,
    /// ' '
    Spacing,
    /// '    ' \t
    Indent,
    Dedent,
    /// % ...
    CommentMarker,
    /// ...
    CommentContent,
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeMarker => f.write_str("code_marker"),
            Self::MathStart => f.write_str("math_start"),
            Self::MathContent => f.write_str("math_content"),
            Self::MathEnd => f.write_str("math_end"),
            Self::RawStart => f.write_str("raw_start"),
            Self::RawLang => f.write_str("raw_lang"),
            Self::RawContent => f.write_str("raw_content"),
            Self::RawEnd => f.write_str("raw_end"),
            Self::HeadingMarker(level) => write!(f, "heading_marker_{level}"),
            // Self::TableRowStart => f.write_str("table_row_start"),
            Self::TableRowSep => f.write_str("table_row_sep"),
            // Self::TableRowEnd => f.write_str("table_row_end"),
            Self::ListItemMarker => f.write_str("list_item_marker"),
            Self::EnumItemMarker => f.write_str("enum_item_marker"),
            Self::TermItemMarker => f.write_str("term_item_marker"),
            Self::TermItemSep => f.write_str("term_item_sep"),
            Self::QuoteMarker => f.write_str("quote_marker"),
            Self::StrikeoutMarker => f.write_str("strikeout_marker"),
            Self::EmphasisMarker => f.write_str("emphasis_marker"),
            Self::StrongMarker => f.write_str("strong_marker"),
            Self::SubscriptMarker => f.write_str("subscript_marker"),
            Self::SupscriptMarker => f.write_str("supscript_marker"),
            Self::LinkStart => f.write_str("link_start"),
            Self::LinkContent => f.write_str("link_content"),
            Self::LinkEnd => f.write_str("link_end"),
            Self::RefMarker => f.write_str("ref_marker"),
            Self::LabelStart => f.write_str("label_start"),
            Self::LabelEnd => f.write_str("label_end"),
            Self::RawInlineStart => f.write_str("raw_inline_start"),
            Self::RawInlineContent => f.write_str("raw_inline_content"),
            Self::RawInlineEnd => f.write_str("raw_inline_end"),
            // Self::MathInlineStart => f.write_str("math_inline_start"),
            // Self::MathInlineContent => f.write_str("math_inline_content"),
            // Self::MathInlineEnd => f.write_str("math_inline_end"),
            Self::Word => f.write_str("word"),
            Self::ContentStart => f.write_str("content_start"),
            Self::ContentEnd => f.write_str("content_end"),
            Self::BlockStart => f.write_str("block_start"),
            Self::BlockEnd => f.write_str("block_end"),
            Self::ArgsStart => f.write_str("args_start"),
            Self::ArgsEnd => f.write_str("args_end"),
            Self::Colon => f.write_str("colon"),
            Self::Semicolon => f.write_str("semicolon"),
            Self::Comma => f.write_str("comma"),
            Self::StringStart => f.write_str("string_start"),
            Self::StringEnd => f.write_str("string_end"),
            Self::Number => f.write_str("number"),
            Self::KwTrue => f.write_str("kw_true"),
            Self::KwFalse => f.write_str("kw_false"),
            Self::EscapeMarker => f.write_str("escape_marker"),
            Self::EscapeContent => f.write_str("escape_content"),
            Self::Ident => f.write_str("ident"),
            Self::Text => f.write_str("text"),
            Self::HardBreak => f.write_str("hard_break"),
            Self::SoftBreak => f.write_str("soft_break"),
            Self::Spacing => f.write_str("spacing"),
            Self::Indent => f.write_str("indent"),
            Self::Dedent => f.write_str("dedent"),
            Self::CommentMarker => f.write_str("comment_marker"),
            Self::CommentContent => f.write_str("comment_content"),
            Self::Error => f.write_str("error"),
        }
    }
}
