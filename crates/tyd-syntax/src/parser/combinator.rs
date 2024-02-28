use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::SpannedInput, select};

use crate::{ast::*, lexer::Node, Span};

pub type Extra<'tokens, 'src> = extra::Err<Rich<'tokens, Node<'src>, Span>>;
pub type ParserInput<'tokens, 'src> = SpannedInput<Node<'src>, Span, &'tokens [(Node<'src>, Span)]>;

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
    let block_quote = term_parser().map(Block::Term);

    let blocks = choice((block, table, nested, block_quote, paragraph))
        // .or(plain)
        .separated_by(just(Node::LineBreak).repeated().at_least(1))
        .allow_leading()
        .allow_trailing()
        .at_least(1)
        .collect();

    blocks.map(|blocks| Ast { blocks })
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
