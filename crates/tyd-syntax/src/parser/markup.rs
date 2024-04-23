use chumsky::{
    prelude::*,
    text::{ascii, newline},
};
use constcat::concat_slices;

use super::{code::code_parser, Extra, ParserContext, ParserExt};
use crate::{ast::*, Span};

pub const SPECIAL: &[char] = &[
    ' ', '\\', '\n', '"', '{', '}', '[', ']', '/', '*', '~', '_', '^', '@', '#', '`', '$', '%', '|',
];

pub fn soft_break_parser<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    newline().repeated().exactly(1)
}

pub fn hard_break_parser<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    newline().repeated().at_least(2)
}

pub fn level_parser<'src>() -> impl Parser<'src, &'src str, usize, Extra<'src>> {
    let indent = just("    ").or(just("\t"));

    soft_break_parser().ignore_then(
        indent
            .repeated()
            .configure(|cfg, ctx: &ParserContext| cfg.exactly(ctx.indent))
            .count(),
    )
}

pub fn indent_parser<'src>() -> impl Parser<'src, &'src str, usize, Extra<'src>> {
    let indent = just("    ").or(just("\t"));

    soft_break_parser().ignore_then(
        indent
            .repeated()
            .configure(|cfg, ctx: &ParserContext| cfg.exactly(ctx.indent + 1))
            .count(),
    )
}

pub fn block_parser<'src>() -> impl Parser<'src, &'src str, Block, Extra<'src>> {
    let text = text_parser(SPECIAL).boxed();
    let paragraph = paragraph_parser(text.clone()).boxed();
    let list_item = list_item_parser(text.clone()).boxed();
    let enum_item = enum_item_parser(text.clone()).boxed();
    let term_item = term_item_parser(paragraph.clone());
    let table_row = table_row_parser(text.clone(), enum_item.clone(), list_item.clone());

    choice((
        heading_parser(paragraph.clone()).map(Block::Heading),
        table_parser(table_row).map(Block::Table),
        term_parser(term_item).map(Block::Terms),
        list_parser(list_item).map(Block::List),
        enum_parser(enum_item).map(Block::Enum),
        raw_parser().map(Block::Raw),
        paragraph.map(Block::Paragraph),
    ))
    .with_ctx(ParserContext { indent: 0 })
}

pub fn paragraph_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, Paragraph, Extra<'src>>
where
    T: Parser<'src, &'src str, Vec<Inline>, Extra<'src>> + 'src,
{
    recursive(
        |par: Recursive<dyn Parser<&'src str, Vec<Inline>, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(par)
                .map_with(|nested, e| (nested, e.span()));

            let el = text
                .then(nested.or_not())
                .map_with(|(mut text, nested), e| {
                    let span = e.span();

                    if let Some((mut nested, nested_span)) = nested {
                        text.push(Inline::SoftBreak(SoftBreak {
                            span: Span::new(span.end, nested_span.start),
                        }));
                        text.append(&mut nested);
                    }
                    (text, span)
                });

            el.separated_by(level_parser())
                .at_least(1)
                .collect()
                .map(|content: Vec<_>| {
                    let (mut content, spans): (Vec<_>, Vec<_>) = content.into_iter().unzip();

                    for (i, span) in spans
                        .array_windows()
                        .map(|[a, b]| Span::new(a.end, b.start))
                        .enumerate()
                    {
                        content[i].push(Inline::SoftBreak(SoftBreak { span }));
                    }

                    content.into_iter().flatten().collect()
                })
                .boxed()
        },
    )
    .map_with(|content, e| Paragraph {
        content,
        span: e.span(),
    })
}

pub fn heading_parser<'src, T>(paragraph: T) -> impl Parser<'src, &'src str, Heading, Extra<'src>>
where
    T: Parser<'src, &'src str, Paragraph, Extra<'src>> + 'src,
{
    let marker = just("=")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_ecow()
        .map_with(|level, e| HeadingLevel {
            level: level.len() as u8,
            span: e.span(),
        })
        .then_ignore(just(" "));

    group((
        marker,
        paragraph.with_ctx(ParserContext { indent: 1 }),
        label_parser().or_not(),
    ))
    .map_with(|(level, par, label), e| Heading {
        level,
        content: par.content,
        label,
        span: e.span(),
    })
}

pub fn raw_parser<'src>() -> impl Parser<'src, &'src str, Raw, Extra<'src>> {
    let delim = "```";
    let lang = ascii::ident()
        .to_ecow()
        .map_with(|lang, e| RawLang {
            lang,
            span: e.span(),
        })
        .or_not();
    let content = none_of(delim)
        .repeated()
        .at_least(1)
        .to_ecow()
        .map_with(|content, e| RawContent {
            content,
            span: e.span(),
        });

    group((lang, content))
        .delimited_by(just(delim), just(delim))
        .map_with(|(lang, content), e| Raw {
            lang,
            content,
            span: e.span(),
        })
}

pub fn table_parser<'src, R>(table_row: R) -> impl Parser<'src, &'src str, Table, Extra<'src>>
where
    R: Parser<'src, &'src str, TableRow, Extra<'src>>,
{
    let label = just(" ").ignore_then(label_parser()).or_not();

    table_row
        .separated_by(soft_break_parser())
        .at_least(1)
        .collect()
        .then(label)
        .map_with(|(rows, label): (Vec<TableRow>, _), e| {
            let col_count = rows[0].cells.len();

            let table = Table {
                col_count,
                rows,
                label,
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
    T: Parser<'src, &'src str, Vec<Inline>, Extra<'src>>,
    E: Parser<'src, &'src str, EnumItem, Extra<'src>>,
    L: Parser<'src, &'src str, ListItem, Extra<'src>>,
{
    let delim = just("|");

    let cell = choice((
        list_item.map(|item| Block::List(item.into())),
        enum_item.map(|item| Block::Enum(item.into())),
        text.map_with(|content, e| {
            Block::Plain(Plain {
                content,
                span: e.span(),
            })
        }),
    ))
    .padded_by(just(" ").repeated());

    cell.separated_by(delim)
        .at_least(1)
        .collect()
        .delimited_by(delim, delim)
        .map_with(|cells, e| TableRow {
            cells,
            span: e.span(),
        })
}

pub fn term_parser<'src, I>(item: I) -> impl Parser<'src, &'src str, Terms, Extra<'src>>
where
    I: Parser<'src, &'src str, TermItem, Extra<'src>>,
{
    item.separated_by(soft_break_parser())
        .at_least(1)
        .collect()
        .map_with(|content, e| Terms {
            content,
            span: e.span(),
        })
}

const TERM_SPECIAL: &[char] = concat_slices!([char]: SPECIAL, &[':']);

pub fn term_item_parser<'src, T>(
    paragraph: T,
) -> impl Parser<'src, &'src str, TermItem, Extra<'src>>
where
    T: Parser<'src, &'src str, Paragraph, Extra<'src>>,
{
    group((
        just("> ").ignored(),
        text_parser(TERM_SPECIAL),
        just(": ").ignored(),
        paragraph.with_ctx(ParserContext { indent: 1 }),
    ))
    .map_with(|((), term, (), par), e| TermItem {
        term,
        content: par.content,
        span: e.span(),
    })
}

pub fn list_parser<'src, I>(list_item: I) -> impl Parser<'src, &'src str, List, Extra<'src>>
where
    I: Parser<'src, &'src str, ListItem, Extra<'src>> + Clone + 'src,
{
    recursive(
        |list: Recursive<dyn Parser<&'src str, List, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(list);

            let item = list_item.then(nested.or_not()).map(|(mut item, nested)| {
                if let Some(nested) = nested {
                    item.content.push(Block::List(nested));
                }

                item
            });

            item.separated_by(level_parser())
                .at_least(1)
                .collect()
                .map_with(|items, e| List {
                    items,
                    span: e.span(),
                })
                .boxed()
        },
    )
}

pub fn list_item_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, ListItem, Extra<'src>>
where
    T: Parser<'src, &'src str, Vec<Inline>, Extra<'src>> + 'src,
{
    just("- ").ignore_then(text.map_with(|content, e| ListItem {
        content: vec![Block::Plain(Plain {
            content,
            span: e.span(),
        })],
        span: e.span(),
    }))
}

pub fn enum_parser<'src, I>(enum_item: I) -> impl Parser<'src, &'src str, Enum, Extra<'src>>
where
    I: Parser<'src, &'src str, EnumItem, Extra<'src>> + Clone + 'src,
{
    recursive(
        |list: Recursive<dyn Parser<&'src str, Enum, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(list);

            let item = enum_item.then(nested.or_not()).map(|(mut item, nested)| {
                if let Some(nested) = nested {
                    item.content.push(Block::Enum(nested));
                }

                item
            });

            item.separated_by(level_parser())
                .at_least(1)
                .collect()
                .map_with(|items, e| Enum {
                    items,
                    span: e.span(),
                })
                .boxed()
        },
    )
}

pub fn enum_item_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, EnumItem, Extra<'src>>
where
    T: Parser<'src, &'src str, Vec<Inline>, Extra<'src>> + 'src,
{
    just("+ ").ignore_then(text.map_with(|content, e| EnumItem {
        content: vec![Block::Plain(Plain {
            content,
            span: e.span(),
        })],
        span: e.span(),
    }))
}
// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, Label, Extra<'src>> {
    ascii::ident()
        .to_ecow()
        .map_with(|label, e| Label {
            label,
            span: e.span(),
        })
        .delimited_by(just("{"), just("}"))
}

pub fn text_parser<'src>(
    special: &'src [char],
) -> impl Parser<'src, &'src str, Vec<Inline>, Extra<'src>> {
    inline_parser(special).repeated().at_least(1).collect()
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
        .delimited_by(just("<"), just(">"))
        .map_with(|href, e| Href {
            href,
            span: e.span(),
        });

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
