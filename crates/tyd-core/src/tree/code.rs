use derive_more::From;
use ecow::EcoString;

use super::inline::Inline;
use crate::id::NodeId;

/// Represents a block of code expression in the AST.
#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Code(pub NodeId<Expr>);

/// Represents various expression types that can appear in code.
#[derive(Debug, From, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    /// A let binding expression, e.g., `let x = 1`.
    Let(NodeId<Let>),
    /// A conditional expression, e.g., `if condition [ ... ] else [ ... ]`.
    If(NodeId<If>),
    /// A for-loop expression, e.g., `for item in collection [ ... ]`.
    For(NodeId<For>),
    /// A function or method call expression.
    Call(NodeId<Call>),
    /// A literal value (string, integer, float, boolean).
    Literal(NodeId<Literal>),
    /// An identifier reference.
    Ident(NodeId<Ident>),
    /// Inline content within code.
    Content(NodeId<Content>),
}

/// Represents a let binding that assigns values to identifiers.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Let(pub Vec<NodeId<Bind>>);

/// Represents a single binding in a let expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bind {
    /// The identifier being bound.
    pub name: NodeId<Ident>,
    /// The expression whose value is bound to the identifier.
    pub value: NodeId<Expr>,
}

/// Represents a conditional (if-else) expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct If {
    /// The condition expression to evaluate.
    pub predicate: NodeId<Expr>,
    /// The content to execute if the condition is true.
    pub then: NodeId<Content>,
    /// The content to execute if the condition is false.
    pub or: NodeId<Content>,
}

/// Represents a for-loop expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct For {
    /// The loop variable identifier.
    pub el: NodeId<Ident>,
    /// The collection being iterated over.
    pub inside: NodeId<Expr>,
    /// The loop body.
    pub content: NodeId<Content>,
}

/// Represents a function or method call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Call {
    /// The function identifier.
    pub ident: NodeId<Ident>,
    /// The arguments passed to the function.
    pub args: NodeId<Args>,
}

/// Represents the arguments in a function call.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Args {
    /// The list of individual arguments.
    pub args: Vec<NodeId<Arg>>,
    /// Optional content passed as a block to the function.
    pub content: Option<NodeId<Content>>,
}

/// Represents a single argument in a function call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Arg {
    /// Optional name for named arguments.
    pub name: Option<NodeId<Ident>>,
    /// The value of the argument.
    pub value: NodeId<Expr>,
}

/// Represents literal values in the code.
#[derive(Debug, From, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    /// A string literal.
    Str(EcoString),
    /// An integer literal.
    Int(i64),
    /// A floating-point literal.
    Float(f64),
    /// A boolean literal.
    Bool(bool),
}

/// Represents an identifier (variable name, function name, etc.).
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Ident(pub EcoString);

/// Represents a block of content consisting of inline elements.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Content(pub Vec<NodeId<Inline>>);
