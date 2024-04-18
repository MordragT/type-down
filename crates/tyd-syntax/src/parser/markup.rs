use chumsky::{
    prelude::*,
    text::{ascii, newline},
};
use constcat::concat_slices;
use ecow::EcoString;

use super::{code::code_parser, Extra, ParserContext, ParserExt};
use crate::ast::*;

pub const SPECIAL: &[char] = &[
    ' ', '\\', '\n', '"', '{', '}', '[', ']', '/', '*', '~', '_', '^', '@', '#', '`', '$', '%', '|',
];

pub fn soft_break_parser<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    newline().repeated().exactly(1)
}

pub fn hard_break_parser<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    newline().repeated().at_least(2)
}

pub fn indent_parser<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    let indent = just("    ").or(just("\t"));

    soft_break_parser().ignore_then(
        indent
            .repeated()
            .configure(|cfg, ctx: &ParserContext| cfg.exactly(ctx.indent)),
    )
}

// pub fn dedent_parser<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
//     let indent = just("    ").or(just("\t"));

//     soft_break_parser()
//         .ignore_then(
//             indent
//                 .repeated()
//                 .configure(|cfg, ctx: &ParserContext| cfg.at_most(ctx.indent - 1)),
//         )
//         .or(hard_break_parser())
// }

pub fn block_parser<'src>() -> impl Parser<'src, &'src str, Block, Extra<'src>> {
    let text = text_parser(SPECIAL).boxed();
    // let div = div_parser(block).boxed();
    let list_item = list_item_parser(text.clone()).boxed();
    let enum_item = enum_item_parser(text.clone()).boxed();

    choice((
        heading_parser(text.clone()).map(Block::Heading),
        // div.map(Block::Div),
        table_parser(table_row_parser(
            text.clone(),
            enum_item.clone(),
            list_item.clone(),
        ))
        .map(Block::Table),
        term_parser(term_item_parser(text.clone())).map(Block::Term),
        list_parser(list_item).map(Block::List),
        enum_parser(enum_item).map(Block::Enum),
        raw_parser().map(Block::Raw),
        paragraph_parser(text.clone()).map(Block::Paragraph),
    ))
    .with_ctx(ParserContext { indent: 0 })
}

// pub fn div_parser<'src, B>(block: B) -> impl Parser<'src, &'src str, Div, Extra<'src>>
// where
//     B: Parser<'src, &'src str, Block, Extra<'src>>,
// {
//     indent_parser()
//         .ignore_then(block)
//         .repeated()
//         .at_least(1)
//         .collect()
//         .then_ignore(dedent_parser())
//         .map_with(|content, e| Div {
//             content,
//             class: None,
//             label: None,
//             span: e.span(),
//         })
// }

pub fn paragraph_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, Paragraph, Extra<'src>>
where
    T: Parser<'src, &'src str, Text, Extra<'src>> + 'src,
{
    text.map(|text| text.content)
        .separated_by(soft_break_parser())
        .at_least(1)
        .collect()
        .map_with(|mut content: Vec<Vec<_>>, e| {
            for line in &mut content {
                line.push(Inline::SoftBreak);
            }

            Paragraph {
                content: content.into_iter().flatten().collect(),
                span: e.span(),
            }
        })
}

pub fn heading_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, Heading, Extra<'src>>
where
    T: Parser<'src, &'src str, Text, Extra<'src>> + 'src,
{
    let marker = just("=")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_ecow()
        .then_ignore(just(" "));

    group((marker, text, label_parser().or_not())).map_with(|(level, content, label), e| Heading {
        level: level.len() as u8,
        content,
        label,
        span: e.span(),
    })
}

pub fn raw_parser<'src>() -> impl Parser<'src, &'src str, Raw, Extra<'src>> {
    let delim = "```";

    group((
        ascii::ident().to_ecow().or_not(),
        just(" ").ignore_then(label_parser()).or_not(),
        none_of(delim).repeated().at_least(1).to_ecow(),
    ))
    .delimited_by(just(delim), just(delim))
    .map_with(|(lang, label, content), e| Raw {
        lang,
        content,
        label,
        span: e.span(),
    })
}

pub fn table_parser<'src, R>(table_row: R) -> impl Parser<'src, &'src str, Table, Extra<'src>>
where
    R: Parser<'src, &'src str, TableRow, Extra<'src>>,
{
    table_row
        .separated_by(soft_break_parser())
        .at_least(1)
        .collect()
        .map_with(|rows: Vec<TableRow>, e| {
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

pub fn table_row_parser<'src, T, E, L>(
    text: T,
    enum_item: E,
    list_item: L,
) -> impl Parser<'src, &'src str, TableRow, Extra<'src>>
where
    T: Parser<'src, &'src str, Text, Extra<'src>>,
    E: Parser<'src, &'src str, EnumItem, Extra<'src>>,
    L: Parser<'src, &'src str, ListItem, Extra<'src>>,
{
    let delim = just("|");
    let label = label_parser();

    let cell = choice((
        list_item.map(|item| Block::List(item.into())),
        enum_item.map(|item| Block::Enum(item.into())),
        text.map(|text| Block::Plain(text.into())),
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

pub fn term_parser<'src, I>(item: I) -> impl Parser<'src, &'src str, Term, Extra<'src>>
where
    I: Parser<'src, &'src str, TermItem, Extra<'src>>,
{
    item.separated_by(soft_break_parser())
        .at_least(1)
        .collect()
        .map_with(|content, e| Term {
            content,
            label: None,
            span: e.span(),
        })
}

const TERM_SPECIAL: &[char] = concat_slices!([char]: SPECIAL, &[':']);

pub fn term_item_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, TermItem, Extra<'src>>
where
    T: Parser<'src, &'src str, Text, Extra<'src>>,
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

pub fn list_parser<'src, I>(list_item: I) -> impl Parser<'src, &'src str, List, Extra<'src>>
where
    I: Parser<'src, &'src str, ListItem, Extra<'src>> + Clone + 'src,
{
    recursive(
        |list: Recursive<dyn Parser<&'src str, List, Extra<'src>>>| {
            let indent = indent_parser();
            let nested = soft_break_parser().ignore_then(
                just("    ")
                    .or(just("\t"))
                    .repeated()
                    .configure(|cfg, ctx: &ParserContext| cfg.exactly(ctx.indent + 1))
                    .count()
                    .map(|indent| ParserContext { indent })
                    .ignore_with_ctx(list),
            );

            let item = list_item.then(nested.or_not()).map(|(mut item, nested)| {
                if let Some(nested) = nested {
                    item.content.push(Block::List(nested));
                }

                item
            });

            item.separated_by(indent)
                .at_least(1)
                .collect()
                .map_with(|items, e| List {
                    items,
                    label: None,
                    span: e.span(),
                })
                .boxed()
        },
    )
}

pub fn list_item_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, ListItem, Extra<'src>>
where
    T: Parser<'src, &'src str, Text, Extra<'src>> + 'src,
{
    let label = label_parser();

    just("- ")
        .ignore_then(text)
        .then(label.or_not())
        .map_with(|(content, label), e| ListItem {
            content: vec![Block::Plain(content.into())],
            label,
            span: e.span(),
        })
}

pub fn enum_parser<'src, I>(enum_item: I) -> impl Parser<'src, &'src str, Enum, Extra<'src>>
where
    I: Parser<'src, &'src str, EnumItem, Extra<'src>> + Clone + 'src,
{
    recursive(
        |list: Recursive<dyn Parser<&'src str, Enum, Extra<'src>>>| {
            let indent = indent_parser();
            let nested = soft_break_parser().ignore_then(
                just("    ")
                    .or(just("\t"))
                    .repeated()
                    .configure(|cfg, ctx: &ParserContext| cfg.exactly(ctx.indent + 1))
                    .count()
                    .map(|indent| ParserContext { indent })
                    .ignore_with_ctx(list),
            );

            let item = enum_item.then(nested.or_not()).map(|(mut item, nested)| {
                if let Some(nested) = nested {
                    item.content.push(Block::Enum(nested));
                }

                item
            });

            item.separated_by(indent)
                .at_least(1)
                .collect()
                .map_with(|items, e| Enum {
                    items,
                    label: None,
                    span: e.span(),
                })
                .boxed()
        },
    )
}

pub fn enum_item_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, EnumItem, Extra<'src>>
where
    T: Parser<'src, &'src str, Text, Extra<'src>> + 'src,
{
    let label = label_parser();

    just("+ ")
        .ignore_then(text)
        .then(label.or_not())
        .map_with(|(content, label), e| EnumItem {
            content: vec![Block::Plain(content.into())],
            label,
            span: e.span(),
        })
}
// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, EcoString, Extra<'src>> {
    ascii::ident().to_ecow().delimited_by(just("{"), just("}"))
}

// TODO allow softbreak then indent in text_parser

pub fn text_parser<'src>(special: &'src [char]) -> impl Parser<'src, &'src str, Text, Extra<'src>> {
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
) -> impl Parser<'src, &'src str, Inline, Extra<'src>> + Clone {
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
            math_inline_parser().map(Inline::MathInline),
            comment_parser().map(Inline::Comment),
            escape_parser().map(Inline::Escape),
            spacing_parser().map(Inline::Spacing),
            word_parser(special).map(Inline::Word),
        ))
        .boxed()
    })
}

pub fn quote_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Quote, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
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

pub fn strikeout_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Strikeout, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
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

pub fn emphasis_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Emphasis, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
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

pub fn strong_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Strong, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
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

pub fn subscript_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Subscript, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
{
    let c = none_of(SPECIAL).to_ecow().map_with(|content, e| {
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

pub fn supscript_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Supscript, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
{
    let c = none_of(SPECIAL).to_ecow().map_with(|content, e| {
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

pub fn link_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Link, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>>,
{
    let href = none_of(">")
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_ecow()
        .delimited_by(just("<"), just(">"));

    let content = inline
        .repeated()
        .collect()
        .delimited_by(just("["), just("]"));

    href.then(content.or_not())
        .map_with(|(href, content), e| Link {
            href,
            content,
            span: e.span(),
        })
}

pub fn cite_parser<'src>() -> impl Parser<'src, &'src str, Cite, Extra<'src>> {
    just("@")
        .ignore_then(ascii::ident().to_ecow())
        .map_with(|ident, e| Cite {
            ident,
            span: e.span(),
        })
}

pub fn escape_parser<'src>() -> impl Parser<'src, &'src str, Escape, Extra<'src>> {
    just("\\")
        .ignore_then(one_of(SPECIAL).to_ecow())
        .map_with(|content, e| Escape {
            content,
            span: e.span(),
        })
}

pub fn raw_inline_parser<'src>() -> impl Parser<'src, &'src str, RawInline, Extra<'src>> {
    let delim = "`";

    let content = none_of(delim)
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_ecow();

    content
        .delimited_by(just(delim), just(delim))
        .map_with(|content, e| RawInline {
            content,
            span: e.span(),
        })
}

pub fn math_inline_parser<'src>() -> impl Parser<'src, &'src str, MathInline, Extra<'src>> {
    let delim = "$";

    let content = none_of(delim)
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_ecow();

    content
        .delimited_by(just(delim), just(delim))
        .map_with(|content, e| MathInline {
            content,
            span: e.span(),
        })
}

pub fn comment_parser<'src>() -> impl Parser<'src, &'src str, Comment, Extra<'src>> {
    let content = any()
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_ecow();

    just("%")
        .ignore_then(content)
        .map_with(|content, e| Comment {
            content,
            span: e.span(),
        })
}

pub fn word_parser<'src>(special: &'src [char]) -> impl Parser<'src, &'src str, Word, Extra<'src>> {
    none_of(special)
        .repeated()
        .at_least(1)
        .to_ecow()
        .map_with(|content, e| Word {
            content,
            span: e.span(),
        })
}

pub fn spacing_parser<'src>() -> impl Parser<'src, &'src str, Spacing, Extra<'src>> {
    just(" ")
        .repeated()
        .at_least(1)
        .to_ecow()
        .map_with(|content, e| Spacing {
            content,
            span: e.span(),
        })
}
