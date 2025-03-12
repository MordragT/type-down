use derive_more::From;
use ecow::EcoString;

use super::inline::Inline;
use crate::id::NodeId;

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Code(pub NodeId<Expr>);

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    Ident(NodeId<Ident>),
    Call(NodeId<Call>),
    Literal(NodeId<Literal>),
    Block(NodeId<ExprBlock>),
    Content(NodeId<Content>),
}

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Ident(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprBlock(pub Vec<NodeId<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub ident: NodeId<Ident>,
    pub args: NodeId<Args>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    pub args: Vec<NodeId<Arg>>,
    pub content: Option<NodeId<Content>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub key: Option<NodeId<Ident>>,
    pub value: NodeId<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(EcoString),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Content(pub Vec<NodeId<Inline>>);
