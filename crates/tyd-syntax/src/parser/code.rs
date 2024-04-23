use chumsky::{
    prelude::*,
    text::{ascii, digits, newline},
};

use super::{
    markup::{indent_parser, level_parser, soft_break_parser},
    Extra, ParserContext, ParserExt,
};
use crate::{ast::*, Span};

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
        let ident = ascii::ident().to_ecow().map_with(|ident, e| Ident {
            ident,
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
            .map_with(|block, e| Expr::Block(block, e.span()));
        let literal = literal_parser().map_with(|literal, e| Expr::Literal(literal, e.span()));

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
                .ignore_with_ctx(par)
                .map_with(|nested, e| (nested, e.span()));

            let el = text
                .clone()
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
                .allow_leading()
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
        .map_with(|ident, e| Ident {
            ident,
            span: e.span(),
        })
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
