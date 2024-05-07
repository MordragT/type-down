use chumsky::{
    prelude::*,
    text::{newline, unicode},
};
use constcat::concat_slices;
use ecow::EcoString;

use super::{
    code::code_parser,
    ext::{BranchParser, LeafParser},
    Extra, ParserContext,
};
use crate::{
    kind::SyntaxKind,
    node::{BranchNode, LeafNode, Node},
    Span,
};

pub const SPECIAL: &[char] = &[
    ' ', '\\', '\n', '"', '{', '}', '[', ']', '/', '*', '~', '_', '^', '@', '#', '`', '$', '%', '|',
];

pub fn parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    let doc = block_parser()
        .map(Node::from)
        // .recover_with(recovery)
        .separated_by(hard_break_parser())
        .allow_trailing()
        .at_least(1)
        .collect()
        .to_branch(SyntaxKind::Document);
    doc.then_ignore(end())
}

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

pub fn block_parser<'src>() -> impl Parser<'src, &'src str, BranchNode, Extra<'src>> {
    let inline = inline_parser(SPECIAL).boxed();
    let text = text_parser(inline.clone()).boxed();
    let list_item = list_item_parser(inline.clone()).boxed();
    let enum_item = enum_item_parser(inline.clone()).boxed();
    let term_item = term_item_parser(text.clone());
    let table_row = table_row_parser(text.clone(), enum_item.clone(), list_item.clone());

    choice((
        heading_parser(text.clone()),
        table_parser(table_row),
        terms_parser(term_item),
        list_parser(list_item),
        enum_parser(enum_item),
        raw_parser(),
        text.to_branch_node(SyntaxKind::Paragraph),
    ))
    .with_ctx(ParserContext { indent: 0 })
}

pub fn heading_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    T: Parser<'src, &'src str, Vec<Node>, Extra<'src>> + 'src,
{
    let marker = just("=")
        .repeated()
        .at_least(1)
        .at_most(6)
        .to_leaf(SyntaxKind::HeadingMarker)
        .then_ignore(just(" "));

    group((
        marker,
        text.with_ctx(ParserContext { indent: 1 })
            .to_branch(SyntaxKind::Content),
        label_parser().or_not(),
    ))
    .map(|(level, content, label)| {
        let mut children = vec![level, content];
        if let Some(label) = label {
            children.push(label);
        }
        children
    })
    .to_branch_node(SyntaxKind::Heading)
}

pub fn raw_parser<'src>() -> impl Parser<'src, &'src str, BranchNode, Extra<'src>> {
    let delim = "```";
    let lang = unicode::ident().to_leaf(SyntaxKind::RawLang).or_not();

    let content = none_of(delim)
        .repeated()
        .at_least(1)
        .to_leaf(SyntaxKind::Text);

    group((lang, content))
        .delimited_by(just(delim), just(delim))
        .map(|(l, c)| if let Some(l) = l { vec![l, c] } else { vec![c] })
        .to_branch_node(SyntaxKind::Raw)
}

pub fn table_parser<'src, R>(table_row: R) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    R: Parser<'src, &'src str, Node, Extra<'src>>,
{
    let rows = table_row
        .separated_by(soft_break_parser())
        .at_least(1)
        .collect::<Vec<_>>()
        .validate(|rows, e, emitter| {
            let cols = rows[0].children().len();
            if rows.iter().any(|row| row.children().len() != cols) {
                emitter.emit(Rich::custom(
                    e.span(),
                    "Adjacent table rows must contain equal number of cells.",
                ))
            }
            rows
        });
    let label = just(" ").ignore_then(label_parser()).or_not();

    rows.foldl(label, |mut nodes, l| {
        nodes.push(l);
        nodes
    })
    .to_branch_node(SyntaxKind::Table)
}

pub fn table_row_parser<'src, T, E, L>(
    text: T,
    enum_item: E,
    list_item: L,
) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    T: Parser<'src, &'src str, Vec<Node>, Extra<'src>>,
    E: Parser<'src, &'src str, BranchNode, Extra<'src>>,
    L: Parser<'src, &'src str, BranchNode, Extra<'src>>,
{
    let delim = just("|");

    let cell = choice((
        list_item.map(Node::Branch),
        enum_item.map(Node::Branch),
        text.to_branch(SyntaxKind::Plain),
    ))
    .padded_by(just(" ").repeated());

    cell.separated_by(delim)
        .at_least(1)
        .collect()
        .delimited_by(delim, delim)
        .to_branch(SyntaxKind::TableRow)
}

pub fn terms_parser<'src, I>(item: I) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>>,
{
    item.separated_by(soft_break_parser())
        .at_least(1)
        .collect()
        .to_branch_node(SyntaxKind::Terms)
}

const TERM_SPECIAL: &[char] = concat_slices!([char]: SPECIAL, &[':']);

pub fn term_item_parser<'src, T>(text: T) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    T: Parser<'src, &'src str, Vec<Node>, Extra<'src>>,
{
    let term = inline_parser(TERM_SPECIAL).repeated().at_least(1).collect();
    let desc = text.with_ctx(ParserContext { indent: 1 });

    group((
        just("> ").to_leaf(SyntaxKind::TermMarker),
        term.to_branch(SyntaxKind::Content),
        just(": ").to_leaf(SyntaxKind::Colon),
        desc.to_branch(SyntaxKind::Content),
    ))
    .map_with(|(m, t, s, d), e| {
        Node::Branch(BranchNode {
            kind: SyntaxKind::TermItem,
            span: e.span(),
            children: vec![m, t, s, d],
        })
    })
}

pub fn list_parser<'src, I>(list_item: I) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    I: Parser<'src, &'src str, BranchNode, Extra<'src>> + Clone + 'src,
{
    recursive(
        |list: Recursive<dyn Parser<&'src str, BranchNode, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(list)
                .map(Node::Branch);

            let item = list_item.then(nested.or_not()).map(|(mut item, nested)| {
                if let Some(nested) = nested {
                    item.children.push(nested);
                }

                Node::Branch(item)
            });

            item.separated_by(level_parser())
                .at_least(1)
                .collect()
                .to_branch_node(SyntaxKind::List)
                .boxed()
        },
    )
}

pub fn list_item_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + 'src,
{
    let text = inline
        .repeated()
        .at_least(1)
        .collect()
        .to_branch(SyntaxKind::Plain);

    just("- ")
        .to_leaf(SyntaxKind::ListMarker)
        .then(text)
        .map(|(m, t)| vec![m, t])
        .to_branch_node(SyntaxKind::ListItem)
}

pub fn enum_parser<'src, I>(enum_item: I) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    I: Parser<'src, &'src str, BranchNode, Extra<'src>> + Clone + 'src,
{
    recursive(
        |list: Recursive<dyn Parser<&'src str, BranchNode, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(list)
                .map(Node::Branch);

            let item = enum_item.then(nested.or_not()).map(|(mut item, nested)| {
                if let Some(nested) = nested {
                    item.children.push(nested);
                }
                Node::Branch(item)
            });

            item.separated_by(level_parser())
                .at_least(1)
                .collect()
                .to_branch_node(SyntaxKind::Enum)
                .boxed()
        },
    )
}

pub fn enum_item_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, BranchNode, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + 'src,
{
    let text = inline
        .repeated()
        .at_least(1)
        .collect()
        .to_branch(SyntaxKind::Plain);

    just("+ ")
        .to_leaf(SyntaxKind::EnumMarker)
        .then(text)
        .map(|(m, t)| vec![m, t])
        .to_branch_node(SyntaxKind::EnumItem)
}
// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

pub fn label_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    unicode::ident()
        .to_leaf(SyntaxKind::Label)
        .delimited_by(just("{"), just("}"))
}

// static INLINE_PARSER: LazyLock<Cache<InlineParser>> = LazyLock::new(Cache::default);

// #[derive(Default)]
// pub struct InlineParser;

// impl Cached for InlineParser {
//     type Parser<'src> = Arc<dyn Parser<'src, &'src str, Node, Extra<'src>> + Send + Sync + 'src>;

//     fn make_parser<'src>(self) -> Self::Parser<'src> {
//         Arc::new(inline_parser(SPECIAL))
//     }
// }

pub fn text_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Vec<Node>, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + 'src,
{
    let line = inline.repeated().at_least(1).collect::<Vec<_>>();

    recursive(
        |text: Recursive<dyn Parser<&'src str, Vec<Node>, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(text)
                .map_with(|nested, e| (nested, e.span()));

            let el = line
                .then(nested.or_not())
                .map_with(|(mut text, nested), e| {
                    let span = e.span();

                    if let Some((mut nested, nested_span)) = nested {
                        text.push(Node::Leaf(LeafNode {
                            span: Span::new(span.end, nested_span.start),
                            kind: SyntaxKind::SoftBreak,
                            text: EcoString::new(),
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
                        content[i].push(Node::Leaf(LeafNode {
                            span,
                            kind: SyntaxKind::SoftBreak,
                            text: EcoString::new(),
                        }));
                    }

                    content.into_iter().flatten().collect()
                })
                .boxed()
        },
    )
}

pub fn content_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + Clone + 'src,
{
    let simple = inline
        .clone()
        .repeated()
        .collect()
        .delimited_by(just("["), just("]"))
        .to_branch(SyntaxKind::Content);

    let nested = level_parser()
        .ignore_then(text_parser(inline))
        .with_ctx(ParserContext { indent: 1 })
        .delimited_by(just("["), soft_break_parser().then(just("]")))
        .to_branch(SyntaxKind::Content);

    simple.or(nested)
}

// TODO allow quote, strikeout, emphasis symbols only with a leading space so: " *strong* and not*strong*here"

pub fn inline_parser<'src>(
    special: &'src [char],
) -> impl Parser<'src, &'src str, Node, Extra<'src>> + Clone {
    recursive(|inline| {
        choice((
            code_parser(inline.clone()),
            quote_parser(inline.clone()),
            strikeout_parser(inline.clone()),
            strong_parser(inline.clone()),
            emphasis_parser(inline.clone()),
            subscript_parser(inline.clone()),
            supscript_parser(inline.clone()),
            link_parser(inline.clone()),
            ref_parser(),
            raw_inline_parser(),
            math_inline_parser(),
            comment_parser(),
            escape_parser(),
            spacing_parser(),
            word_parser(special),
        ))
        .boxed()
        .labelled("inline")
        .as_context()
    })
}

pub fn inline_recovery<'src>(delim: &'src str) -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    just(delim)
        .then(
            none_of(SPECIAL)
                .and_is(newline().not())
                .repeated()
                .at_least(1),
        )
        .to_error()
}

pub fn quote_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("\""), just("\""))
        .to_branch(SyntaxKind::Quote)
        .recover_with(via_parser(inline_recovery("\"")))
}

pub fn strikeout_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("~"), just("~"))
        .to_branch(SyntaxKind::Strikeout)
        .recover_with(via_parser(inline_recovery("~")))
}

pub fn emphasis_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("/"), just("/"))
        .to_branch(SyntaxKind::Emphasis)
        .recover_with(via_parser(inline_recovery("/")))
}

pub fn strong_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>>,
{
    inline
        .repeated()
        .collect()
        .delimited_by(just("*"), just("*"))
        .to_branch(SyntaxKind::Strong)
        .recover_with(via_parser(inline_recovery("*")))
}

pub fn subscript_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + Clone + 'src,
{
    let content = content_parser(inline).or(none_of(SPECIAL).to_leaf(SyntaxKind::Word));

    just("_")
        .to_leaf(SyntaxKind::SubscriptMarker)
        .then(content)
        .map(|(m, c)| vec![m, c])
        .to_branch(SyntaxKind::Subscript)
}

pub fn supscript_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + Clone + 'src,
{
    let content = content_parser(inline).or(none_of(SPECIAL).to_leaf(SyntaxKind::Word));

    just("^")
        .to_leaf(SyntaxKind::SupscriptMarker)
        .then(content)
        .map(|(m, c)| vec![m, c])
        .to_branch(SyntaxKind::Supscript)
}

pub fn link_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + Clone + 'src,
{
    let href = none_of(">")
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .delimited_by(just("<"), just(">"))
        .to_leaf(SyntaxKind::Text);

    let content = content_parser(inline).or_not();

    href.then(content)
        .map(|(href, content)| {
            if let Some(content) = content {
                vec![href, content]
            } else {
                vec![href]
            }
        })
        .to_branch(SyntaxKind::Link)
}

pub fn ref_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    just("@")
        .to_leaf(SyntaxKind::RefMarker)
        .then(unicode::ident().to_leaf(SyntaxKind::Ident))
        .map(|(m, i)| vec![m, i])
        .to_branch(SyntaxKind::Ref)
}

pub fn escape_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    let recovery = just("\\").then(any()).ignored().to_error();

    just("\\")
        .to_leaf(SyntaxKind::EscapeMarker)
        .then(one_of(SPECIAL).to_leaf(SyntaxKind::Word))
        .map(|(m, c)| vec![m, c])
        .to_branch(SyntaxKind::Escape)
        .recover_with(via_parser(recovery))
}

pub fn raw_inline_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    let delim = "`";

    none_of(delim)
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_leaf(SyntaxKind::RawInline)
        .delimited_by(just(delim), just(delim))
}

pub fn math_inline_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    let delim = "$";

    none_of(delim)
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_leaf(SyntaxKind::MathInline)
        .delimited_by(just(delim), just(delim))
}

pub fn comment_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    let marker = just("%").to_leaf(SyntaxKind::CommentMarker);
    let content = any()
        .and_is(newline().not())
        .repeated()
        .at_least(1)
        .to_leaf(SyntaxKind::Text);

    marker
        .then(content)
        .map(|(m, c)| vec![m, c])
        .to_branch(SyntaxKind::Comment)
}

pub fn word_parser<'src>(special: &'src [char]) -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    none_of(special)
        .repeated()
        .at_least(1)
        .to_leaf(SyntaxKind::Word)
}

pub fn spacing_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    just(" ")
        .repeated()
        .at_least(1)
        .to_leaf(SyntaxKind::Spacing)
}
