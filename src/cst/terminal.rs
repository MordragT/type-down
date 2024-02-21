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
pub struct TripleBacktick(BackTick, BackTick, BackTick);

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
pub struct RawContent(pub String);

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
