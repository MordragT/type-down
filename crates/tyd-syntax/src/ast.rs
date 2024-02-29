use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast<'src> {
    pub blocks: Vec<Block<'src>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'src> {
    Div(Div<'src>),
    Raw(Raw<'src>),
    Heading(Heading<'src>),
    Table(Table<'src>),
    List(List<'src>),
    Enum(Enum<'src>),
    Term(Term<'src>),
    Paragraph(Paragraph<'src>),
    Plain(Plain<'src>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div<'src> {
    pub content: Vec<Block<'src>>,
    pub class: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw<'src> {
    pub content: &'src str,
    pub lang: Option<&'src str>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading<'src> {
    pub content: Text<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'src> {
    pub rows: Vec<TableRow<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
    pub col_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow<'src> {
    pub cells: Vec<Block<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'src> {
    pub items: Vec<ListItem<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

impl<'src> From<ListItem<'src>> for List<'src> {
    fn from(value: ListItem<'src>) -> Self {
        let items = vec![value.clone()];
        let ListItem {
            content: _,
            label,
            span,
        } = value;

        Self { items, label, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'src> {
    pub content: Vec<Block<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum<'src> {
    pub items: Vec<EnumItem<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}
impl<'src> From<EnumItem<'src>> for Enum<'src> {
    fn from(value: EnumItem<'src>) -> Self {
        let items = vec![value.clone()];
        let EnumItem {
            content: _,
            label,
            span,
        } = value;

        Self { items, label, span }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem<'src> {
    pub content: Vec<Block<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term<'src> {
    pub content: Vec<TermItem<'src>>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermItem<'src> {
    pub term: Text<'src>,
    pub content: Text<'src>,
    pub label: Option<&'src str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Plain<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

impl<'src> From<Text<'src>> for Plain<'src> {
    fn from(value: Text<'src>) -> Self {
        let Text { content, span } = value;

        Self { content, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline<'src> {
    Quote(Quote<'src>),
    Strikeout(Strikeout<'src>),
    Emphasis(Emphasis<'src>),
    Strong(Strong<'src>),
    Subscript(Subscript<'src>),
    Supscript(Supscript<'src>),
    Link(Link<'src>),
    Cite(Cite<'src>),
    RawInline(RawInline<'src>),
    MathInline(MathInline<'src>),
    Comment(Comment<'src>),
    Escape(Escape<'src>),
    Word(Word<'src>),
    Spacing(Spacing<'src>),
    // not used in cst stage but in ast
    SoftBreak,
    Code(Code<'src>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strikeout<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emphasis<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strong<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Supscript<'src> {
    pub content: Vec<Inline<'src>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<'src> {
    pub href: &'src str,
    pub content: Option<Vec<Inline<'src>>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cite<'src> {
    pub ident: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escape<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawInline<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathInline<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'src> {
    pub content: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spacing<'src> {
    pub content: &'src str,
    pub span: Span,
}

// Code

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code<'src> {
    pub expr: Expr<'src>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'src> {
    Ident(&'src str),
    Call(Call<'src>),
    Literal(Literal<'src>),
    Block(Vec<Expr<'src>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'src> {
    pub ident: &'src str,
    pub args: Vec<Arg<'src>>,
    pub content: Option<Vec<Inline<'src>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg<'src> {
    pub name: Option<&'src str>,
    pub value: Expr<'src>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'src> {
    Boolean(bool),
    Int(i64),
    // Float(f64),
    Str(&'src str),
}
