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
pub const SPECIAL: &str = " \\\n\"{}[]/*~_^@#`";

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
    Plain(Plain<'src>),
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
            Self::Plain(_) => f.write_str("Plain"),
            Self::LineBreak => f.write_str("LineBreak"),
            Self::Indentation => f.write_str("Indentation"),
        }
    }
}

pub fn node_parser<'src>() -> impl Parser<'src, &'src str, Node<'src>, Extra<'src>> {
    let inline = inline_parser(SPECIAL).boxed();
    let plain = plain_parser(inline.clone()).boxed();
    let list_item = list_item_parser(plain.clone()).boxed();
    let enum_item = enum_item_parser(plain.clone()).boxed();
    let bq_item = block_quote_item_parser(plain.clone(), enum_item.clone(), list_item.clone());

    let div = div_parser().map(Node::Div);
    let raw = raw_parser().map(Node::Raw);
    let table_row = table_row_parser().map(Node::TableRow);
    let list_item = list_item.map(Node::ListItem);
    let enum_item = enum_item.map(Node::EnumItem);
    let bq_item = bq_item.map(Node::BlockQuoteItem);
    let heading = heading_parser(inline).map(Node::Heading);
    let line_break = newline().to(Node::LineBreak);
    let indentation = just("    ").to(Node::Indentation);
    let plain = plain.map(Node::Plain);
    // let label = label_parser().map(Node::Label);

    div.or(raw)
        .or(table_row)
        .or(list_item)
        .or(enum_item)
        .or(bq_item)
        .or(heading)
        .or(line_break)
        .or(indentation)
        .or(plain)
    // .or(label)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div<'src> {
    pub content: &'src str,
    pub class: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn div_parser<'src>() -> impl Parser<'src, &'src str, Div<'src>, Extra<'src>> {
    let delim = ":::";

    let content = none_of(delim).repeated().to_slice();
    let class = ascii::ident().to_slice();
    let label = label_parser();

    class
        .or_not()
        .then(just(" ").ignore_then(label).or_not())
        .then_ignore(newline())
        .then(content)
        .delimited_by(just(delim), just(delim))
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
    let special = "| \\\n\"{}[]/*~_^@#`";
    let delim = just("|");
    let label = label_parser();

    let plain = plain_parser(inline_parser(special)).boxed();
    let list_item = list_item_parser(plain.clone()).boxed();
    let enum_item = enum_item_parser(plain.clone()).boxed();
    let bq_item = block_quote_item_parser(plain.clone(), enum_item.clone(), list_item.clone());

    let plain = plain.map(Node::Plain);
    let list_item = list_item.map(Node::ListItem);
    let enum_item = enum_item.map(Node::EnumItem);
    let bq_item = bq_item.map(Node::BlockQuoteItem);

    let cell = list_item
        .or(enum_item)
        .or(bq_item)
        .or(plain)
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

pub fn list_item_parser<'src, P>(
    plain: P,
) -> impl Parser<'src, &'src str, ListItem<'src>, Extra<'src>>
where
    P: Parser<'src, &'src str, Plain<'src>, Extra<'src>> + 'src,
{
    let label = label_parser();

    // let plain = plain.map(Node::Plain);
    // let item = plain;

    just("- ")
        .ignore_then(plain)
        .then(label.or_not())
        .map_with(|(plain, label), e| ListItem {
            item: plain.content,
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

pub fn enum_item_parser<'src, P>(
    plain: P,
) -> impl Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>
where
    P: Parser<'src, &'src str, Plain<'src>, Extra<'src>> + 'src,
{
    let label = label_parser();

    // let plain = plain.map(Node::Plain);
    // let item = plain;

    just("+ ")
        .ignore_then(plain)
        .then(label.or_not())
        .map_with(|(plain, label), e| EnumItem {
            item: plain.content,
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

pub fn heading_parser<'src>(
    inline_parser: impl Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
) -> impl Parser<'src, &'src str, Heading<'src>, Extra<'src>> {
    just("=")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_slice()
        .then_ignore(just(" "))
        .then(inline_parser.repeated().at_least(1).collect())
        .then(label_parser().or_not())
        .map_with(|((level, content), label), e| Heading {
            level: level.len() as u8,
            content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plain<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

pub fn plain_parser<'src>(
    inline_parser: impl Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
) -> impl Parser<'src, &'src str, Plain<'src>, Extra<'src>> {
    inline_parser
        .repeated()
        .at_least(1)
        .collect()
        .map_with(|content, e| Plain {
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

pub fn block_quote_item_parser<'src, P, E, L>(
    plain: P,
    enum_item: E,
    list_item: L,
) -> impl Parser<'src, &'src str, BlockQuoteItem<'src>, Extra<'src>>
where
    P: Parser<'src, &'src str, Plain<'src>, Extra<'src>>,
    E: Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>,
    L: Parser<'src, &'src str, ListItem<'src>, Extra<'src>>,
{
    let label = label_parser();

    let plain = plain.map(Node::Plain);
    let enum_item = enum_item.map(Node::EnumItem);
    let list_item = list_item.map(Node::ListItem);

    let item = list_item.or(enum_item).or(plain);

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

pub fn inline_parser<'src>(
    special: &'src str,
) -> impl Parser<'src, &'src str, Inline<'src>, Extra<'src>> + Clone {
    recursive(|inline| {
        let subscript = subscript_parser(inline.clone()).map(Inline::Subscript);
        let supscript = supscript_parser(inline.clone()).map(Inline::Supscript);
        let link = link_parser(inline.clone()).map(Inline::Link);
        let cite = cite_parser().map(Inline::Cite);
        let raw_inline = raw_inline_parser().map(Inline::RawInline);
        let comment = comment_parser().map(Inline::Comment);
        let escape = escape_parser().map(Inline::Escape);
        let word = word_parser(special).map(Inline::Word);
        let spacing = spacing_parser().map(Inline::Spacing);

        let emphasis_inline = subscript
            .or(supscript)
            .or(link)
            .or(cite)
            .or(raw_inline)
            .or(comment)
            .or(escape)
            .or(word)
            .or(spacing)
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

pub fn word_parser<'src>(
    special: &'src str,
) -> impl Parser<'src, &'src str, Word<'src>, Extra<'src>> {
    none_of(special)
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

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> {
    ascii::ident().to_slice().delimited_by(just("{"), just("}"))
}
