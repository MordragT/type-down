use std::convert::Infallible;

use parol_runtime::{Location, Token as ParolToken};

use crate::grammar_trait::{BlockquoteSign, Element, HeadingSign};

#[derive(Clone, Debug)]
pub struct Token {
    pub text: String,
    pub location: Location,
}

impl TryFrom<&ParolToken<'_>> for Token {
    type Error = anyhow::Error;

    fn try_from(t: &ParolToken<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            text: t.text().to_owned(),
            location: t.location.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct HeadingLevel(pub usize);

impl TryFrom<&HeadingSign<'_>> for HeadingLevel {
    type Error = anyhow::Error;

    fn try_from(sign: &HeadingSign<'_>) -> Result<Self, Self::Error> {
        Ok(Self(sign.heading_sign_list.len()))
    }
}

#[derive(Clone, Debug)]
pub struct BlockquoteLevel(pub usize);

impl TryFrom<&BlockquoteSign<'_>> for BlockquoteLevel {
    type Error = anyhow::Error;

    fn try_from(sign: &BlockquoteSign<'_>) -> Result<Self, Self::Error> {
        Ok(Self(sign.blockquote_sign_list.len()))
    }
}

// pub struct Link {
//     pub link: Token,
//     pub anchor: Option<Elements>,
// }

// pub struct Escape {
//     pub character: Token,
// }

// pub struct Monospace {
//     pub content: Vec<Token>,
// }

// pub struct Subscript {
//     pub content: Element,
// }

// pub struct Supscript {
//     pub content: Element,
// }

// // pub enum Element {
// //     Word(Token),
// //     List(Elements),
// //     Link(Link),
// //     Escape(Escape),
// //     Monospace(Monospace),
// //     Subscript(Subscript),
// //     Supscript(Supscript),
// // }

// pub type Elements = Vec<Element>;

// pub struct Label {
//     pub identifier: Token,
// }

// pub struct Line {
//     pub elements: Elements,
//     pub label: Option<Label>,
// }

// // Markup Blocks ------------------------
// pub struct Heading {
//     pub level: usize,
//     pub line: Line,
// }

// pub struct Paragraph {
//     pub lines: Vec<Line>,
// }

// pub struct List {
//     pub lines: Vec<Line>,
// }

// pub struct OrderedList {
//     pub lines: Vec<Line>,
// }

// pub struct Table {
//     pub table: Vec<Vec<Elements>>,
// }

// pub struct Blockquote {
//     pub levels: Vec<usize>,
//     pub lines: Vec<Line>,
// }
