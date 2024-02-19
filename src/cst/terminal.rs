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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct Character(pub char);

impl Parseable<'_, char> for Character {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
        NewLine::parser()
            .ignored()
            .or(At::parser().ignored())
            .or(Pipe::parser().ignored())
            .or(BackTick::parser().ignored())
            .or(BackSlash::parser().ignored())
            .or(LeftAngle::parser().ignored())
            .or(Space::parser().ignored())
            .or(Underscore::parser().ignored())
            .or(Caret::parser().ignored())
            .or(Star::parser().ignored())
            .or(Slash::parser().ignored())
            .or(Tilde::parser().ignored())
            .or(DoubleQuote::parser().ignored())
            .not()
            .map(Character)
    }
}

impl FromIterator<Character> for String {
    fn from_iter<T: IntoIterator<Item = Character>>(iter: T) -> Self {
        iter.into_iter().map(|c| c.0).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct Word(pub String);

impl Parseable<'_, char> for Word {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
        Character::parser()
            .repeated()
            .at_least(1)
            .collect()
            .map(Word)
    }
}
