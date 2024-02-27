use chumsky::{
    prelude::*,
    text::{ascii, newline},
};

use crate::{
    code::{code_parser, Code},
    Span,
};

type Extra<'src> = extra::Err<Rich<'src, char, Span>>;
const SPECIAL: &str = " \\\n\"{}[]/*~_^@#`%";

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
    Code(Code<'src>),
}

pub fn inline_parser<'src, W>(
    word: W,
) -> impl Parser<'src, &'src str, Inline<'src>, Extra<'src>> + Clone
where
    W: Parser<'src, &'src str, Word<'src>, Extra<'src>> + 'src,
{
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
            word.map(Inline::Word),
        ))
        .boxed()
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

    just("%")
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
