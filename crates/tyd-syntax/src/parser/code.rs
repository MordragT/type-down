use chumsky::{
    prelude::*,
    text::{ascii, digits, newline},
};

use super::{Extra, ParserExt};
use crate::ast::*;

pub fn code_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Code, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>> + 'src,
{
    just("#")
        .ignore_then(expr_parser(inline))
        .map_with(|expr, e| Code {
            expr,
            span: e.span(),
        })
}

pub fn expr_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Expr, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>> + 'src,
{
    recursive(|expr| {
        let ident = ascii::ident().to_ecow();
        let args = args_parser(expr.clone());
        let content = inline
            .repeated()
            .collect()
            .delimited_by(just("["), just("]"))
            .or_not();
        let call_tail = args.then(content);

        let access = ident.then(call_tail.or_not()).map(|(ident, tail)| {
            if let Some((args, content)) = tail {
                Expr::Call(Call {
                    ident,
                    args,
                    content,
                })
            } else {
                Expr::Ident(ident)
            }
        });
        let block = expr
            .separated_by(just(";").padded())
            .collect()
            .padded()
            .delimited_by(just("{"), just("}"))
            .map(Expr::Block);
        let literal = literal_parser().map(Expr::Literal);

        choice((literal, access, block)).boxed()
    })
}

pub fn args_parser<'src, E>(expr: E) -> impl Parser<'src, &'src str, Vec<Arg>, Extra<'src>>
where
    E: Parser<'src, &'src str, Expr, Extra<'src>>,
{
    let arg = ascii::ident()
        .to_ecow()
        .then_ignore(just(": "))
        .or_not()
        .then(expr)
        .map(|(name, value)| Arg { name, value });

    arg.separated_by(just(",").padded())
        .allow_trailing()
        .collect()
        .padded()
        .delimited_by(just("("), just(")"))
}

pub fn literal_parser<'src>() -> impl Parser<'src, &'src str, Literal, Extra<'src>> {
    let boolean = just("true")
        .to(true)
        .or(just("false").to(false))
        .map(Literal::Boolean);

    let octa = just("0o").ignore_then(digits(8));
    let hexa = just("0x").ignore_then(digits(16));
    let decimal = digits(10);
    let int = choice((octa, hexa, decimal))
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Literal::Int);

    let string = none_of("\"")
        .and_is(newline().not())
        .repeated()
        .to_ecow()
        .delimited_by(just("\""), just("\""))
        .map(Literal::Str);

    choice((boolean, int, string))
}
