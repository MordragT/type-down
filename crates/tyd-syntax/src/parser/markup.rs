use chumsky::{
    prelude::*,
    text::{newline, unicode},
};
use constcat::concat_slices;
use tyd_core::prelude::*;

use super::{
    code::code_parser,
    ext::ParserExt,
    extra::{Context, Extra, State},
};

/// Special characters that have semantic meaning in the markup language
pub const SPECIAL: &[char] = &[
    ' ', '\\', '\n', '"', '{', '}', '[', ']', '/', '*', '~', '_', '^', '@', '#', '`', '$', '%', '|',
];

/// Special characters for term definitions, includes all regular special characters plus colon
const TERM_SPECIAL: &[char] = concat_slices!([char]: SPECIAL, &[':']);

/// Root parser that processes the entire document
///
/// Returns a vector of Block nodes representing the entire document structure
pub fn parser<'src>() -> impl Parser<'src, &'src str, Vec<NodeId<tree::Block>>, Extra<'src>> {
    block_parser()
        .separated_by(newline().repeated().at_least(2))
        .allow_trailing()
        .at_least(1)
        .collect()
        .then_ignore(end())
}

/// Parser for matching the current indentation level
///
/// Uses the context's indent value to determine how many indentation units to match
pub fn level_parser<'src>() -> impl Parser<'src, &'src str, usize, Extra<'src>> {
    let indent = just("    ").or(just("\t"));

    indent
        .repeated()
        .configure(|cfg, ctx: &Context| cfg.exactly(ctx.indent))
        .count()
}

/// Parser for matching one level of additional indentation beyond the current level
///
/// Uses the context's indent value + 1 to determine how many indentation units to match
pub fn indent_parser<'src>() -> impl Parser<'src, &'src str, usize, Extra<'src>> {
    let indent = just("    ").or(just("\t"));

    indent
        .repeated()
        .configure(|cfg, ctx: &Context| cfg.exactly(ctx.indent + 1))
        .count()
}

/// Parser for all types of block-level elements
///
/// Includes headings, code blocks, lists, enumerations, term definitions, tables, and paragraphs
pub fn block_parser<'src>() -> impl Parser<'src, &'src str, NodeId<tree::Block>, Extra<'src>> {
    let inline = inline_parser(SPECIAL).boxed();
    let text = text_parser(inline.clone()).boxed();

    // Heading parser - handles "=" markers for h1-h6
    let heading_marker = just("=")
        .repeated()
        .at_least(1)
        .at_most(6)
        .count()
        .map_to_node(|n| tree::HeadingMarker(n as u8));
    let heading = group((
        heading_marker.then_ignore(just(" ")),
        text.clone().with_ctx(Context { indent: 1 }),
        label_parser().or_not(),
    ))
    .map_to_node(|(marker, content, label)| tree::Heading {
        marker,
        content,
        label,
    })
    .to_block();

    // Raw code block parser - handles ```code``` blocks with optional language tag
    let delim = "```";
    let raw_lang = unicode::ident().to_ecow().map_to_node(tree::Tag);
    let raw_content = none_of(delim)
        .repeated()
        .at_least(1)
        .to_ecow()
        .map_to_node(tree::Text);
    let raw = raw_lang
        .or_not()
        .then(raw_content)
        .delimited_by(just(delim), just(delim))
        .map_to_node(|(lang, text)| tree::Raw { text, lang })
        .to_block();

    // Plain text block parser - simple inline content
    let plain = inline
        .repeated()
        .at_least(1)
        .collect()
        .map_to_node(tree::Plain)
        .to_block()
        .boxed();

    // List item parsers - handles "+" for enumerated lists and "-" for bullet lists
    let enum_item = just("+ ")
        .ignore_then(plain.clone())
        .map(|plain| tree::EnumItem(vec![plain]));
    let list_item = just("- ")
        .ignore_then(plain)
        .map(|plain| tree::ListItem(vec![plain]));

    // Enumeration parser - handles numbered lists with "+" markers and nested structure
    let enumeration = recursive(
        |enumeration: Recursive<dyn Parser<&'src str, NodeId<tree::Block>, Extra<'src>>>| {
            let nested = newline()
                .ignore_then(indent_parser())
                .map(|indent| Context { indent })
                .ignore_with_ctx(enumeration);

            let item = enum_item
                .clone()
                .then(nested.or_not())
                .map_to_node(|(mut item, nested)| {
                    if let Some(nested) = nested {
                        item.0.push(nested);
                    }
                    item
                });

            item.separated_by(newline().then(level_parser()))
                .at_least(1)
                .collect()
                .map_to_node(tree::Enum)
                .to_block()
                .boxed()
        },
    );

    // List parser - handles bullet lists with "-" markers and nested structure
    let list = recursive(
        |list: Recursive<dyn Parser<&'src str, NodeId<tree::Block>, Extra<'src>>>| {
            let nested = newline()
                .ignore_then(indent_parser())
                .map(|indent| Context { indent })
                .ignore_with_ctx(list);

            let item = list_item
                .clone()
                .then(nested.or_not())
                .map_to_node(|(mut item, nested)| {
                    if let Some(nested) = nested {
                        item.0.push(nested);
                    }
                    item
                });

            item.separated_by(newline().then(level_parser()))
                .at_least(1)
                .collect()
                .map_to_node(tree::List)
                .to_block()
                .boxed()
        },
    );

    // Term definition parser - handles "> term : description" format
    let term = just("> ").ignore_then(inline_parser(TERM_SPECIAL).repeated().at_least(1).collect());
    let desc = just(": ").ignore_then(text.clone().with_ctx(Context { indent: 1 }));
    let term_item = term
        .then(desc)
        .map_to_node(|(term, desc)| tree::TermItem { term, desc });

    let terms = term_item
        .separated_by(newline())
        .at_least(1)
        .collect()
        .map_to_node(tree::Terms)
        .to_block();

    // Paragraph parser - standard text block
    let paragraph = text.map_to_node(tree::Paragraph).to_block().boxed();

    // Table parser - processes pipe-delimited table structures
    let table_cell = choice((
        list_item
            .to_node()
            .map_to_node(|item| tree::List(vec![item]))
            .to_block(),
        enum_item
            .to_node()
            .map_to_node(|item| tree::Enum(vec![item]))
            .to_block(),
        paragraph.clone(),
    ))
    .padded_by(just(" ").repeated());

    let delim = just("|");
    let table_row = table_cell
        .separated_by(delim)
        .at_least(1)
        .collect()
        .delimited_by(delim, delim)
        .map_to_node(tree::TableRow);

    let table = table_row
        .separated_by(newline())
        .at_least(1)
        .collect::<Vec<_>>()
        .validate(|rows, e, emitter| {
            let span = e.span();
            let state: &mut State = e.state();

            let mut count = None;

            for id in &rows {
                let len = state.node(*id).0.len();

                if let Some(c) = count
                    && c != len
                {
                    emitter.emit(Rich::custom(
                        span,
                        "Adjacent table rows must contain equal number of cells.",
                    ))
                } else {
                    count = Some(len);
                }
            }

            (rows, count.unwrap())
        })
        .then(just(" ").ignore_then(label_parser()).or_not())
        .map_to_node(|((rows, columns), label)| tree::Table {
            rows,
            columns,
            label,
        })
        .to_block();

    // Choose among all block-level elements with default indentation of 0
    choice((heading, raw, list, enumeration, terms, table, paragraph))
        .with_ctx(Context { indent: 0 })
}

// TODO maybe allow more attributes to be specified ? Maybe something like {label .class key=value} ?
// then one could also simplify div and raw to just take this new attr literal instead of own lang and class parsers ?

/// Parser for label annotations like {label}
///
/// Labels can be attached to various elements to provide identifiers
pub fn label_parser<'src>() -> impl Parser<'src, &'src str, NodeId<tree::Label>, Extra<'src>> {
    unicode::ident()
        .to_ecow()
        .delimited_by(just("{"), just("}"))
        .map(tree::Label)
        .to_node()
}

/// Parser for text content that may span multiple lines
///
/// Handles soft line breaks and indentation to maintain proper text flow
pub fn text_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, Vec<NodeId<tree::Inline>>, Extra<'src>>
where
    I: Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> + 'src,
{
    let soft_break = newline()
        .to(tree::SoftBreak)
        .to_node()
        .map(tree::Inline::from)
        .to_node()
        .boxed();
    let line = inline.repeated().at_least(1).collect::<Vec<_>>();

    recursive(
        |paragraph: Recursive<dyn Parser<&'src str, Vec<NodeId<tree::Inline>>, Extra<'src>>>| {
            let nested = soft_break
                .clone()
                .then(
                    indent_parser()
                        .map(|indent| Context { indent })
                        .ignore_with_ctx(paragraph),
                )
                .map(|(sb, mut text)| {
                    text.insert(0, sb);
                    text
                });

            let wrapped = line
                .then(nested.or_not())
                .map(|(mut text, nested)| {
                    if let Some(mut nested) = nested {
                        text.append(&mut nested);
                    }
                    text
                })
                .boxed();

            let next = soft_break
                .then_ignore(level_parser())
                .then(wrapped.clone())
                .map(|(sb, mut wrapped)| {
                    wrapped.insert(0, sb);
                    wrapped
                })
                .repeated();

            wrapped
                .foldl(next, |mut wrapped, mut next| {
                    wrapped.append(&mut next);
                    wrapped
                })
                .boxed()
        },
    )
}

/// Parser for content inside brackets
///
/// Handles both simple inline content and nested multi-line content
pub fn content_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, Vec<NodeId<tree::Inline>>, Extra<'src>>
where
    I: Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> + Clone + 'src,
{
    let simple = inline
        .clone()
        .repeated()
        .collect()
        .delimited_by(just("["), just("]"));

    let nested = newline()
        .then(level_parser())
        .ignore_then(text_parser(inline))
        .with_ctx(Context { indent: 1 })
        .delimited_by(just("["), newline().then(just("]")));

    simple.or(nested)
}

// TODO allow quote, strikeout, emphasis symbols only with a leading space so: " *strong* and not*strong*here"

/// Parser for inline-level elements within text
///
/// Handles formatting like quotes, emphasis, links, and other inline markup
pub fn inline_parser<'src>(
    special: &'src [char],
) -> impl Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> + Clone {
    recursive(|inline| {
        // Quote parser - handles "quoted text"
        let quote = inline
            .clone()
            .repeated()
            .collect()
            .delimited_by(just("\""), just("\""))
            .map_to_node(tree::Quote)
            .to_inline()
            .recover_with(via_parser(inline_recovery("\"")))
            .boxed();

        // Strikeout parser - handles ~strikeout text~
        let strikeout = inline
            .clone()
            .repeated()
            .collect()
            .delimited_by(just("~"), just("~"))
            .map_to_node(tree::Strikeout)
            .to_inline()
            .recover_with(via_parser(inline_recovery("~")))
            .boxed();

        // Strong/bold parser - handles *bold text*
        let strong = inline
            .clone()
            .repeated()
            .collect()
            .delimited_by(just("*"), just("*"))
            .map_to_node(tree::Strong)
            .to_inline()
            .recover_with(via_parser(inline_recovery("*")))
            .boxed();

        // Emphasis/italic parser - handles /italic text/
        let emphasis = inline
            .clone()
            .repeated()
            .collect()
            .delimited_by(just("/"), just("/"))
            .map_to_node(tree::Emphasis)
            .to_inline()
            .recover_with(via_parser(inline_recovery("/")))
            .boxed();

        // Word parser - handles regular text content
        let word = none_of(special)
            .repeated()
            .at_least(1)
            .to_ecow()
            .map_to_node(tree::Word)
            .to_inline()
            .boxed();

        let content = content_parser(inline.clone()).boxed();

        // Single letter parser for subscript/superscript usage
        let letter = none_of(special)
            .to_ecow()
            .map_to_node(tree::Word)
            .to_inline()
            .map(|w| vec![w])
            .boxed();

        // Subscript parser - handles _subscript
        let subscript = just("_")
            .ignore_then(content.clone().or(letter.clone()))
            .map_to_node(tree::Subscript)
            .to_inline();

        // Superscript parser - handles ^superscript
        let supscript = just("^")
            .ignore_then(content.clone().or(letter))
            .map_to_node(tree::Supscript)
            .to_inline();

        // Link parser - handles <url> and <url>[content]
        let link = none_of(">")
            .and_is(newline().not())
            .repeated()
            .at_least(1)
            .to_ecow()
            .delimited_by(just("<"), just(">"))
            .map(tree::Text)
            .to_node()
            .then(content.or_not())
            .map_to_node(|(href, content)| tree::Link { href, content })
            .to_inline()
            .boxed();

        // Reference parser - handles @identifier
        let ref_ = just("@").ignore_then(
            unicode::ident()
                .to_ecow()
                .map_to_node(tree::Ref)
                .to_inline()
                .boxed(),
        );

        // Raw inline code parser - handles `code`
        let delim = "`";
        let raw_inline = none_of(delim)
            .and_is(newline().not())
            .repeated()
            .at_least(1)
            .to_ecow()
            .delimited_by(just(delim), just(delim))
            .map_to_node(tree::RawInline)
            .to_inline()
            .boxed();

        // Math inline parser - handles $math$
        let delim = "$";
        let math_inline = none_of(delim)
            .and_is(newline().not())
            .repeated()
            .at_least(1)
            .to_ecow()
            .delimited_by(just(delim), just(delim))
            .map_to_node(tree::MathInline)
            .to_inline()
            .boxed();

        // Escape sequence parser - handles \char for special characters
        let recovery = just("\\")
            .then(any())
            .to_ecow()
            .map_to_node(tree::Error)
            .to_inline();
        let escape = just("\\")
            .ignore_then(
                one_of(SPECIAL)
                    .to_ecow()
                    .map_to_node(tree::Escape)
                    .to_inline(),
            )
            .recover_with(via_parser(recovery))
            .boxed();

        // Spacing parser - handles consecutive spaces
        let spacing = just(" ")
            .repeated()
            .at_least(1)
            .to(tree::Spacing)
            .to_node()
            .to_inline()
            .boxed();

        // Comment parser - handles %comment
        let content = any()
            .and_is(newline().not())
            .repeated()
            .at_least(1)
            .to_ecow()
            .map_to_node(tree::Comment)
            .to_inline();
        let comment = just("%").ignore_then(content);

        // Code block parser
        let code = code_parser(inline.clone()).to_inline();

        // Combined inline parser with priority order
        choice((
            code,
            quote,
            strikeout,
            strong,
            emphasis,
            subscript,
            supscript,
            link,
            ref_,
            raw_inline,
            math_inline,
            escape,
            spacing,
            comment,
            word,
        ))
        .boxed()
        .labelled("inline")
        .as_context()
    })
}

/// Recovery parser for inline elements with unmatched delimiters
///
/// Helps provide better error reporting for malformed inline elements
pub fn inline_recovery<'src>(
    delim: &'src str,
) -> impl Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> {
    just(delim)
        .then(
            none_of(SPECIAL)
                .and_is(newline().not())
                .repeated()
                .at_least(1),
        )
        .to_ecow()
        .map_to_node(tree::Error)
        .to_inline()
}
