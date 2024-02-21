use parasite::{
    chumsky::{prelude::*, Parseable},
    combinators::{Any, End, Identifier, NewLine, NonEmptyVec, PaddedBy, Rec, SeparatedBy},
    Parseable,
};
use terminal::*;

pub mod terminal;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Cst(pub NonEmptyVec<(Block, NonEmptyVec<NewLine>)>, pub End);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Block {
    Raw(RawBlock),
    Mark(MarkBlock),
    // Code(CodeBlock),
    // Math(MathBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct RawBlock(
    pub TripleBacktick,
    pub Option<PaddedBy<Vec<Space>, Identifier>>,
    pub NewLine,
    pub RawContent,
    pub TripleBacktick,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum MarkBlock {
    Heading(Heading),
    List(List),
    OrderedList(OrderedList),
    Table(Table),
    Blockquote(Blockquote),
    Paragraph(Paragraph),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct HeadingLevel(pub NonEmptyVec<Equals>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Heading(pub HeadingLevel, pub Line);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Paragraph(pub NonEmptyVec<Line>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct List(pub NonEmptyVec<(Minus, Line)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct OrderedList(pub NonEmptyVec<(Plus, Line)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Table(pub NonEmptyVec<TableRow>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct TableRow(pub Pipe, pub NonEmptyVec<(Elements, Pipe)>, pub NewLine);

// TODO allow multiple levels of Blockquote
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Blockquote(pub NonEmptyVec<(RightAngle, Line)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Label(pub At, pub Identifier);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Line(pub Elements, pub Option<Label>, pub NewLine);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elements(pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, Element>>);

impl Parseable<'static, char> for Elements {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> BoxedParser<'static, char, Self, Self::Error> {
        if !ctx.contains::<Recursive<'static, char, Self, Self::Error>>() {
            let script: Recursive<'static, char, Self, Self::Error> = Recursive::declare();
            ctx.insert(script);

            let elements = PaddedBy::parser(ctx).map(Elements);

            let parser = ctx
                .get_mut::<Recursive<'static, char, Self, Self::Error>>()
                .unwrap();
            parser.define(elements);

            return parser.clone().boxed();
        }

        ctx.get::<Recursive<'static, char, Self, Self::Error>>()
            .unwrap()
            .clone()
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Element {
    Quote(Quote),
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Enclosed(Enclosed),
    Link(Link),
    Escape(Escape),
    Monospace(Monospace),
    Script(Script),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Quote(
    pub DoubleQuote,
    pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, QuoteElement>>,
    pub DoubleQuote,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum QuoteElement {
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Script(Script),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strikethrough(
    pub Tilde,
    pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, StrikethroughElement>>,
    pub Tilde,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum StrikethroughElement {
    Emphasis(Emphasis),
    Strong(Strong),
    Script(Script),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Emphasis(
    pub Slash,
    pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, Script>>,
    pub Slash,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strong(
    pub Star,
    pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, Script>>,
    pub Star,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Enclosed(pub LeftBracket, pub Rec<Elements>, pub RightBracket);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Link(
    pub LeftAngle,
    pub LinkContent,
    pub RightAngle,
    pub Option<Enclosed>,
);

// TODO escape body /[@ / blah blah]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Escape(pub BackSlash, pub Any);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Monospace(pub BackTick, pub MonospaceContent, pub BackTick);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Script(pub Word, pub Option<ScriptTail>);

impl Parseable<'static, char> for Script {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> BoxedParser<'static, char, Self, Self::Error> {
        if !ctx.contains::<Recursive<'static, char, Self, Self::Error>>() {
            let script: Recursive<'static, char, Self, Self::Error> = Recursive::declare();
            ctx.insert(script);

            let word = Word::parser(ctx);
            let script = Option::<ScriptTail>::parser(ctx);

            let parser = ctx
                .get_mut::<Recursive<'static, char, Self, Self::Error>>()
                .unwrap();
            parser.define(word.then(script).map(|(word, script)| Script(word, script)));

            return parser.clone().boxed();
        }

        ctx.get::<Recursive<'static, char, Self, Self::Error>>()
            .unwrap()
            .clone()
            .boxed()
    }
}

// TODO script body X_[long subscript]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum ScriptTail {
    Sub(Underscore, Character, Rec<Script>),
    Sup(Caret, Character, Rec<Script>),
}
