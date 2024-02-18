use parasite::{
    chumsky::{Parseable, Parser},
    combinators::{Just, NewLine},
    Parseable, Terminal,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Tilde(Just<'~'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Star(Just<'*'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Slash(Just<'/'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Underscore(Just<'_'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Caret(Just<'^'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct BackSlash(Just<'\\'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct BackTick(Just<'`'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct DoubleQuote(Just<'"'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct LeftBracket(Just<'['>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct RightBracket(Just<']'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct At(Just<'@'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct LeftAngle(Just<'<'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct RightAngle(Just<'>'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Pipe(Just<'|'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Space(Just<' '>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Plus(Just<'+'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Minus(Just<'-'>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Equals(Just<'='>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct LinkContent(pub String);

impl Parseable<'_, char> for LinkContent {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
        NewLine::parser()
            .ignored()
            .or(RightAngle::parser().ignored())
            .not()
            .repeated()
            .collect()
            .map(LinkContent)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct MonospaceContent(pub String);

impl Parseable<'_, char> for MonospaceContent {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
        NewLine::parser()
            .ignored()
            .or(BackTick::parser().ignored())
            .not()
            .repeated()
            .collect()
            .map(MonospaceContent)
    }
}
