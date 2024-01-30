use html_tag::HtmlTag;
use parasite::{
    chumsky::{prelude::*, text::newline, Parseable},
    combinators::NonEmptyVec,
    Parseable,
};
use token::*;

pub mod token;

// TODO track spans in ast ?

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeDown {
    // skip newline
    blocks: Vec<Block>,
}

impl Into<HtmlTag> for TypeDown {
    fn into(self) -> HtmlTag {
        let mut div = HtmlTag::new("div");

        for block in self.blocks {
            div.add_child(block.into())
        }

        div
    }
}

impl Parseable<'_, char> for TypeDown {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        // TODO allow whitespace in empty newlines

        Block::parser()
            .then_ignore(newline().repeated().at_least(1))
            .repeated()
            .at_least(1)
            .collect()
            .map(|blocks| TypeDown { blocks })
            .then_ignore(end())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Block {
    Mark(MarkBlock),
    // Raw(RawBlock),
    // Code(CodeBlock),
    // Math(MathBlock),
}

impl Into<HtmlTag> for Block {
    fn into(self) -> HtmlTag {
        match self {
            Self::Mark(block) => block.into(),
        }
    }
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

impl Into<HtmlTag> for MarkBlock {
    fn into(self) -> HtmlTag {
        match self {
            Self::Heading(heading) => heading.into(),
            Self::List(list) => list.into(),
            Self::OrderedList(ordered) => ordered.into(),
            Self::Table(table) => table.into(),
            Self::Blockquote(blockquote) => blockquote.into(),
            Self::Paragraph(paragraph) => paragraph.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeadingLevel(usize);

impl Parseable<'_, char> for HeadingLevel {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        just('=')
            .repeated()
            .at_least(1)
            .at_most(6)
            .collect::<String>()
            .map(|level| HeadingLevel(level.len()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Heading {
    level: HeadingLevel,
    line: Line,
}

impl Into<HtmlTag> for Heading {
    fn into(self) -> HtmlTag {
        let Self { level, line } = self;

        HtmlTag::new(&format!("h{}", level.0)).with_child(line.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Paragraph {
    elements: NonEmptyVec<Line>,
}

impl Into<HtmlTag> for Paragraph {
    fn into(self) -> HtmlTag {
        let mut p = HtmlTag::new("p");

        for line in self.elements.0 {
            p.add_child(line.into())
        }

        p
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct List {
    elements: NonEmptyVec<(Minus, Line)>,
}

impl Into<HtmlTag> for List {
    fn into(self) -> HtmlTag {
        let mut ul = HtmlTag::new("ul");

        for (_, line) in self.elements.0 {
            let li = HtmlTag::new("li").with_child(line.into());
            ul.add_child(li);
        }

        ul
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct OrderedList {
    elements: NonEmptyVec<(Plus, Line)>,
}

impl Into<HtmlTag> for OrderedList {
    fn into(self) -> HtmlTag {
        let mut ol = HtmlTag::new("ol");

        for (_, line) in self.elements.0 {
            let li = HtmlTag::new("li").with_child(line.into());
            ol.add_child(li);
        }

        ol
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    rows: Vec<Vec<Elements>>,
}

impl Parseable<'_, char> for Table {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        let row = Pipe::parser()
            .ignore_then(
                Elements::parser()
                    .separated_by(just('|'))
                    .at_least(1)
                    .collect(),
            )
            .then_ignore(Pipe::parser())
            .then_ignore(NewLine::parser());

        row.repeated()
            .at_least(1)
            .collect()
            .map(|rows| Table { rows })
    }
}

impl Into<HtmlTag> for Table {
    fn into(self) -> HtmlTag {
        let mut table = HtmlTag::new("table");

        for row in self.rows {
            let mut tr = HtmlTag::new("tr");

            for el in row {
                let mut td = HtmlTag::new("td").with_child(el.into());
                tr.add_child(td);
            }

            table.add_child(tr);
        }

        table
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Blockquote {
    // TODO allow multiple levels of Blockquote
    elements: NonEmptyVec<(RightAngle, Line)>,
}

impl Into<HtmlTag> for Blockquote {
    fn into(self) -> HtmlTag {
        let mut blockquote = HtmlTag::new("blockquote");

        for (_, line) in self.elements.0 {
            blockquote.add_child(line.into());
        }

        blockquote
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Label(At, Identifier);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Line(Elements, Option<Label>, NewLine);

impl Into<HtmlTag> for Line {
    fn into(self) -> HtmlTag {
        self.0.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elements(Vec<Element>);

impl Parseable<'_, char> for Elements {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        Element::parser()
            .separated_by(just(' ').repeated().at_least(1))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect()
            .map(Elements)
    }
}

impl Into<HtmlTag> for Elements {
    fn into(self) -> HtmlTag {
        let mut div = HtmlTag::new("div");

        for el in self.0 {
            div.add_child(el.into())
        }

        div
    }
}

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

impl Into<HtmlTag> for Element {
    fn into(self) -> HtmlTag {
        match self {
            Self::Quote(quote) => quote.into(),
            Self::Strikethrough(strike) => strike.into(),
            Self::Emphasis(emphasis) => emphasis.into(),
            Self::Strong(strong) => strong.into(),
            Self::Link(link) => link.into(),
            Self::Escape(escape) => escape.into(),
            Self::Monospace(monospace) => monospace.into(),
            Self::Word(word) => HtmlTag::new("div").with_body(&word.to_body()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote(Vec<QuoteElement>);

impl Parseable<'_, char> for Quote {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        QuoteElement::parser()
            .separated_by(just(' '))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect()
            .map(Quote)
            .delimited_by(just('\"'), just('\"'))
    }
}

impl Into<HtmlTag> for Quote {
    fn into(self) -> HtmlTag {
        let mut q = HtmlTag::new("q");

        for el in self.0 {
            q.add_child(el.into());
        }

        q
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum QuoteElement {
    Strikethrough(Strikethrough),
    Emphasis(Emphasis),
    Strong(Strong),
    Word(Word),
}

impl Into<HtmlTag> for QuoteElement {
    fn into(self) -> HtmlTag {
        match self {
            Self::Strikethrough(strike) => strike.into(),
            Self::Emphasis(emphasis) => emphasis.into(),
            Self::Strong(strong) => strong.into(),
            Self::Word(word) => HtmlTag::new("div").with_body(&word.to_body()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strikethrough(Vec<StrikethroughElement>);

impl Parseable<'_, char> for Strikethrough {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        StrikethroughElement::parser()
            .separated_by(just(' '))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect()
            .map(Strikethrough)
            .delimited_by(just('~'), just('~'))
    }
}

impl Into<HtmlTag> for Strikethrough {
    fn into(self) -> HtmlTag {
        let mut del = HtmlTag::new("del");

        for el in self.0 {
            del.add_child(el.into())
        }

        del
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum StrikethroughElement {
    Emphasis(Emphasis),
    Strong(Strong),
    Word(Word),
}

impl Into<HtmlTag> for StrikethroughElement {
    fn into(self) -> HtmlTag {
        match self {
            Self::Emphasis(emphasis) => emphasis.into(),
            Self::Strong(strong) => strong.into(),
            Self::Word(word) => HtmlTag::new("div").with_body(&word.to_body()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Emphasis(Vec<Word>);

impl Parseable<'_, char> for Emphasis {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        Word::parser()
            .separated_by(just(' '))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect()
            .map(Emphasis)
            .delimited_by(just('/'), just('/'))
    }
}

impl Into<HtmlTag> for Emphasis {
    fn into(self) -> HtmlTag {
        let mut body = String::new();

        for word in self.0 {
            body.push_str(&word.to_body());
        }

        HtmlTag::new("em").with_body(&body)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strong(Vec<Word>);

impl Parseable<'_, char> for Strong {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        Word::parser()
            .separated_by(just(' '))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect()
            .map(Strong)
            .delimited_by(just('*'), just('*'))
    }
}

impl Into<HtmlTag> for Strong {
    fn into(self) -> HtmlTag {
        let mut body = String::new();

        for word in self.0 {
            body.push_str(&word.to_body());
        }

        HtmlTag::new("strong").with_body(&body)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    link: String,
    children: Option<Vec<Element>>,
}

impl Parseable<'_, char> for Link {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        NewLine::parser()
            .ignored()
            .or(RightAngle::parser().ignored())
            .not()
            .repeated()
            .collect()
            .map(|link| Link {
                link,
                children: None,
            })
            .delimited_by(just('<'), just('>'))
        // .then(
        //     Element::parser()
        //         .repeated()
        //         .at_least(1)
        //         .collect::<Vec<_>>()
        //         .delimited_by(just("["), just(']'))
        //         .or_not(),
        // )
        // .map(|(link, children)| Link { link, children })
    }
}

impl Into<HtmlTag> for Link {
    fn into(self) -> HtmlTag {
        HtmlTag::new("div").with_body(&self.link)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Escape(char);

impl Parseable<'_, char> for Escape {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        BackSlash::parser().ignore_then(any()).map(Escape)
    }
}

impl Into<HtmlTag> for Escape {
    fn into(self) -> HtmlTag {
        // TODO escape html style
        HtmlTag::new("div").with_body(&self.0.escape_default().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monospace(String);

impl Parseable<'_, char> for Monospace {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
        NewLine::parser()
            .ignored()
            .or(BackTick::parser().ignored())
            .not()
            .repeated()
            .collect()
            .map(Monospace)
            .delimited_by(just('`'), just('`'))
    }
}

impl Into<HtmlTag> for Monospace {
    fn into(self) -> HtmlTag {
        HtmlTag::new("code").with_body(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Script {
    Sub(Box<Word>),
    Sup(Box<Word>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    word: String,
    script: Script,
}

impl Parseable<'_, char> for Word {
    fn parser() -> impl Parser<char, Self, Error = Self::Error> {
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

impl Word {
    fn to_body(self) -> String {
        let Self { word, script } = self;

        match script {
            Script::None => word,
            Script::Sub(word) => HtmlTag::new("sub").with_body(&word.to_body()).to_html(),
            Script::Sup(word) => HtmlTag::new("sup").with_body(&word.to_body()).to_html(),
        }
    }
}

// impl Into<HtmlTag> for Word {
//     fn into(self) -> HtmlTag {
//         let Self { word, sub, sup } = self;

//         let mut div = HtmlTag::new("div").with_body(&word);

//         if let Some(item) = sub {
//             let mut sub = HtmlTag::new("sub");
//             sub.add_child(Word::into(*item));

//             div.add_child(sub);
//         } else if let Some(item) = sup {
//             let mut sup = HtmlTag::new("sup");
//             sup.add_child(Word::into(*item));

//             div.add_child(sup);
//         }

//         div
//     }
// }
