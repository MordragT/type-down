use core::fmt;

use chumsky::{
    prelude::*,
    text::{ascii, newline},
};
use miette::NamedSource;

use crate::{
    error::{ParseError, TydError},
    inline::{inline_parser, table_word_parser, word_parser, Inline},
    Span,
};

type Extra<'src> = extra::Err<Rich<'src, char, Span>>;

pub fn parse<'src>(src: &'src str, name: impl AsRef<str>) -> Result<Vec<Node<'src>>, TydError> {
    let parser = nodes_parser();
    let cst = parser
        .parse(&src)
        .into_result()
        .map_err(|errs| ParseError {
            src: NamedSource::new(name, src.to_owned()),
            related: errs.into_iter().map(Into::into).collect(),
        })?;

    Ok(cst)
}

pub fn parse_spanned<'src>(
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

pub fn nodes_parser<'src>() -> impl Parser<'src, &'src str, Vec<Node<'src>>, Extra<'src>> {
    let node = node_parser();

    node.repeated().at_least(1).collect().then_ignore(end())
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
    Heading(Heading<'src>),
    Div(Div<'src>),
    Raw(Raw<'src>),
    TableRow(TableRow<'src>),
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    BlockQuoteItem(BlockQuoteElement<'src>),
    Text(Text<'src>),
    // Label(&'src str),
    LineBreak,
    Indentation,
}

impl<'src> fmt::Display for Node<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Heading(_) => f.write_str("Heading"),
            Self::Div(_) => f.write_str("Div"),
            Self::Raw(_) => f.write_str("Raw"),
            Self::TableRow(_) => f.write_str("TableRow"),
            Self::ListItem(_) => f.write_str("ListItem"),
            Self::EnumItem(_) => f.write_str("EnumItem"),
            Self::BlockQuoteItem(_) => f.write_str("BlockQuoteItem"),
            Self::Text(_) => f.write_str("Text"),
            Self::LineBreak => f.write_str("LineBreak"),
            Self::Indentation => f.write_str("Indentation"),
        }
    }
}

pub fn node_parser<'src>() -> impl Parser<'src, &'src str, Node<'src>, Extra<'src>> {
    // recursive(|node| {
    let inline = inline_parser(word_parser());

    let text = text_parser(inline).boxed();
    let list_item = list_item_parser(text.clone()).boxed();
    let enum_item = enum_item_parser(text.clone()).boxed();
    let bq_item = block_quote_element_parser(text.clone(), enum_item.clone(), list_item.clone());

    let heading = heading_parser(text.clone()).map(Node::Heading);
    // let label = label_parser().map(Node::Label);

    choice((
        heading,
        div_parser(text.clone()).map(Node::Div),
        raw_parser().map(Node::Raw),
        table_row_parser().map(Node::TableRow),
        list_item.map(Node::ListItem),
        enum_item.map(Node::EnumItem),
        bq_item.map(Node::BlockQuoteItem),
        newline().to(Node::LineBreak),
        just("    ").to(Node::Indentation),
        text.map(Node::Text),
    ))
    // .boxed()
    // })
}

// div start

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div<'src> {
    pub content: Text<'src>,
    pub class: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn div_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, Div<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>>,
{
    let class = ascii::ident().to_slice();
    let label = label_parser();
    let body = just(" ")
        .ignore_then(class)
        .or_not()
        .then(just(" ").ignore_then(label).or_not())
        .then_ignore(newline())
        .then(text);

    body.delimited_by(just("["), just("]"))
        .map_with(|((class, label), content), e| Div {
            class,
            content,
            label,
            span: e.span(),
        })
}

// pub fn div_parser<'src, N>(node: N) -> impl Parser<'src, &'src str, Div<'src>, Extra<'src>>
// where
//     N: Parser<'src, &'src str, Node<'src>, Extra<'src>>,
// {
//     let content = node.repeated().at_least(1).collect();
//     let class = ascii::ident().to_slice();
//     let label = label_parser();
//     let body = just(" ")
//         .ignore_then(class)
//         .or_not()
//         .then(just(" ").ignore_then(label).or_not())
//         .then_ignore(newline())
//         .then(content);

//     body.delimited_by(just("["), just("]"))
//         .map_with(|((class, label), content), e| Div {
//             class,
//             content,
//             label,
//             span: e.span(),
//         })
// }

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
pub struct Heading<'src> {
    pub level: u8,
    pub content: Text<'src>,
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
        .then(text)
        .then(label_parser().or_not())
        .map_with(|((level, content), label), e| Heading {
            level: level.len() as u8,
            content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow<'src> {
    pub cells: Vec<TableCell<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn table_row_parser<'src>() -> impl Parser<'src, &'src str, TableRow<'src>, Extra<'src>> {
    let delim = just("|");
    let label = label_parser();

    let text = text_parser(inline_parser(table_word_parser())).boxed();
    let list_item = list_item_parser(text.clone()).boxed();
    let enum_item = enum_item_parser(text.clone()).boxed();
    let bq_element = block_quote_element_parser(text.clone(), enum_item.clone(), list_item.clone());

    let cell = choice((
        list_item.map(TableCell::ListItem),
        enum_item.map(TableCell::EnumItem),
        bq_element.map(TableCell::BlockQuoteElement),
        text.map(TableCell::Text),
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
pub enum TableCell<'src> {
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    BlockQuoteElement(BlockQuoteElement<'src>),
    Text(Text<'src>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'src> {
    pub content: Text<'src>,
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
        .map_with(|(content, label), e| ListItem {
            content,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem<'src> {
    pub content: Text<'src>,
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
        .map_with(|(content, label), e| EnumItem {
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
pub struct BlockQuoteElement<'src> {
    pub level: u8,
    pub item: BlockQuoteItem<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn block_quote_element_parser<'src, T, E, L>(
    text: T,
    enum_item: E,
    list_item: L,
) -> impl Parser<'src, &'src str, BlockQuoteElement<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>>,
    E: Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>,
    L: Parser<'src, &'src str, ListItem<'src>, Extra<'src>>,
{
    let label = label_parser();

    let item = choice((
        list_item.map(BlockQuoteItem::ListItem),
        enum_item.map(BlockQuoteItem::EnumItem),
        text.map(BlockQuoteItem::Text),
    ));

    just(">")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_slice()
        .then_ignore(just(" "))
        .then(item)
        .then(label.or_not())
        .map_with(|((level, item), label), e| BlockQuoteElement {
            level: level.len() as u8,
            item,
            label,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockQuoteItem<'src> {
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    Text(Text<'src>),
}

// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> {
    ascii::ident().to_slice().delimited_by(just("{"), just("}"))
}
