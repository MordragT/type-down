use chumsky::{
    prelude::*,
    text::{ascii, newline},
};
use constcat::concat_slices;

use super::{code::code_parser, Extra, Node};
use crate::{ast::*, Span};

pub const SPECIAL: &[char] = &[
    ' ', '\\', '\n', '"', '{', '}', '[', ']', '/', '*', '~', '_', '^', '@', '#', '`', '%', '|',
];

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

pub fn node_parser<'src>() -> impl Parser<'src, &'src str, Node<'src>, Extra<'src>> {
    let text = text_parser(SPECIAL).boxed();
    let list_item = list_item_parser(text.clone()).boxed();
    let enum_item = enum_item_parser(text.clone()).boxed();
    let term_item = term_item(text.clone());
    let table_row = table_row_parser(text.clone(), enum_item.clone(), list_item.clone());

    let heading = heading_parser(text.clone()).map(Node::Heading);
    // let label = label_parser().map(Node::Label);

    choice((
        heading,
        div_parser(text.clone()).map(Node::Div),
        raw_parser().map(Node::Raw),
        table_row.map(Node::TableRow),
        list_item.map(Node::ListItem),
        enum_item.map(Node::EnumItem),
        term_item.map(Node::TermItem),
        newline().to(Node::LineBreak),
        just("    ").to(Node::Indentation),
        text.map(Node::Text),
    ))
}

// div start

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

pub fn table_row_parser<'src, T, E, L>(
    text: T,
    enum_item: E,
    list_item: L,
) -> impl Parser<'src, &'src str, TableRow<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>>,
    E: Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>,
    L: Parser<'src, &'src str, ListItem<'src>, Extra<'src>>,
{
    let delim = just("|");
    let label = label_parser();

    let cell = choice((
        list_item.map(|item| Block::List(item.into())),
        enum_item.map(|item| Block::Enum(item.into())),
        text.map(|text| Block::Paragraph(text.into())),
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

const TERM_SPECIAL: &[char] = concat_slices!([char]: SPECIAL, &[':']);

pub fn term_item<'src, T>(text: T) -> impl Parser<'src, &'src str, TermItem<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>>,
{
    let label = label_parser();
    let term = text_parser(TERM_SPECIAL);

    just("> ")
        .ignore_then(term)
        .then_ignore(just(": "))
        .then(text)
        .then(label.or_not())
        .map_with(|((term, content), label), e| TermItem {
            term,
            content,
            label,
            span: e.span(),
        })
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

pub fn list_item_parser<'src, T>(
    text: T,
) -> impl Parser<'src, &'src str, ListItem<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>> + 'src,
{
    let label = label_parser();

    just("- ")
        .ignore_then(text)
        .then(label.or_not())
        .map_with(|(content, label), e| ListItem {
            content,
            label,
            span: e.span(),
        })
}

pub fn enum_item_parser<'src, T>(
    text: T,
) -> impl Parser<'src, &'src str, EnumItem<'src>, Extra<'src>>
where
    T: Parser<'src, &'src str, Text<'src>, Extra<'src>> + 'src,
{
    let label = label_parser();

    just("+ ")
        .ignore_then(text)
        .then(label.or_not())
        .map_with(|(content, label), e| EnumItem {
            content,
            label,
            span: e.span(),
        })
}

// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> {
    ascii::ident().to_slice().delimited_by(just("{"), just("}"))
}

pub fn text_parser<'src>(
    special: &'src [char],
) -> impl Parser<'src, &'src str, Text<'src>, Extra<'src>> {
    inline_parser(special)
        .repeated()
        .at_least(1)
        .collect()
        .map_with(|content, e| Text {
            content,
            span: e.span(),
        })
}

pub fn inline_parser<'src>(
    special: &'src [char],
) -> impl Parser<'src, &'src str, Inline<'src>, Extra<'src>> + Clone {
    recursive(|inline| {
        choice((
            code_parser(inline.clone()).map(Inline::Code),
            quote_parser(inline.clone()).map(Inline::Quote),
            strikeout_parser(inline.clone()).map(Inline::Strikeout),
            strong_parser(inline.clone()).map(Inline::Strong),
            emphasis_parser(inline.clone()).map(Inline::Emphasis),
            subscript_parser(inline.clone()).map(Inline::Subscript),
            supscript_parser(inline.clone()).map(Inline::Supscript),
            link_parser(inline.clone()).map(Inline::Link),
            cite_parser().map(Inline::Cite),
            raw_inline_parser().map(Inline::RawInline),
            comment_parser().map(Inline::Comment),
            escape_parser().map(Inline::Escape),
            spacing_parser().map(Inline::Spacing),
            word_parser(special).map(Inline::Word),
        ))
        .boxed()
    })
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

pub fn link_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Link<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>>,
{
    let href = none_of(">")
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_slice();

    // let content = inline
    //     .repeated()
    //     .collect()
    //     .delimited_by(just("["), just("]"));

    href.delimited_by(just("<"), just(">"))
        .map_with(|href, e| Link {
            href,
            content: None,
            span: e.span(),
        })
}

pub fn cite_parser<'src>() -> impl Parser<'src, &'src str, Cite<'src>, Extra<'src>> {
    just("@")
        .ignore_then(ascii::ident().to_slice())
        .map_with(|ident, e| Cite {
            ident,
            span: e.span(),
        })
}

pub fn escape_parser<'src>() -> impl Parser<'src, &'src str, Escape<'src>, Extra<'src>> {
    just("\\")
        .ignore_then(one_of(SPECIAL).to_slice())
        .map_with(|content, e| Escape {
            content,
            span: e.span(),
        })
}

pub fn raw_inline_parser<'src>() -> impl Parser<'src, &'src str, RawInline<'src>, Extra<'src>> {
    let delim = "`";

    let content = none_of(delim)
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

pub fn comment_parser<'src>() -> impl Parser<'src, &'src str, Comment<'src>, Extra<'src>> {
    let content = any()
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_slice();

    just("%")
        .ignore_then(content)
        .map_with(|content, e| Comment {
            content,
            span: e.span(),
        })
}

pub fn word_parser<'src>(
    special: &'src [char],
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
