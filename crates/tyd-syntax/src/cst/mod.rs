use parasite::{
    chumsky::{prelude::*, Parseable},
    combinators::{Any, End, Identifier, NewLine, NonEmptyVec, PaddedBy, Rec, SeparatedBy},
    Parseable,
};
use terminal::*;

pub mod fmt;
pub mod terminal;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Cst(pub NonEmptyVec<(Block, NonEmptyVec<NewLine>)>, pub End);

// TODO NewLineBlock instead of NewLines in Line and Cst ?

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Block {
    Raw(Raw),
    Heading(Heading),
    BulletList(BulletList),
    OrderedList(OrderedList),
    Table(Table),
    BlockQuote(BlockQuote),
    Paragraph(Paragraph),
    // Expr(Expr),
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
// and use tab as indentation level for lists, block_quote

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Paragraph(pub NonEmptyVec<Line>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct BulletList(pub NonEmptyVec<(Minus, Line)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct OrderedList(pub NonEmptyVec<(Plus, Line)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Table(pub NonEmptyVec<TableRow>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct TableRow(pub Pipe, pub NonEmptyVec<(Elements, Pipe)>, pub NewLine);

// TODO allow multiple levels of Blockquote
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct BlockQuote(pub NonEmptyVec<(RightAngle, Line)>);

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
    Access(Access),
    Quote(Quote),
    Strikeout(Strikeout),
    Emphasis(Emphasis),
    Strong(Strong),
    Enclosed(Enclosed),
    Link(Link),
    Escape(Escape),
    RawInline(RawInline),
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
    Access(Access),
    Strikeout(Strikeout),
    Emphasis(Emphasis),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strikeout(pub Tilde, pub NonEmptyVec<StrikeoutElement>, pub Tilde);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum StrikeoutElement {
    Access(Access),
    Emphasis(Emphasis),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Emphasis(pub Slash, pub NonEmptyVec<EmphasizedElement>, pub Slash);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strong(pub Star, pub NonEmptyVec<EmphasizedElement>, pub Star);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum EmphasizedElement {
    Access(Access),
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
}

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
pub struct RawInline(pub BackTick, pub RawInlineContent, pub BackTick);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Access(pub Pound, pub Identifier, pub Option<CallTail>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct CallTail(
    pub LeftParen,
    pub Args,
    pub RightParen,
    pub Option<Enclosed>,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Args(pub SeparatedBy<PaddedBy<Vec<Space>, Comma>, Arg>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Arg(pub Identifier, pub Colon, pub Vec<Space>, pub Value);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Value {
    Identifier(Identifier),
    String(Str),
    // Number(Number),
    // Bool(Boolean),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Str(pub DoubleQuote, pub StringContent, pub DoubleQuote);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
// pub enum Boolean {
//     True(KwTrue),
//     False(KwFalse),
// }
