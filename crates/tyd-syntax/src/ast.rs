use ecow::EcoString;

use crate::{kind::SyntaxKind, node::Node, Span};

pub trait TypedNode<'a>: Sized + 'a {
    fn from_node(node: &'a Node) -> Option<Self>;
    fn to_node(self) -> &'a Node;

    fn span(self) -> Span {
        self.to_node().span()
    }
}

macro_rules! node {
    ($(#[$attr:meta])* $name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        $(#[$attr])*
        pub struct $name<'a>(&'a Node);

        impl<'a> TypedNode<'a> for $name<'a> {
            #[inline]
            fn from_node(node: &'a Node) -> Option<Self> {
                if node.kind() == SyntaxKind::$name {
                    Some(Self(node))
                } else {
                    Option::None
                }
            }

            #[inline]
            fn to_node(self) -> &'a Node {
                self.0
            }
        }
    };
}

node! {
    /// A parsed file
    Document
}

impl<'a> IntoIterator for Document<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Block<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Block::from_node)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Block<'a> {
    Raw(Raw<'a>),
    Heading(Heading<'a>),
    Table(Table<'a>),
    List(List<'a>),
    Enum(Enum<'a>),
    Terms(Terms<'a>),
    Paragraph(Paragraph<'a>),
    Plain(Plain<'a>),
}

impl<'a> TypedNode<'a> for Block<'a> {
    fn from_node(node: &'a Node) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Raw => Raw::from_node(node).map(Self::Raw),
            SyntaxKind::Heading => Heading::from_node(node).map(Self::Heading),
            SyntaxKind::Table => Table::from_node(node).map(Self::Table),
            SyntaxKind::List => List::from_node(node).map(Self::List),
            SyntaxKind::Enum => Enum::from_node(node).map(Self::Enum),
            SyntaxKind::Terms => Terms::from_node(node).map(Self::Terms),
            SyntaxKind::Paragraph => Paragraph::from_node(node).map(Self::Paragraph),
            SyntaxKind::Plain => Plain::from_node(node).map(Self::Plain),
            _ => None,
        }
    }

    fn to_node(self) -> &'a Node {
        match self {
            Self::Raw(r) => r.to_node(),
            Self::Heading(h) => h.to_node(),
            Self::Table(t) => t.to_node(),
            Self::List(l) => l.to_node(),
            Self::Enum(e) => e.to_node(),
            Self::Terms(t) => t.to_node(),
            Self::Paragraph(p) => p.to_node(),
            Self::Plain(p) => p.to_node(),
        }
    }
}

node! {
    /// Raw text with possible syntax highlighting
    Raw
}

impl<'a> Raw<'a> {
    pub fn text(self) -> Text<'a> {
        self.0
            .children()
            .find_map(Text::from_node)
            .expect("raw expects text")
    }

    pub fn lang(self) -> Option<RawLang<'a>> {
        self.0.children().find_map(RawLang::from_node)
    }
}

node! {
    /// A language tag
    RawLang
}

impl<'a> RawLang<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

node! {
    /// A heading
    Heading
}

impl<'a> Heading<'a> {
    pub fn depth(self) -> u8 {
        self.0
            .children()
            .find_map(HeadingMarker::from_node)
            .expect("heading expects marker")
            .depth()
    }

    pub fn content(self) -> Content<'a> {
        self.0
            .children()
            .find_map(Content::from_node)
            .expect("heading expects content")
    }

    pub fn label(self) -> Option<Label<'a>> {
        self.0.children().rev().find_map(Label::from_node)
    }
}

node! {
    HeadingMarker
}

impl HeadingMarker<'_> {
    pub fn depth(self) -> u8 {
        self.0.text().len() as u8
    }
}

node! {
    Table
}

impl<'a> Table<'a> {
    pub fn col_count(self) -> usize {
        self.rows().next().unwrap().col_count()
    }

    pub fn label(self) -> Option<Label<'a>> {
        self.0.children().rev().find_map(Label::from_node)
    }

    pub fn rows(self) -> impl Iterator<Item = TableRow<'a>> {
        self.0.children().filter_map(TableRow::from_node)
    }
}

node! {
    TableRow
}

impl<'a> TableRow<'a> {
    pub fn col_count(self) -> usize {
        self.into_iter().count()
    }
}

impl<'a> IntoIterator for TableRow<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Block<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Block::from_node)
    }
}

node! {
    List
}

impl<'a> IntoIterator for List<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = ListItem<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(ListItem::from_node)
    }
}

node! {
    ListItem
}

impl<'a> IntoIterator for ListItem<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Block<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Block::from_node)
    }
}

node! {
    Enum
}

impl<'a> IntoIterator for Enum<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = EnumItem<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(EnumItem::from_node)
    }
}

node! {
    EnumItem
}

impl<'a> IntoIterator for EnumItem<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Block<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Block::from_node)
    }
}

node! {
    Terms
}

impl<'a> IntoIterator for Terms<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = TermItem<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(TermItem::from_node)
    }
}

node! {
    TermItem
}

impl<'a> TermItem<'a> {
    pub fn term(&self) -> Content<'a> {
        self.0
            .children()
            .find_map(Content::from_node)
            .expect("term-item expects term")
    }

    pub fn desc(&self) -> Content<'a> {
        self.0
            .children()
            .rev()
            .find_map(Content::from_node)
            .expect("term-item expects desc")
    }
}

node! {
    Paragraph
}

impl<'a> IntoIterator for Paragraph<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Plain
}

impl<'a> IntoIterator for Plain<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Content
}

impl<'a> IntoIterator for Content<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Label
}

impl<'a> Label<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Inline<'a> {
    Quote(Quote<'a>),
    Strikeout(Strikeout<'a>),
    Emphasis(Emphasis<'a>),
    Strong(Strong<'a>),
    Subscript(Subscript<'a>),
    Supscript(Supscript<'a>),
    Link(Link<'a>),
    Ref(Ref<'a>),
    RawInline(RawInline<'a>),
    MathInline(MathInline<'a>),
    Comment(Comment<'a>),
    Escape(Escape<'a>),
    Word(Word<'a>),
    Spacing(Spacing<'a>),
    SoftBreak(SoftBreak<'a>),
    Code(Code<'a>),
}

impl<'a> TypedNode<'a> for Inline<'a> {
    fn from_node(node: &'a Node) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Quote => Quote::from_node(node).map(Self::Quote),
            SyntaxKind::Strikeout => Strikeout::from_node(node).map(Self::Strikeout),
            SyntaxKind::Emphasis => Emphasis::from_node(node).map(Self::Emphasis),
            SyntaxKind::Strong => Strong::from_node(node).map(Self::Strong),
            SyntaxKind::Subscript => Subscript::from_node(node).map(Self::Subscript),
            SyntaxKind::Supscript => Supscript::from_node(node).map(Self::Supscript),
            SyntaxKind::Link => Link::from_node(node).map(Self::Link),
            SyntaxKind::Ref => Ref::from_node(node).map(Self::Ref),
            SyntaxKind::RawInline => RawInline::from_node(node).map(Self::RawInline),
            SyntaxKind::MathInline => MathInline::from_node(node).map(Self::MathInline),
            SyntaxKind::Comment => Comment::from_node(node).map(Self::Comment),
            SyntaxKind::Escape => Escape::from_node(node).map(Self::Escape),
            SyntaxKind::Word => Word::from_node(node).map(Self::Word),
            SyntaxKind::Spacing => Spacing::from_node(node).map(Self::Spacing),
            SyntaxKind::SoftBreak => SoftBreak::from_node(node).map(Self::SoftBreak),
            SyntaxKind::Code => Code::from_node(node).map(Self::Code),
            _ => None,
        }
    }

    fn to_node(self) -> &'a Node {
        match self {
            Self::Quote(q) => q.to_node(),
            Self::Strikeout(s) => s.to_node(),
            Self::Emphasis(e) => e.to_node(),
            Self::Strong(s) => s.to_node(),
            Self::Subscript(s) => s.to_node(),
            Self::Supscript(s) => s.to_node(),
            Self::Link(l) => l.to_node(),
            Self::Ref(r) => r.to_node(),
            Self::RawInline(r) => r.to_node(),
            Self::MathInline(m) => m.to_node(),
            Self::Comment(c) => c.to_node(),
            Self::Escape(e) => e.to_node(),
            Self::Word(w) => w.to_node(),
            Self::Spacing(s) => s.to_node(),
            Self::SoftBreak(s) => s.to_node(),
            Self::Code(c) => c.to_node(),
        }
    }
}

node! {
    Quote
}

impl<'a> IntoIterator for Quote<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Strikeout
}

impl<'a> IntoIterator for Strikeout<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Emphasis
}

impl<'a> IntoIterator for Emphasis<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Strong
}

impl<'a> IntoIterator for Strong<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Subscript
}

impl<'a> IntoIterator for Subscript<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Supscript
}

impl<'a> IntoIterator for Supscript<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Inline<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Inline::from_node)
    }
}

node! {
    Link
}

impl<'a> Link<'a> {
    pub fn href(self) -> Text<'a> {
        self.0
            .children()
            .find_map(Text::from_node)
            .expect("link expects href")
    }

    pub fn content(self) -> Option<Content<'a>> {
        self.0.children().find_map(Content::from_node)
    }
}

node! {
    Ref
}

impl<'a> Ref<'a> {
    pub fn target(self) -> &'a EcoString {
        self.0
            .children()
            .find_map(Ident::from_node)
            .expect("ref expects target")
            .get()
    }
}

node! {
    RawInline
}

impl<'a> RawInline<'a> {
    pub fn get(self) -> &'a EcoString {
        // self.0
        //     .children()
        //     .find_map(Text::from_node)
        //     .expect("raw inline expects text")
        //     .get()
        self.0.text()
    }
}

node! {
    MathInline
}

impl<'a> MathInline<'a> {
    pub fn get(self) -> &'a EcoString {
        // self.0
        //     .children()
        //     .find_map(Text::from_node)
        //     .expect("math inline expects text")
        //     .get()
        self.0.text()
    }
}

node! {
    Comment
}

impl<'a> Comment<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0
            .children()
            .find_map(Text::from_node)
            .expect("math inline expects text")
            .get()
    }
}

node! {
    Escape
}

impl<'a> Escape<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0
            .children()
            .find_map(Word::from_node)
            .expect("escape expects word")
            .get()
    }
}

node! {
    Text
}

impl<'a> Text<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

node! {
    Word
}

impl<'a> Word<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

node! {
    Spacing
}

node! {
    SoftBreak
}

node! {
    Code
}

impl<'a> Code<'a> {
    pub fn get(self) -> Expr<'a> {
        self.0
            .children()
            .find_map(Expr::from_node)
            .expect("code expects expr")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    Ident(Ident<'a>),
    Call(Call<'a>),
    Literal(Literal<'a>),
    Block(ExprBlock<'a>),
    Content(Content<'a>),
}

impl<'a> TypedNode<'a> for Expr<'a> {
    fn from_node(node: &'a Node) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Ident => Ident::from_node(node).map(Self::Ident),
            SyntaxKind::Call => Call::from_node(node).map(Self::Call),
            SyntaxKind::Bool | SyntaxKind::Float | SyntaxKind::Int | SyntaxKind::Str => {
                Literal::from_node(node).map(Self::Literal)
            }
            SyntaxKind::Content => Content::from_node(node).map(Self::Content),
            _ => None,
        }
    }

    fn to_node(self) -> &'a Node {
        match self {
            Self::Ident(i) => i.to_node(),
            Self::Call(c) => c.to_node(),
            Self::Literal(l) => l.to_node(),
            Self::Block(b) => b.to_node(),
            Self::Content(c) => c.to_node(),
        }
    }
}

node! {
    ExprBlock
}

impl<'a> IntoIterator for ExprBlock<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Expr<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Expr::from_node)
    }
}

node! {
    Ident
}

impl<'a> Ident<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

node! {
    Call
}

impl<'a> Call<'a> {
    pub fn ident(self) -> CallIdent<'a> {
        self.0
            .children()
            .find_map(CallIdent::from_node)
            .expect("call expects ident")
    }

    pub fn args(self) -> Args<'a> {
        self.0
            .children()
            .find_map(Args::from_node)
            .expect("call expects args")
    }
}

node! {
    CallIdent
}

impl<'a> CallIdent<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

node! {
    Args
}

impl<'a> IntoIterator for Args<'a> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Arg<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.children().filter_map(Arg::from_node)
    }
}

impl<'a> Args<'a> {
    pub fn content(self) -> Option<Content<'a>> {
        self.0.children().find_map(Content::from_node)
    }
}

node! {
    Arg
}

impl<'a> Arg<'a> {
    pub fn ident(self) -> Option<ArgIdent<'a>> {
        self.0.children().find_map(ArgIdent::from_node)
    }

    pub fn value(self) -> Expr<'a> {
        self.0
            .children()
            .find_map(Expr::from_node)
            .expect("arg expects value")
    }
}

node! {
    ArgIdent
}

impl<'a> ArgIdent<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Literal<'a> {
    Str(Str<'a>),
    Int(Int<'a>),
    Float(Float<'a>),
    Bool(Bool<'a>),
}

impl<'a> TypedNode<'a> for Literal<'a> {
    fn from_node(node: &'a Node) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Str => Str::from_node(node).map(Self::Str),
            SyntaxKind::Int => Int::from_node(node).map(Self::Int),
            SyntaxKind::Float => Float::from_node(node).map(Self::Float),
            SyntaxKind::Bool => Bool::from_node(node).map(Self::Bool),
            _ => None,
        }
    }

    fn to_node(self) -> &'a Node {
        match self {
            Self::Str(s) => s.to_node(),
            Self::Int(i) => i.to_node(),
            Self::Float(f) => f.to_node(),
            Self::Bool(b) => b.to_node(),
        }
    }
}

node! {
    Str
}

impl<'a> Str<'a> {
    pub fn get(self) -> &'a EcoString {
        self.0.text()
    }
}

node! {
    Int
}

impl<'a> Int<'a> {
    pub fn get(self) -> i64 {
        self.0.text().as_str().parse().unwrap()
    }
}

node! {
    Float
}

impl<'a> Float<'a> {
    pub fn get(self) -> f64 {
        self.0.text().as_str().parse().unwrap()
    }
}

node! {
    Bool
}

impl<'a> Bool<'a> {
    pub fn get(self) -> bool {
        self.0.text().as_str().parse().unwrap()
    }
}
