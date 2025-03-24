use chumsky::{
    prelude::*,
    text::{inline_whitespace, newline, unicode},
};
use tyd_core::prelude::*;

use super::{ext::ParserExt, extra::Extra, markup};

/// Parses code expressions that start with a hash (#) symbol
///
/// # Arguments
///
/// * `inline` - Parser for inline elements that can appear within code expressions
///
/// # Returns
///
/// A parser that recognizes code expressions and produces a `tree::Code` node
pub fn code_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, NodeId<tree::Code>, Extra<'src>>
where
    I: Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> + Clone + 'src,
{
    just("#")
        .ignore_then(expr_parser(inline))
        .map_to_node(tree::Code)
}

/// Parses identifier tokens
///
/// # Returns
///
/// A parser that recognizes valid identifiers and produces a `tree::Ident` node
pub fn ident_parser<'src>() -> impl Parser<'src, &'src str, NodeId<tree::Ident>, Extra<'src>> {
    unicode::ident().to_ecow().map_to_node(tree::Ident)
}

/// Parses expressions recursively
///
/// Handles various expression types including:
/// - Identifiers
/// - Literals
/// - Function calls
/// - For loops
/// - If-then-else conditionals
/// - Let bindings
/// - Content blocks
///
/// # Arguments
///
/// * `inline` - Parser for inline elements that can appear within expressions
///
/// # Returns
///
/// A parser that recognizes expressions and produces a `tree::Expr` node
pub fn expr_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, NodeId<tree::Expr>, Extra<'src>>
where
    I: Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> + Clone + 'src,
{
    recursive(|expr| {
        let ident = ident_parser().to_expr();
        let literal = literal_parser().to_expr();

        let content = markup::content_parser(inline)
            .map_to_node(tree::Content)
            .boxed();
        let args = args_parser(expr.clone(), content.clone());

        let call = ident_parser()
            .then(args)
            .map_to_node(|(ident, args)| tree::Call { ident, args })
            .to_expr()
            .boxed();

        let for_ = just("for")
            .then(inline_whitespace().at_least(1))
            .ignore_then(ident_parser())
            .then_ignore(just("in").then(inline_whitespace().at_least(1)))
            .then(ident_parser().to_expr().or(call.clone()))
            .then(content.clone())
            .map_to_node(|((el, inside), content)| tree::For {
                el,
                inside,
                content,
            })
            .to_expr()
            .boxed();

        let if_ = just("if")
            .then(inline_whitespace().at_least(1))
            .ignore_then(expr.clone())
            .then_ignore(just("then").padded())
            .then(content.clone())
            .then_ignore(just("else").padded())
            .then(content.clone())
            .map_to_node(|((predicate, then), or)| tree::If {
                predicate,
                then,
                or,
            })
            .to_expr()
            .boxed();

        let bind = ident_parser()
            .then_ignore(just("=").padded_by(inline_whitespace().at_least(1)))
            .then(expr.clone())
            .map_to_node(|(name, value)| tree::Bind { name, value });

        let let_ = just("let")
            .then(inline_whitespace().at_least(1))
            .ignore_then(
                bind.separated_by(just(";"))
                    .at_least(1)
                    .allow_trailing()
                    .collect(),
            )
            .map_to_node(tree::Let)
            .to_expr();

        choice((literal, for_, if_, let_, call, ident, content.to_expr())).boxed()
    })
}

/// Parses function arguments and optional content block
///
/// # Arguments
///
/// * `expr` - Parser for expressions that can appear as argument values
/// * `content` - Parser for content blocks that can follow the argument list
///
/// # Returns
///
/// A parser that recognizes argument lists and produces a `tree::Args` node
pub fn args_parser<'src, E, C>(
    expr: E,
    content: C,
) -> impl Parser<'src, &'src str, NodeId<tree::Args>, Extra<'src>>
where
    E: Parser<'src, &'src str, NodeId<tree::Expr>, Extra<'src>>,
    C: Parser<'src, &'src str, NodeId<tree::Content>, Extra<'src>> + 'src,
{
    let arg = ident_parser()
        .then_ignore(just(": "))
        .or_not()
        .then(expr)
        .map_to_node(|(key, value)| tree::Arg { name: key, value });

    arg.separated_by(just(",").padded())
        .allow_trailing()
        .collect()
        .padded()
        .delimited_by(just("("), just(")"))
        .then(content.or_not())
        .map_to_node(|(args, content)| tree::Args { args, content })
}

/// Parses literal values
///
/// Supports the following literal types:
/// - Booleans (`true` or `false`)
/// - Integers (decimal, octal with `0o` prefix, hexadecimal with `0x` prefix)
/// - Floating point numbers
/// - String literals enclosed in double quotes
///
/// # Returns
///
/// A parser that recognizes literals and produces a `tree::Literal` node
pub fn literal_parser<'src>() -> impl Parser<'src, &'src str, NodeId<tree::Literal>, Extra<'src>> {
    let boolean = just("true")
        .to(true)
        .or(just("false").to(false))
        .map(tree::Literal::Bool);

    let int = choice((
        just("0o").ignore_then(text::int(8)),
        just("0x").ignore_then(text::int(16)),
        text::int(10),
    ))
    .from_str()
    .unwrapped()
    .map(tree::Literal::Int);

    let float = text::int(10)
        .then(just('.').then(text::digits(10)))
        .to_slice()
        .from_str()
        .unwrapped()
        .map(tree::Literal::Float);

    let string = none_of("\"")
        .and_is(newline().not())
        .repeated()
        .to_ecow()
        .map(tree::Literal::Str)
        .delimited_by(just("\""), just("\""));

    choice((boolean, int, float, string)).to_node()
}
