use chumsky::{
    prelude::*,
    text::{digits, newline, unicode},
};

use super::{
    ext::{BranchParser, LeafParser},
    markup, Extra,
};
use crate::{kind::SyntaxKind, node::*};

pub fn code_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + Clone + 'src,
{
    just("#")
        .to_leaf(SyntaxKind::CodeMarker)
        .then(expr_parser(inline))
        .map(|(m, e)| vec![m, e])
        .to_branch(SyntaxKind::Code)
}

pub fn expr_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    I: Parser<'src, &'src str, Node, Extra<'src>> + Clone + 'src,
{
    recursive(|expr| {
        let content = markup::content_parser(inline).boxed();
        let args = args_parser(expr.clone(), content.clone());

        let call = unicode::ident()
            .to_leaf(SyntaxKind::CallIdent)
            .then(args)
            .map(|(i, a)| vec![i, a])
            .to_branch(SyntaxKind::Call);
        let access = unicode::ident().to_leaf(SyntaxKind::Ident);

        let block = expr
            .separated_by(just(";").padded())
            .collect()
            .padded()
            .delimited_by(just("{"), just("}"))
            .to_branch(SyntaxKind::ExprBlock);

        choice((literal_parser(), call, access, block, content)).boxed()
    })
}

pub fn args_parser<'src, E, C>(
    expr: E,
    content: C,
) -> impl Parser<'src, &'src str, Node, Extra<'src>>
where
    E: Parser<'src, &'src str, Node, Extra<'src>>,
    C: Parser<'src, &'src str, Node, Extra<'src>> + 'src,
{
    let arg = unicode::ident()
        .to_leaf(SyntaxKind::ArgIdent)
        .then_ignore(just(": "))
        .or_not()
        .then(expr)
        .map(|(ident, value)| {
            if let Some(i) = ident {
                vec![i, value]
            } else {
                vec![value]
            }
        })
        .to_branch(SyntaxKind::Arg);

    let args = arg
        .separated_by(just(",").padded())
        .allow_trailing()
        .collect::<Vec<_>>()
        .padded()
        .delimited_by(just("("), just(")"));

    let content = content.or_not();

    args.foldl(content, |mut args, c| {
        args.push(c);
        args
    })
    .to_branch(SyntaxKind::Args)
}

pub fn literal_parser<'src>() -> impl Parser<'src, &'src str, Node, Extra<'src>> {
    let boolean = just("true").or(just("false")).to_leaf(SyntaxKind::Bool);

    let octa = just("0o").ignore_then(digits(8));
    let hexa = just("0x").ignore_then(digits(16));
    let decimal = digits(10);
    let int = choice((octa, hexa, decimal)).to_leaf(SyntaxKind::Int);

    let float = group((digits(10), just("."), digits(10))).to_leaf(SyntaxKind::Float);

    let string = none_of("\"")
        .and_is(newline().not())
        .repeated()
        .to_leaf(SyntaxKind::Str)
        .delimited_by(just("\""), just("\""));

    choice((boolean, int, float, string))
}
