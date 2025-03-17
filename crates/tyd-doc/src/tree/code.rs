use derive_more::From;
use ecow::EcoString;

use super::inline::Inline;
use crate::id::NodeId;

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Code(pub NodeId<Expr>);

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    Let(NodeId<Let>),
    If(NodeId<If>),
    For(NodeId<For>),
    Call(NodeId<Call>),
    Literal(NodeId<Literal>),
    Ident(NodeId<Ident>),
    Content(NodeId<Content>),
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Let(pub Vec<NodeId<Bind>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bind {
    pub name: NodeId<Ident>,
    pub value: NodeId<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct If {
    pub predicate: NodeId<Expr>,
    pub then: NodeId<Content>,
    pub or: NodeId<Content>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct For {
    pub el: NodeId<Ident>,
    pub inside: NodeId<Expr>,
    pub content: NodeId<Content>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Call {
    pub ident: NodeId<Ident>,
    pub args: NodeId<Args>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Args {
    pub args: Vec<NodeId<Arg>>,
    pub content: Option<NodeId<Content>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Arg {
    pub name: Option<NodeId<Ident>>,
    pub value: NodeId<Expr>,
}

#[derive(Debug, From, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Str(EcoString),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Ident(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Content(pub Vec<NodeId<Inline>>);
