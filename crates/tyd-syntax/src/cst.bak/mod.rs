use miette::NamedSource;
use parasite::{
    chumsky::{prelude::*, Context as ParseContext, Parseable},
    combinators::{Any, End, Identifier, NewLine, NonEmptyVec, PaddedBy, Rec, SeparatedBy},
    Parseable,
};
use std::{fs::File, io::Read, path::Path};

use crate::error::{ParseError, TydError};
use terminal::*;

pub mod fmt;
pub mod terminal;

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Cst, TydError> {
    let name = path.as_ref().as_os_str().to_string_lossy().into_owned();

    let mut file = File::open(path)?;
    let mut source = String::new();

    file.read_to_string(&mut source)?;

    // source = source.trim().to_owned();
    // source.push('\n');
    // source.push('\n');

    let mut parse_ctx = ParseContext::new();
    let parser = Cst::parser(&mut parse_ctx);

    let cst = parser.parse(source.as_str()).map_err(|errs| ParseError {
        src: NamedSource::new(name, source),
        related: errs.into_iter().map(Into::into).collect(),
    })?;

    Ok(cst)
}

// TODO store span in cst elements

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Cst(pub Nodes, pub End);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nodes(pub NonEmptyVec<Node>);

impl Parseable<'static, char> for Nodes {
    fn parser(
        ctx: &mut parasite::chumsky::Context,
    ) -> BoxedParser<'static, char, Self, Self::Error> {
        if !ctx.contains::<Recursive<'static, char, Self, Self::Error>>() {
            let nodes: Recursive<'static, char, Self, Self::Error> = Recursive::declare();
            ctx.insert(nodes);

            let nodes = NonEmptyVec::parser(ctx).map(Nodes);

            let parser = ctx
                .get_mut::<Recursive<'static, char, Self, Self::Error>>()
                .unwrap();
            parser.define(nodes);

            return parser.clone().boxed();
        }

        ctx.get::<Recursive<'static, char, Self, Self::Error>>()
            .unwrap()
            .clone()
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Node {
    Raw(Raw),
    Heading(Heading),
    BlockQuote(BlockQuote),
    ListItem(ListItem),
    TableRow(TableRow),
    Label(Label),
    LineBreak(NewLine),
    Plain(Elements),
    // Math(Math),
}

// TODO remove newlines from RAw

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Raw(
    pub TripleBackTick,
    // pub Option<PaddedBy<Vec<Space>, Identifier>>,
    // pub NewLine,
    pub RawContent,
    pub TripleBackTick,
    // pub NewLine,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct HeadingLevel(pub NonEmptyVec<Equals>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Heading(pub HeadingLevel, pub Space, pub Elements);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct ListItem(
    pub Option<Indentation>,
    pub ListItemDelim,
    pub Space,
    pub ListItemContent,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum ListItemContent {
    BlockQuote(Rec<BlockQuote>),
    Plain(Elements),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum ListItemDelim {
    Minus(Minus),
    Plus(Plus),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct TableRow(pub Pipe, pub NonEmptyVec<(TableCell, Pipe)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum TableCell {
    ListItem(ListItem),
    BlockQuote(BlockQuote),
    Plain(Elements),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct BlockQuoteLevel(pub NonEmptyVec<RightAngle>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockQuote(pub BlockQuoteLevel, pub Space, pub BlockQuoteItem);

impl<'a> Parseable<'a, char> for BlockQuote {
    fn parser(ctx: &mut parasite::chumsky::Context) -> BoxedParser<'a, char, Self, Self::Error> {
        if !ctx.contains::<Recursive<'static, char, Self, Self::Error>>() {
            let block_quote: Recursive<'static, char, Self, Self::Error> = Recursive::declare();
            ctx.insert(block_quote);

            let level = BlockQuoteLevel::parser(ctx);
            let space = Space::parser(ctx);
            let item = BlockQuoteItem::parser(ctx);

            let block_quote = level
                .then(space)
                .then(item)
                .map(|((level, space), item)| Self(level, space, item));

            let parser = ctx
                .get_mut::<Recursive<'static, char, Self, Self::Error>>()
                .unwrap();
            parser.define(block_quote);

            return parser.clone().boxed();
        }

        ctx.get::<Recursive<'static, char, Self, Self::Error>>()
            .unwrap()
            .clone()
            .boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum BlockQuoteItem {
    ListItem(ListItem),
    Plain(Elements),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Label(pub At, pub Identifier);

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
    Code(Code),
    Quote(Quote),
    Strikeout(Strikeout),
    Emphasis(Emphasis),
    Strong(Strong),
    // Enclosed(Enclosed),
    Link(Link),
    Escape(Escape),
    RawInline(RawInline),
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
    Comment(Comment),
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
    Code(Code),
    Escape(Escape),
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
    Code(Code),
    Escape(Escape),
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
    Code(Code),
    Escape(Escape),
    SubScript(SubScript),
    SupScript(SupScript),
    Word(Word),
    Spacing(Spacing),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Link(
    pub LeftAngle,
    pub LinkContent,
    pub RightAngle,
    pub Option<Content>,
);

// TODO escape body /[@ / blah blah]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Escape(pub BackSlash, pub Any);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct RawInline(pub BackTick, pub RawInlineContent, pub BackTick);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Comment(pub DoubleSlash, pub CommentContent);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Code(pub Pound, pub Expr);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub enum Expr {
    Access(Access),
    Content(Content),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Access(pub Identifier, pub Option<CallTail>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Content(pub LeftBracket, pub Rec<Nodes>, pub RightBracket);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct CallTail(pub LeftParen, pub Args, pub RightParen, pub Option<Content>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Args(pub SeparatedBy<PaddedBy<Vec<Space>, Comma>, Arg>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parseable)]
pub struct Arg(pub Option<(Identifier, Colon)>, pub Vec<Space>, pub Value);

// TODO make named argument optional
// pub struct Arg(pub Option<(Identifier, Colon, Vec<Space>)>, pub Value);

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
