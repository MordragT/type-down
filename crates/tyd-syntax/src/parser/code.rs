use chumsky::{
    prelude::*,
    text::{newline, unicode},
};
use tyd_doc::prelude::*;

use super::{ext::ParserExt, extra::Extra, markup};

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

pub fn ident_parser<'src>() -> impl Parser<'src, &'src str, NodeId<tree::Ident>, Extra<'src>> {
    unicode::ident().to_ecow().map_to_node(tree::Ident)
}

pub fn expr_parser<'src, I>(
    inline: I,
) -> impl Parser<'src, &'src str, NodeId<tree::Expr>, Extra<'src>>
where
    I: Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>> + Clone + 'src,
{
    recursive(|expr| {
        let content = markup::content_parser(inline)
            .map_to_node(tree::Content)
            .boxed();
        let args = args_parser(expr.clone(), content.clone());

        let call = ident_parser()
            .then(args)
            .map_to_node(|(ident, args)| tree::Call { ident, args })
            .to_expr();

        let block = expr
            .separated_by(just(";").padded())
            .collect()
            .padded()
            .delimited_by(just("{"), just("}"))
            .map_to_node(tree::ExprBlock)
            .to_expr();

        let access = ident_parser().to_expr();
        let literal = literal_parser().to_expr();

        choice((literal, call, access, block, content.to_expr())).boxed()
    })
}

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
        .map_to_node(|(key, value)| tree::Arg { key, value });

    arg.separated_by(just(",").padded())
        .allow_trailing()
        .collect()
        .padded()
        .delimited_by(just("("), just(")"))
        .then(content.or_not())
        .map_to_node(|(args, content)| tree::Args { args, content })
}

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
