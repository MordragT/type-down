use std::fmt;

use parasite::{
    chumsky::{Parseable, Parser},
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
pub struct BackSlash(Just<'\\'>);

impl fmt::Display for BackSlash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct BackTick(Just<'`'>);

impl fmt::Display for BackTick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable, Terminal)]
pub struct TripleBacktick(BackTick, BackTick, BackTick);

impl fmt::Display for TripleBacktick {
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
pub struct At(Just<'@'>);

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@")
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
        TripleBacktick::parser(ctx)
            .ignored()
            .not()
            .repeated()
            .collect()
            .map(RawContent)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Terminal)]
pub struct MonospaceContent(pub String);

impl fmt::Display for MonospaceContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable<'static, char> for MonospaceContent {
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
            .map(MonospaceContent)
            .boxed()
    }
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
        let newline = NewLine::parser(ctx).ignored();
        let at = At::parser(ctx).ignored();
        let pipe = Pipe::parser(ctx).ignored();
        let back_tick = BackTick::parser(ctx).ignored();
        let back_slash = BackSlash::parser(ctx).ignored();
        let left_angle = LeftAngle::parser(ctx).ignored();
        let space = Space::parser(ctx).ignored();
        let underscore = Underscore::parser(ctx).ignored();
        let caret = Caret::parser(ctx).ignored();
        let star = Star::parser(ctx).ignored();
        let slash = Slash::parser(ctx).ignored();
        let tilde = Tilde::parser(ctx).ignored();
        let double_quote = DoubleQuote::parser(ctx).ignored();
        let left_bracket = LeftBracket::parser(ctx).ignored();
        let right_bracket = RightBracket::parser(ctx).ignored();

        newline
            .or(at)
            .or(pipe)
            .or(back_tick)
            .or(back_slash)
            .or(left_angle)
            .or(space)
            .or(underscore)
            .or(caret)
            .or(star)
            .or(slash)
            .or(tilde)
            .or(double_quote)
            .or(left_bracket)
            .or(right_bracket)
            .not()
            .map(Character)
            .boxed()
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
