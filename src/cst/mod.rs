use parasite::{
    chumsky::{prelude::*, Parseable},
    combinators::{Any, End, Identifier, NewLine, NonEmptyVec, PaddedBy, SeparatedBy},
    Parseable,
};
use terminal::*;

pub mod terminal;

// TODO track spans in ast ?

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Cst(pub NonEmptyVec<(Block, NonEmptyVec<NewLine>)>, pub End);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Block {
    Mark(MarkBlock),
    // Raw(RawBlock),
    // Code(CodeBlock),
    // Math(MathBlock),
}

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
    Word(Word),
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
    Word(Word),
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
    Word(Word),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Emphasis(
    pub Slash,
    pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, Word>>,
    pub Slash,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Strong(
    pub Star,
    pub PaddedBy<Vec<Space>, SeparatedBy<NonEmptyVec<Space>, Word>>,
    pub Star,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Link(pub LeftAngle, pub LinkContent, pub RightAngle);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Link {
//     link: String,
//     children: Option<Vec<Element>>,
// }

// impl Parseable<'_, char> for Link {
//     fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
//         NewLine::parser()
//             .ignored()
//             .or(RightAngle::parser().ignored())
//             .not()
//             .repeated()
//             .collect()
//             .map(|link| Link {
//                 link,
//                 children: None,
//             })
//             .delimited_by(just('<'), just('>'))
//         // .then(
//         //     Element::parser()
//         //         .repeated()
//         //         .at_least(1)
//         //         .collect::<Vec<_>>()
//         //         .delimited_by(just("["), just(']'))
//         //         .or_not(),
//         // )
//         // .map(|(link, children)| Link { link, children })
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Escape(pub BackSlash, pub Any);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Monospace(pub BackTick, pub MonospaceContent, pub BackTick);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Script {
    Sub(Box<Word>),
    Sup(Box<Word>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    pub word: String,
    pub script: Script,
}

impl Parseable<'_, char> for Word {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> + Clone {
        let mut word = Recursive::declare();

        let text = NewLine::parser()
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
            .not()
            .repeated()
            .at_least(1)
            .collect();

        word.define(
            text.then(
                Underscore::parser()
                    .ignore_then(word.clone())
                    .map(|script| Script::Sub(Box::new(script)))
                    .or(Caret::parser()
                        .ignore_then(word.clone())
                        .map(|script| Script::Sup(Box::new(script))))
                    .or_not()
                    .map(|script| match script {
                        Some(script) => script,
                        None => Script::None,
                    }),
            )
            .map(|(word, script)| Word { word, script }),
        );

        //     text.then(
        //         Underscore::parser()
        //         .ignore_then(word.clone())
        //         .or(Caret::parser().ignore_then(word.clone()))
        //         .or_not(),
        // .map(|((word, sub))| Word {
        //     word,
        //     sub: sub.map(Box::new),
        //     // sup: sup.map(Box::new),
        // })

        word
    }
}
