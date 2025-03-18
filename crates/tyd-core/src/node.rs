use derive_more::From;

use crate::{TryAsMut, TryAsRef, impl_try_as, kind::NodeKind, tree::*};

#[derive(Clone, Debug, PartialEq, From)]
pub enum Node {
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
