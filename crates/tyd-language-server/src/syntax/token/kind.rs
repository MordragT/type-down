#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TokenKind {
    Error,
    Label,
    RawLang,
    RawContent,
    HeadingMarker(u8),
    Href,
    Cite,
    RawInline,
    MathInline,
    Comment,
    Escape,
    Word,
    Spacing,
    SoftBreak,
    // kinds of idents
    Ident,
    CallIdent,
    ArgIdent,
    // literals
    Literal,
    Str,
    Int,
    Bool,
}
