use parasite::{
    chumsky::{prelude::*, Parseable},
    combinators::{Any, End, Identifier, NewLine, NonEmptyVec, PaddedBy, Rec},
    Parseable,
};
use terminal::*;

pub mod fmt;
pub mod terminal;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Cst(pub NonEmptyVec<(Block, NonEmptyVec<NewLine>)>, pub End);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Block {
    Raw(Raw),
    Heading(Heading),
    List(List),
    OrderedList(OrderedList),
    Table(Table),
    Blockquote(Blockquote),
    Paragraph(Paragraph),
    // Code(Code),
    // Math(Math),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Raw(
    pub TripleBacktick,
    pub Option<PaddedBy<Vec<Space>, Identifier>>,
    pub NewLine,
    pub RawContent,
    pub TripleBacktick,
    pub NewLine,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct HeadingLevel(pub NonEmptyVec<Equals>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Heading(pub HeadingLevel, pub Line);

// TODO enforce paragaph does not start with tab(4 spaces)
// and use tab as indentation level for lists, blockquote

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
pub struct Elements(pub NonEmptyVec<Element>);

impl Parseable<'static, char> for Elements {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> BoxedParser<'static, char, Self, Self::Error> {
        if !ctx.contains::<Recursive<'static, char, Self, Self::Error>>() {
            let script: Recursive<'static, char, Self, Self::Error> = Recursive::declare();
            ctx.insert(script);

            let elements = NonEmptyVec::parser(ctx).map(Elements);

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
    Inline(Inline),
    Quote(Quote),
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Enclosed(Enclosed),
    Link(Link),
    Escape(Escape),
    Monospace(Monospace),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Inline {
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct SubScript(pub Underscore, pub Character);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct SupScript(pub Caret, pub Character);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Spacing(pub NonEmptyVec<Space>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Quote(
    pub DoubleQuote,
    pub NonEmptyVec<QuoteElement>,
    pub DoubleQuote,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum QuoteElement {
    Inline(Inline),
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strikethrough(pub Tilde, pub NonEmptyVec<StrikethroughElement>, pub Tilde);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum StrikethroughElement {
    Inline(Inline),
    Emphasis(Emphasis),
    Strong(Strong),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Emphasis(pub Slash, pub NonEmptyVec<Inline>, pub Slash);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strong(pub Star, pub NonEmptyVec<Inline>, pub Star);

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
