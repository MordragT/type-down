use chumsky::{
    prelude::*,
    text::{ascii, digits, newline},
};

use crate::{inline::Inline, Span};

type Extra<'src> = extra::Err<Rich<'src, char, Span>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code<'src> {
    pub expr: Expr<'src>,
    pub span: Span,
}

pub fn code_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Code<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>> + 'src,
{
    just("#")
        .ignore_then(expr_parser(inline))
        .map_with(|expr, e| Code {
            expr,
            span: e.span(),
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'src> {
    Ident(&'src str),
    Call(Call<'src>),
    Literal(Literal<'src>),
    Block(Vec<Expr<'src>>),
}

pub fn expr_parser<'src, I>(inline: I) -> impl Parser<'src, &'src str, Expr<'src>, Extra<'src>>
where
    I: Parser<'src, &'src str, Inline<'src>, Extra<'src>> + 'src,
{
    recursive(|expr| {
        let ident = ascii::ident();
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

        choice((access, block, literal)).boxed()
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'src> {
    pub ident: &'src str,
    pub args: Vec<Arg<'src>>,
    pub content: Option<Vec<Inline<'src>>>,
}

pub fn args_parser<'src, E>(expr: E) -> impl Parser<'src, &'src str, Vec<Arg<'src>>, Extra<'src>>
where
    E: Parser<'src, &'src str, Expr<'src>, Extra<'src>>,
{
    let arg = ascii::ident()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg<'src> {
    pub name: Option<&'src str>,
    pub value: Expr<'src>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'src> {
    Boolean(bool),
    Int(i64),
    // Float(f64),
    Str(&'src str),
}

pub fn literal_parser<'src>() -> impl Parser<'src, &'src str, Literal<'src>, Extra<'src>> {
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
        .delimited_by(just("\""), just("\""))
        .to_slice()
        .map(Literal::Str);

    choice((boolean, int, string))
}
