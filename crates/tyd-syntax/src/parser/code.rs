use chumsky::{
    prelude::*,
    text::{ascii, digits, newline},
};

use super::{
    markup::{indent_parser, level_parser, soft_break_parser},
    Extra, ParserContext, ParserExt,
};
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
        let ident = ascii::ident().to_ecow().map_with(|value, e| Ident {
            value,
            span: e.span(),
        });
        let content = content_parser(inline).boxed();

        let args = args_parser(expr.clone(), content.clone());

        let access = ident.then(args.or_not()).map_with(|(ident, args), e| {
            if let Some(args) = args {
                Expr::Call(Call {
                    ident,
                    args,
                    span: e.span(),
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

        choice((literal, access, block, content.map(Expr::Content))).boxed()
    })
}

pub fn content_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Content, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline, Extra<'src>> + 'src,
{
    let text = inline.repeated().collect::<Vec<_>>().boxed();

    let paragraph = recursive(
        |par: Recursive<dyn Parser<&'src str, Vec<Inline>, Extra<'src>>>| {
            let nested = indent_parser()
                .map(|indent| ParserContext { indent })
                .ignore_with_ctx(par);

            text.clone()
                .then(nested.or_not())
                .map(|(mut text, nested)| {
                    if let Some(mut nested) = nested {
                        text.push(Inline::SoftBreak);
                        text.append(&mut nested);
                    }

                    text
                })
                .separated_by(level_parser())
                .allow_leading()
                .at_least(1)
                .collect()
                .map(|mut content: Vec<Vec<_>>| {
                    for line in &mut content {
                        line.push(Inline::SoftBreak);
                    }
                    content.into_iter().flatten().collect()
                })
                .boxed()
        },
    );

    text.delimited_by(just("["), just("]"))
        .or(paragraph
            .with_ctx(ParserContext { indent: 1 })
            .delimited_by(just("["), soft_break_parser().then(just("]"))))
        .map_with(|content, e| Content {
            content,
            span: e.span(),
        })
}

pub fn args_parser<'src, E, C>(
    expr: E,
    content: C,
) -> impl Parser<'src, &'src str, Args, Extra<'src>>
where
    E: Parser<'src, &'src str, Expr, Extra<'src>>,
    C: Parser<'src, &'src str, Content, Extra<'src>> + 'src,
{
    let arg = ascii::ident()
        .to_ecow()
        .then_ignore(just(": "))
        .or_not()
        .then(expr)
        .map_with(|(name, value), e| Arg {
            name,
            value,
            span: e.span(),
        });

    let args = arg
        .separated_by(just(",").padded())
        .allow_trailing()
        .collect()
        .padded()
        .delimited_by(just("("), just(")"));
    let content = content.or_not();

    args.then(content).map_with(|(args, content), e| Args {
        args,
        content,
        span: e.span(),
    })
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
