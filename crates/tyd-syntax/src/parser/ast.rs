// pub use crate::node::{Div, Heading, Raw};

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::SpannedInput, select};
use miette::NamedSource;

use super::error::{ParseErrors, SyntaxError};
use crate::{
    lexer::{lex_spanned, node::*},
    Span,
};

pub type Extra<'tokens, 'src> = extra::Err<Rich<'tokens, Node<'src>, Span>>;
pub type ParserInput<'tokens, 'src> = SpannedInput<Node<'src>, Span, &'tokens [(Node<'src>, Span)]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast<'src> {
    pub blocks: Vec<Block<'src>>,
}

impl<'src> Ast<'src> {
    pub fn parse<'tokens>(src: &'src str, name: impl AsRef<str>) -> Result<Ast<'src>, SyntaxError> {
        let nodes = lex_spanned(src, name.as_ref())?;
        let input = nodes.as_slice().spanned((src.len()..src.len()).into());

        let parser = ast_parser();
        let ast = parser
            .parse(input)
            .into_result()
            .map_err(|errs| ParseErrors {
                src: NamedSource::new(name, src.to_owned()),
                related: errs.into_iter().map(Into::into).collect(),
            })?;

        Ok(ast)
    }
}

pub fn ast_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Ast<'src>, Extra<'tokens, 'src>> {
    let block = select! {
        Node::Heading(heading) => Block::Heading(heading),
        Node::Raw(raw) => Block::Raw(raw),
        Node::Div(div) => Block::Div(div)
    };

    // let plain = select! { Node::Plain(plain) => Block::Plain(plain) };
    let paragraph = paragraph_parser().map(Block::Paragraph);

    let nested = nested_parser().map(|nested| match nested {
        Nested::List(list) => Block::List(*list),
        Nested::Enum(en) => Block::Enum(*en),
    });

    let table = table_parser().map(Block::Table);
    let block_quote = block_quote_parser().map(Block::BlockQuote);

    let blocks = choice((block, table, nested, block_quote, paragraph))
        // .or(plain)
        .separated_by(just(Node::LineBreak).repeated().at_least(1))
        .allow_leading()
        .allow_trailing()
        .at_least(1)
        .collect();

    blocks.map(|blocks| Ast { blocks })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'src> {
    Div(Div<'src>),
    Raw(Raw<'src>),
    Table(Table<'src>),
    List(List<'src>),
    Enum(Enum<'src>),
    BlockQuote(BlockQuote<'src>),
    Heading(Heading<'src>),
    Paragraph(Paragraph<'src>),
    // Plain(Plain<'src>),
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Div<'src> {
//     pub content: Vec<Block<'src>>,
//     pub class: Option<&'src str>,
//     pub label: Option<&'src str>,
//     pub span: Span,
// }

// pub fn div_parser<'tokens, 'src: 'tokens, B>(
//     blocks: B,
// ) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Div<'src>, Extra<'tokens, 'src>>
// where
//     B: Parser<'tokens, ParserInput<'tokens, 'src>, Vec<Block<'src>>, Extra<'tokens, 'src>>,
// {
//     let nodes = select! { Node::Div(div) => div.content };

//     blocks.nested_in(nodes).map_with(|content, e| Div {
//         content,
//         class: None,
//         label: None,
//         span: e.span(),
//     })
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'src> {
    pub col_count: usize,
    pub rows: Vec<TableRow<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn table_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Table<'src>, Extra<'tokens, 'src>> {
    let table_row = select! { Node::TableRow(row) => row };

    table_row
        .separated_by(just(Node::LineBreak))
        .at_least(1)
        .collect()
        .map_with(|rows: Vec<TableRow<'_>>, e| {
            let col_count = rows[0].cells.len();

            let table = Table {
                col_count,
                rows,
                label: None,
                span: e.span(),
            };
            (table, col_count)
        })
        .validate(|(table, col_count), e, emitter| {
            if table.rows.iter().any(|row| row.cells.len() != col_count) {
                emitter.emit(Rich::custom(
                    e.span(),
                    "Adjacent table rows must contain equal number of cells.",
                ))
            }
            table
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'src> {
    pub head: Vec<ListItem<'src>>,
    pub body: Option<Nested<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

impl<'src> From<ListItem<'src>> for List<'src> {
    fn from(value: ListItem<'src>) -> Self {
        let head = vec![value.clone()];
        let ListItem {
            content: _,
            label,
            span,
        } = value;

        Self {
            head,
            body: None,
            label,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum<'src> {
    pub head: Vec<EnumItem<'src>>,
    pub body: Option<Nested<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

impl<'src> From<EnumItem<'src>> for Enum<'src> {
    fn from(value: EnumItem<'src>) -> Self {
        let head = vec![value.clone()];
        let EnumItem {
            content: _,
            label,
            span,
        } = value;

        Self {
            head,
            body: None,
            label,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nested<'src> {
    List(Box<List<'src>>),
    Enum(Box<Enum<'src>>),
}

pub fn nested_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Nested<'src>, Extra<'tokens, 'src>> {
    let list_item = select! { Node::ListItem(item) => item };
    let enum_item = select! { Node::EnumItem(item) => item };

    let nested = recursive(|block| {
        let indent = just(Node::Indentation)
            .repeated()
            .configure(|cfg, parent_indent| cfg.exactly(*parent_indent));

        let list_items = indent
            .clone()
            .ignore_then(list_item)
            .separated_by(just(Node::LineBreak))
            .at_least(1)
            .collect();

        let enum_items = indent
            .ignore_then(enum_item)
            .separated_by(just(Node::LineBreak))
            .at_least(1)
            .collect();

        let body = just(Node::LineBreak)
            .ignore_then(
                just(Node::Indentation)
                    .repeated()
                    .configure(|cfg, parent_indent| cfg.exactly(*parent_indent + 1))
                    .count()
                    .rewind()
                    .ignore_with_ctx(block),
            )
            .or_not();

        let list = list_items
            .then(body.clone())
            .map_with(|(head, body), e| List {
                head,
                body,
                label: None,
                span: e.span(),
            });

        let enumeration = enum_items.then(body).map_with(|(items, body), e| Enum {
            head: items,
            body,
            label: None,
            span: e.span(),
        });

        list.map(|list| Nested::List(Box::new(list)))
            .or(enumeration.map(|en| Nested::Enum(Box::new(en))))
    });

    nested.with_ctx(0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuote<'src> {
    pub level: u8,
    pub content: Vec<BlockQuoteItem<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

pub fn block_quote_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, BlockQuote<'src>, Extra<'tokens, 'src>> {
    let item = select! { Node::BlockQuoteItem(item) => item };

    item.separated_by(just(Node::LineBreak))
        .at_least(1)
        .collect()
        .map_with(|content: Vec<BlockQuoteItem<'_>>, e| {
            let level = content[0].level;

            let bq = BlockQuote {
                content,
                level,
                label: None,
                span: e.span(),
            };
            (bq, level)
        })
        .validate(|(bq, level), e, emitter| {
            if bq.content.iter().any(|item| item.level != level) {
                emitter.emit(Rich::custom(
                    e.span(),
                    "Adjacent BlockQuotes mut be of the same level.",
                ))
            }
            bq
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

impl<'src> From<Text<'src>> for Paragraph<'src> {
    fn from(value: Text<'src>) -> Self {
        let Text { content, span } = value;

        Self { content, span }
    }
}

pub fn paragraph_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Paragraph<'src>, Extra<'tokens, 'src>> {
    let plain = select! { Node::Text(plain) => plain };

    plain
        .separated_by(just(Node::LineBreak))
        .collect()
        .map(|contents: Vec<Text<'_>>| {
            contents
                .into_iter()
                .flat_map(|plain| {
                    plain
                        .content
                        .into_iter()
                        .chain(std::iter::once(Inline::SoftBreak))
                })
                .collect::<Vec<_>>()
        })
        // .then_ignore(just([Node::LineBreak, Node::LineBreak]))
        .map_with(|content, e| Paragraph {
            content,
            span: e.span(),
        })
}
