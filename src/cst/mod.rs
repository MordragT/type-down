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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Elements(pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, Element>>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Element {
    Quote(Quote),
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Link(Link),
    Escape(Escape),
    Monospace(Monospace),
    // Enclosed((LeftBracket, Vec<Element>, RightBracket)),
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

// TODO link body <www.example.com>[Hier klicken]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Link(pub LeftAngle, pub LinkContent, pub RightAngle);

// TODO escape body /[@ / blah blah]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Escape(pub BackSlash, pub Any);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Monospace(pub BackTick, pub MonospaceContent, pub BackTick);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Script(pub Word, pub Option<ScriptTail>);

impl Parseable<'_, char> for Script {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
        let mut script = Recursive::declare();

        let sub_script = Underscore::parser()
            .then(Character::parser())
            .then(script.clone())
            .map(|((underscore, c), script)| ScriptTail::Sub(underscore, c, Rec(Box::new(script))));

        let sup_script = Caret::parser()
            .then(Character::parser())
            .then(script.clone())
            .map(|((caret, c), script)| ScriptTail::Sup(caret, c, Rec(Box::new(script))));

        script.define(
            Word::parser()
                .then(sub_script.or(sup_script).or_not())
                .map(|(word, tail)| Script(word, tail)),
        );

        script
    }
}

// TODO script body X_[long subscript]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptTail {
    Sub(Underscore, Character, Rec<Script>),
    Sup(Caret, Character, Rec<Script>),
}
