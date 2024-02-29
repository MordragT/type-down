use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::SpannedInput, select};

use crate::{ast::*, lexer::node::Node, Span};

pub type Extra<'tokens, 'src> = extra::Err<Rich<'tokens, Node<'src>, Span>>;
pub type ParserInput<'tokens, 'src> = SpannedInput<Node<'src>, Span, &'tokens [(Node<'src>, Span)]>;

pub fn ast_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Ast<'src>, Extra<'tokens, 'src>> {
    let blocks = blocks_parser();

    blocks.map(|blocks| Ast { blocks })
}

pub fn blocks_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Vec<Block<'src>>, Extra<'tokens, 'src>> {
    let simple_block = select! {
        Node::Heading(heading) => Block::Heading(heading),
        Node::Raw(raw) => Block::Raw(raw),
    };

    let blocks = recursive(|blocks| {
        let div = div_parser(blocks).boxed();

        let block = choice((
            simple_block,
            list_parser(div.clone()).map(Block::List),
            enum_parser(div.clone()).map(Block::Enum),
            div.map(Block::Div),
            table_parser().map(Block::Table),
            term_parser().map(Block::Term),
            paragraph_parser(),
        ));

        block
            // .or(plain)
            .separated_by(just(Node::LineBreak).repeated().at_least(1))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect()
            .boxed()
    });

    blocks
}

pub fn div_parser<'tokens, 'src: 'tokens, B>(
    blocks: B,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Div<'src>, Extra<'tokens, 'src>>
where
    B: Parser<'tokens, ParserInput<'tokens, 'src>, Vec<Block<'src>>, Extra<'tokens, 'src>>,
{
    just(Node::Indent)
        .ignore_then(blocks)
        .then_ignore(just(Node::Dedent))
        .map_with(|content, e| Div {
            content,
            class: None,
            label: None,
            span: e.span(),
        })
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

pub fn list_parser<'tokens, 'src: 'tokens, D>(
    div: D,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, List<'src>, Extra<'tokens, 'src>>
where
    D: Parser<'tokens, ParserInput<'tokens, 'src>, Div<'src>, Extra<'tokens, 'src>>
        + Clone
        + 'tokens,
{
    let list_item = select! { Node::ListItem(item) => item };
    let head = list_item
        .separated_by(just(Node::LineBreak))
        .at_least(1)
        .collect();

    recursive(|list| {
        head.then(div.or_not()).then(list.or_not()).map_with(
            |((head, div), tail): ((_, _), Option<List>), e| {
                let mut items: Vec<ListItem> = head;
                if let Some(mut body) = div {
                    let tail = items.last_mut().unwrap();
                    tail.content.append(&mut body.content)
                }
                if let Some(mut tail) = tail {
                    items.append(&mut tail.items);
                }

                List {
                    items,
                    label: None,
                    span: e.span(),
                }
            },
        )
    })
}

pub fn enum_parser<'tokens, 'src: 'tokens, D>(
    div: D,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Enum<'src>, Extra<'tokens, 'src>>
where
    D: Parser<'tokens, ParserInput<'tokens, 'src>, Div<'src>, Extra<'tokens, 'src>>
        + Clone
        + 'tokens,
{
    let enum_item = select! { Node::EnumItem(item) => item };
    let head = enum_item
        .separated_by(just(Node::LineBreak))
        .at_least(1)
        .collect();

    recursive(|enumeration| {
        head.then(div.or_not()).then(enumeration.or_not()).map_with(
            |((head, div), tail): ((_, _), Option<Enum>), e| {
                let mut items: Vec<EnumItem> = head;
                if let Some(mut body) = div {
                    let tail = items.last_mut().unwrap();
                    tail.content.append(&mut body.content)
                }
                if let Some(mut tail) = tail {
                    items.append(&mut tail.items);
                }

                Enum {
                    items,
                    label: None,
                    span: e.span(),
                }
            },
        )
    })
}

pub fn term_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Term<'src>, Extra<'tokens, 'src>> {
    let item = select! { Node::TermItem(item) => item };

    item.separated_by(just(Node::LineBreak))
        .at_least(1)
        .collect()
        .map_with(|content, e| Term {
            content,
            label: None,
            span: e.span(),
        })
}

pub fn paragraph_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Block<'src>, Extra<'tokens, 'src>> {
    let text = select! { Node::Text(text) => text };

    text.separated_by(just(Node::LineBreak))
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
        .then(just([Node::LineBreak, Node::LineBreak]).or_not().rewind())
        .map_with(|(content, par_break), e| {
            if par_break.is_some() {
                Block::Paragraph(Paragraph {
                    content,
                    span: e.span(),
                })
            } else {
                Block::Plain(Plain {
                    content,
                    span: e.span(),
                })
            }
        })
}
