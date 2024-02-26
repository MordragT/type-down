use std::fmt;

use parasite::{
    chumsky::{primitive::just, Parseable, Parser},
    combinators::{Just, NewLine},
    Parseable, Terminal,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Tilde(Just<'~'>);

impl fmt::Display for Tilde {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "~")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Star(Just<'*'>);

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Slash(Just<'/'>);

impl fmt::Display for Slash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct DoubleSlash;

impl<'a> Parseable<'a, char> for DoubleSlash {
    fn parser(
        _ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'a, char, Self, Self::Error> {
        just("//").to(Self).boxed()
    }
}

impl fmt::Display for DoubleSlash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "//")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct BackSlash(Just<'\\'>);

impl fmt::Display for BackSlash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Underscore(Just<'_'>);

impl fmt::Display for Underscore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Caret(Just<'^'>);

impl fmt::Display for Caret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct BackTick(Just<'`'>);

impl fmt::Display for BackTick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct TripleBackTick;

impl<'a> Parseable<'a, char> for TripleBackTick {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'a, char, Self, Self::Error> {
        just("```").to(Self).boxed()
    }
}

impl fmt::Display for TripleBackTick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "```")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct DoubleQuote(Just<'"'>);

impl fmt::Display for DoubleQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct LeftParen(Just<'('>);

impl fmt::Display for LeftParen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct RightParen(Just<')'>);

impl fmt::Display for RightParen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct LeftBracket(Just<'['>);

impl fmt::Display for LeftBracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct RightBracket(Just<']'>);

impl fmt::Display for RightBracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct LeftAngle(Just<'<'>);

impl fmt::Display for LeftAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct RightAngle(Just<'>'>);

impl fmt::Display for RightAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct At(Just<'@'>);

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Pound(Just<'#'>);

impl fmt::Display for Pound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Pipe(Just<'|'>);

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Space(Just<' '>);

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct Indentation;

impl<'a> Parseable<'a, char> for Indentation {
    fn parser(
        _ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'a, char, Self, Self::Error> {
        just("    ").to(Self).boxed()
    }
}

impl fmt::Display for Indentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "    ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Plus(Just<'+'>);

impl fmt::Display for Plus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Minus(Just<'-'>);

impl fmt::Display for Minus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Equals(Just<'='>);

impl fmt::Display for Equals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Comma(Just<','>);

impl fmt::Display for Comma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ",")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct Colon(Just<':'>);

impl fmt::Display for Colon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct CommentContent(pub String);

impl<'a> Parseable<'a, char> for CommentContent {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'a, char, Self, Self::Error> {
        NewLine::parser(ctx)
            .ignored()
            .not()
            .repeated()
            .collect()
            .map(CommentContent)
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct StringContent(pub String);

impl fmt::Display for StringContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for StringContent {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'static, char, Self, Self::Error> {
        DoubleQuote::parser(ctx)
            .ignored()
            .not()
            .repeated()
            .collect()
            .map(StringContent)
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct RawContent(pub String);

impl fmt::Display for RawContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for RawContent {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'static, char, Self, Self::Error> {
        TripleBackTick::parser(ctx)
            .ignored()
            .not()
            .repeated()
            .collect()
            .map(RawContent)
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct RawInlineContent(pub String);

impl fmt::Display for RawInlineContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for RawInlineContent {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'static, char, Self, Self::Error> {
        let newline = NewLine::parser(ctx);
        let backtick = BackTick::parser(ctx).ignored();

        newline
            .ignored()
            .or(backtick)
            .not()
            .repeated()
            .collect()
            .map(RawInlineContent)
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct LinkContent(pub String);

impl fmt::Display for LinkContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for LinkContent {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'static, char, Self, Self::Error> {
        let newline = NewLine::parser(ctx);
        let right_angle = RightAngle::parser(ctx).ignored();

        newline
            .ignored()
            .or(right_angle)
            .not()
            .repeated()
            .collect()
            .map(LinkContent)
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Special {
    NewLine(NewLine),
    Space(Space),
    At(At),
    Pound(Pound),
    Pipe(Pipe),
    Underscore(Underscore),
    Caret(Caret),
    BackSlash(BackSlash),
    Slash(Slash),
    Star(Star),
    Tilde(Tilde),
    Minus(Minus),
    Plus(Plus),
    BackTick(BackTick),
    DoubleQuote(DoubleQuote),
    LeftAngle(LeftAngle),
    RightAngle(RightAngle),
    LeftBracket(LeftBracket),
    RightBracket(RightBracket),
    LeftParen(LeftParen),
    RightParen(RightParen),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum SpecialInline {
    Space(Space),
    NewLine(NewLine),
    At(At),
    Pound(Pound),
    Underscore(Underscore),
    Caret(Caret),
    BackSlash(BackSlash),
    Slash(Slash),
    Star(Star),
    Tilde(Tilde),
    BackTick(BackTick),
    DoubleQuote(DoubleQuote),
    LeftAngle(LeftAngle),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct Character(pub char);

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for Character {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'static, char, Self, Self::Error> {
        let special = SpecialInline::parser(ctx);

        special.not().map(Character).boxed()
    }
}

impl FromIterator<Character> for String {
    fn from_iter<T: IntoIterator<Item = Character>>(iter: T) -> Self {
        iter.into_iter().map(|c| c.0).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct Word(pub String);

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for Word {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> parasite::chumsky::prelude::BoxedParser<'static, char, Self, Self::Error> {
        Character::parser(ctx)
            .repeated()
            .at_least(1)
            .collect()
            .map(Word)
            .boxed()
    }
}
