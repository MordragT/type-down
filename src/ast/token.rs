use parasite::chumsky::{
    prelude::*,
    text::{newline, whitespace},
    Parseable,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl Parseable<'_, char> for Identifier {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        text::ident().map(Identifier)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tilde;

impl Parseable<'_, char> for Tilde {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('~').to(Tilde)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Star;

impl Parseable<'_, char> for Star {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('*').to(Star)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slash;

impl Parseable<'_, char> for Slash {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('/').to(Slash)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Underscore;

impl Parseable<'_, char> for Underscore {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('_').to(Underscore)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Caret;

impl Parseable<'_, char> for Caret {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('^').to(Caret)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackSlash;

impl Parseable<'_, char> for BackSlash {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('\\').to(BackSlash)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackTick;

impl Parseable<'_, char> for BackTick {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('`').to(BackTick)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeftBracket;

impl Parseable<'_, char> for LeftBracket {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('[').to(LeftBracket)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RightBracket;

impl Parseable<'_, char> for RightBracket {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just(']').to(RightBracket)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct At;

impl Parseable<'_, char> for At {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('@').to(At)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeftAngle;

impl Parseable<'_, char> for LeftAngle {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('<').to(LeftAngle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RightAngle;

impl Parseable<'_, char> for RightAngle {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('>').to(RightAngle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pipe;

impl Parseable<'_, char> for Pipe {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('|').to(Pipe)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewLine;

impl Parseable<'_, char> for NewLine {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        newline().to(NewLine)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WhiteSpace;

impl Parseable<'_, char> for WhiteSpace {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        whitespace().to(WhiteSpace)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Space;

impl Parseable<'_, char> for Space {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just(' ').to(Space)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plus;

impl Parseable<'_, char> for Plus {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('+').to(Plus)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Minus;

impl Parseable<'_, char> for Minus {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('-').to(Minus)
    }
}
