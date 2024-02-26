use core::fmt;

use chumsky::{
    prelude::*,
    text::{ascii, newline},
};
use miette::NamedSource;

use crate::{
    error::{ParseError, TydError},
    Span,
};

pub type Extra<'src> = extra::Err<Rich<'src, char, Span>>;

// TODO maybe allow word to take more of the
// special symbols if other variants are not successful
const SPECIAL: &str = " \\\n\"{}[]/*~_^@#`";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cst<'src> {
    pub nodes: Vec<Node<'src>>,
}

impl<'src> Cst<'src> {
    pub fn parse(src: &'src str, name: impl AsRef<str>) -> Result<Self, TydError> {
        let parser = cst_parser();
        let cst = parser
            .parse(&src)
            .into_result()
            .map_err(|errs| ParseError {
                src: NamedSource::new(name, src.to_owned()),
                related: errs.into_iter().map(Into::into).collect(),
            })?;

        Ok(cst)
    }

    pub fn parse_spanned(
        src: &'src str,
        name: impl AsRef<str>,
    ) -> Result<Vec<(Node<'src>, Span)>, TydError> {
        let parser = nodes_spanned_parser();

        let nodes = parser
            .parse(&src)
            .into_result()
            .map_err(|errs| ParseError {
                src: NamedSource::new(name, src.to_owned()),
                related: errs.into_iter().map(Into::into).collect(),
            })?;

        Ok(nodes)
    }
}

pub fn cst_parser<'src>() -> impl Parser<'src, &'src str, Cst<'src>, Extra<'src>> {
    let node = node_parser();

    node.repeated()
        .at_least(1)
        .collect()
        .then_ignore(end())
        .map(|nodes| Cst { nodes })
}

pub fn nodes_spanned_parser<'src>(
) -> impl Parser<'src, &'src str, Vec<(Node<'src>, Span)>, Extra<'src>> {
    let node = node_parser();

    node.map_with(|node, e| (node, e.span()))
        .repeated()
        .at_least(1)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'src> {
    Div(Div<'src>),
    Raw(Raw<'src>),
    TableRow(TableRow<'src>),
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    BlockQuoteItem(BlockQuoteItem<'src>),
    Heading(Heading<'src>),
    Text(Text<'src>),
    // Label(&'src str),
    LineBreak,
    Indentation,
}

impl<'src> fmt::Display for Node<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Div(_) => f.write_str("Div"),
            Self::Raw(_) => f.write_str("Raw"),
            Self::TableRow(_) => f.write_str("TableRow"),
            Self::ListItem(_) => f.write_str("ListItem"),
            Self::EnumItem(_) => f.write_str("EnumItem"),
            Self::BlockQuoteItem(_) => f.write_str("BlockQuoteItem"),
            Self::Heading(_) => f.write_str("Heading"),
            Self::Text(_) => f.write_str("Text"),
            Self::LineBreak => f.write_str("LineBreak"),
            Self::Indentation => f.write_str("Indentation"),
        }
    }
}

pub fn node_parser<'src>() -> impl Parser<'src, &'src str, Node<'src>, Extra<'src>> {
    recursive(|node| {
        let inline = inline_parser(word_parser());

        let text = text_parser(inline).boxed();
        let list_item = list_item_parser(text.clone()).boxed();
        let enum_item = enum_item_parser(text.clone()).boxed();
        let bq_item = block_quote_item_parser(text.clone(), enum_item.clone(), list_item.clone());

        let heading = heading_parser(text.clone()).map(Node::Heading);
        // let label = label_parser().map(Node::Label);

        choice((
            div_parser(node).map(Node::Div),
            raw_parser().map(Node::Raw),
            table_row_parser().map(Node::TableRow),
            list_item.map(Node::ListItem),
            enum_item.map(Node::EnumItem),
            bq_item.map(Node::BlockQuoteItem),
            heading,
            newline().to(Node::LineBreak),
            just("    ").to(Node::Indentation),
            text.map(Node::Text),
        ))
        .boxed()
    })
}

// TODO Div content should be Vec of Nodes or Vec of Inline

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div<'src> {
    pub content: Vec<Node<'src>>,
    pub class: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn div_parser<'src, N>(node: N) -> impl Parser<'src, &'src str, Div<'src>, Extra<'src>>
where
    N: Parser<'src, &'src str, Node<'src>, Extra<'src>>,
{
    let content = node.repeated().at_least(1).collect();
    let class = ascii::ident().to_slice();
    let label = label_parser();
    let body = just(" ")
        .ignore_then(class)
        .or_not()
        .then(just(" ").ignore_then(label).or_not())
        .then_ignore(newline())
        .then(content);

    body.delimited_by(just("["), just("]"))
        .map_with(|((class, label), content), e| Div {
            class,
            content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw<'src> {
    pub content: &'src str,
    pub lang: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn raw_parser<'src>() -> impl Parser<'src, &'src str, Raw<'src>, Extra<'src>> {
    let delim = "```";

    let content = none_of(delim).repeated().to_slice();
    let lang = ascii::ident().to_slice();
    let label = label_parser();

    lang.or_not()
        .then(just(" ").ignore_then(label).or_not())
        .then_ignore(newline())
        .then(content)
        .delimited_by(just(delim), just(delim))
        .map_with(|((lang, label), content), e| Raw {
            lang,
            content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow<'src> {
    pub cells: Vec<Node<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn table_row_parser<'src>() -> impl Parser<'src, &'src str, TableRow<'src>, Extra<'src>> {
    let delim = just("|");
    let label = label_parser();

    let text = text_parser(inline_parser(table_word_parser())).boxed();
    let list_item = list_item_parser(text.clone()).boxed();
    let enum_item = enum_item_parser(text.clone()).boxed();
    let bq_item = block_quote_item_parser(text.clone(), enum_item.clone(), list_item.clone());

    let cell = choice((
        list_item.map(Node::ListItem),
        enum_item.map(Node::EnumItem),
        bq_item.map(Node::BlockQuoteItem),
        text.map(Node::Text),
    ))
    .padded_by(just(" ").repeated());

    cell.separated_by(delim)
        .at_least(1)
        .collect()
        .delimited_by(delim, delim)
        .then(just(" ").ignore_then(label).or_not())
        .map_with(|(cells, label), e| TableRow {
            cells,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'src> {
    pub item: Vec<Inline<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn list_item_parser<'src, T>(
    text: T,
) -> impl Parser<'src, &'src str, ListItem<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>> + 'src,
{
    let label = label_parser();

    // let text = text.map(Node::text);
    // let item = text;

    just("- ")
        .ignore_then(text)
        .then(label.or_not())
        .map_with(|(text, label), e| ListItem {
            item: text.content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem<'src> {
    pub item: Vec<Inline<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn enum_item_parser<'src, T>(
    text: T,
) -> impl Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>> + 'src,
{
    let label = label_parser();

    // let text = text.map(Node::text);
    // let item = text;

    just("+ ")
        .ignore_then(text)
        .then(label.or_not())
        .map_with(|(text, label), e| EnumItem {
            item: text.content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading<'src> {
    pub level: u8,
    pub content: Vec<Inline<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn heading_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, Heading<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>> + 'src,
{
    just("=")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_slice()
        .then_ignore(just(" "))
        .then(text.map(|text| text.content))
        .then(label_parser().or_not())
        .map_with(|((level, content), label), e| Heading {
            level: level.len() as u8,
            content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn text_parser<'src>(
    inline: impl Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
) -> impl Parser<'src, &'src str, Text<'src>, Extra<'src>> {
    inline
        .repeated()
        .at_least(1)
        .collect()
        .map_with(|content, e| Text {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteItem<'src> {
    pub level: u8,
    pub item: Box<Node<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn block_quote_item_parser<'src, T, E, L>(
    text: T,
    enum_item: E,
    list_item: L,
) -> impl Parser<'src, &'src str, BlockQuoteItem<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>>,
    E: Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>,
    L: Parser<'src, &'src str, ListItem<'src>, Extra<'src>>,
{
    let label = label_parser();

    let item = choice((
        list_item.map(Node::ListItem),
        enum_item.map(Node::EnumItem),
        text.map(Node::Text),
    ));

    just(">")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_slice()
        .then_ignore(just(" "))
        .then(item.map(Box::new))
        .then(label.or_not())
        .map_with(|((level, item), label), e| BlockQuoteItem {
            level: level.len() as u8,
            item,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline<'src> {
    Quote(Quote<'src>),
    Strikeout(Strikeout<'src>),
    Emphasis(Emphasis<'src>),
    Strong(Strong<'src>),
    Subscript(Subscript<'src>),
    Supscript(Supscript<'src>),
    Link(Link<'src>),
    Cite(Cite<'src>),
    RawInline(RawInline<'src>),
    Comment(Comment<'src>),
    Escape(Escape<'src>),
    Word(Word<'src>),
    Spacing(Spacing<'src>),
    // not used in cst stage but in ast
    SoftBreak,
}

pub fn inline_parser<'src, W>(
    word: W,
) -> impl Parser<'src, &'src str, Inline<'src>, Extra<'src>> + Clone
where
    W: Parser<'src, &'src str, Word<'src>, Extra<'src>> + 'src,
{
    recursive(|inline| {
        let emphasis_inline = choice((
            subscript_parser(inline.clone()).map(Inline::Subscript),
            supscript_parser(inline.clone()).map(Inline::Supscript),
            link_parser(inline.clone()).map(Inline::Link),
            cite_parser().map(Inline::Cite),
            raw_inline_parser().map(Inline::RawInline),
            comment_parser().map(Inline::Comment),
            escape_parser().map(Inline::Escape),
            word.map(Inline::Word),
            spacing_parser().map(Inline::Spacing),
        ))
        .boxed();

        let emphasis = emphasis_parser(emphasis_inline.clone()).map(Inline::Emphasis);
        let strong = strong_parser(emphasis_inline.clone()).map(Inline::Strong);

        let strikeout_inline = emphasis_inline.or(emphasis).or(strong).boxed();
        let strikeout = strikeout_parser(strikeout_inline.clone()).map(Inline::Strikeout);

        let quote_inline = strikeout_inline.or(strikeout).boxed();
        let quote = quote_parser(quote_inline.clone()).map(Inline::Quote);

        quote_inline.or(quote).boxed()
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn quote_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Quote<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("\""), just("\""))
        .map_with(|content, e| Quote {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strikeout<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn strikeout_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, Strikeout<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("~"), just("~"))
        .map_with(|content, e| Strikeout {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emphasis<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn emphasis_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, Emphasis<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("/"), just("/"))
        .map_with(|content, e| Emphasis {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strong<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn strong_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Strong<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("*"), just("*"))
        .map_with(|content, e| Strong {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn subscript_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, Subscript<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    let c = none_of(SPECIAL).to_slice().map_with(|content, e| {
        vec![Inline::Word(Word {
            content,
            span: e.span(),
        })]
    });
    let content = inline
        .repeated()
        .collect()
        .delimited_by(just("["), just("]"));

    just("_")
        .ignore_then(c.or(content))
        .map_with(|content, e| Subscript {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Supscript<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn supscript_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, Supscript<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    let c = none_of(SPECIAL).to_slice().map_with(|content, e| {
        vec![Inline::Word(Word {
            content,
            span: e.span(),
        })]
    });
    let content = inline
        .repeated()
        .collect()
        .delimited_by(just("["), just("]"));

    just("^")
        .ignore_then(c.or(content))
        .map_with(|content, e| Supscript {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<'src> {
    pub href: &'src str,
    pub content: Option<Vec<Inline<'src>>>,
    pub span: Span,
}

pub fn link_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Link<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    let href = none_of(">")
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_slice();

    let content = inline
        .repeated()
        .collect()
        .delimited_by(just("["), just("]"));

    href.delimited_by(just("<"), just(">"))
        .then(content.or_not())
        .map_with(|(href, content), e| Link {
            href,
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cite<'src> {
    pub ident: &'src str,
    pub span: Span,
}

pub fn cite_parser<'src>() -> impl Parser<'src, &'src str, Cite<'src>, Extra<'src>> {
    just("@")
        .ignore_then(ascii::ident().to_slice())
        .map_with(|ident, e| Cite {
            ident,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escape<'src> {
    pub content: &'src str,
    pub span: Span,
}

pub fn escape_parser<'src>() -> impl Parser<'src, &'src str, Escape<'src>, Extra<'src>> {
    just("\\")
        .ignore_then(one_of(SPECIAL).to_slice())
        .map_with(|content, e| Escape {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawInline<'src> {
    pub content: &'src str,
    pub span: Span,
}

pub fn raw_inline_parser<'src>() -> impl Parser<'src, &'src str, RawInline<'src>, Extra<'src>> {
    let delim = "`";

    let content = none_of("`")
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_slice();

    content
        .delimited_by(just(delim), just(delim))
        .map_with(|content, e| RawInline {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'src> {
    pub content: &'src str,
    pub span: Span,
}

pub fn comment_parser<'src>() -> impl Parser<'src, &'src str, Comment<'src>, Extra<'src>> {
    let content = any()
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_slice();

    just("//")
        .ignore_then(content)
        .map_with(|content, e| Comment {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'src> {
    pub content: &'src str,
    pub span: Span,
}

pub fn word_parser<'src>() -> impl Parser<'src, &'src str, Word<'src>, Extra<'src>> {
    none_of(SPECIAL)
        .repeated()
        .at_least(1)
        .to_slice()
        .map_with(|content, e| Word {
            content,
            span: e.span(),
        })
}

pub fn table_word_parser<'src>() -> impl Parser<'src, &'src str, Word<'src>, Extra<'src>> {
    none_of(format!("|{SPECIAL}"))
        .repeated()
        .at_least(1)
        .to_slice()
        .map_with(|content, e| Word {
            content,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spacing<'src> {
    pub content: &'src str,
    pub span: Span,
}

pub fn spacing_parser<'src>() -> impl Parser<'src, &'src str, Spacing<'src>, Extra<'src>> {
    just(" ")
        .repeated()
        .at_least(1)
        .to_slice()
        .map_with(|content, e| Spacing {
            content,
            span: e.span(),
        })
}

// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> {
    ascii::ident().to_slice().delimited_by(just("{"), just("}"))
}
