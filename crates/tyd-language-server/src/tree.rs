use tyd_syntax::Span;

use crate::{kind::SyntaxKind, token::Token};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxElement {
    Node(SyntaxNode),
    Leaf(Token),
    // Error(SyntaxError),
}

impl From<SyntaxNode> for SyntaxElement {
    fn from(value: SyntaxNode) -> Self {
        Self::Node(value)
    }
}

impl From<Token> for SyntaxElement {
    fn from(value: Token) -> Self {
        Self::Leaf(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub children: Vec<SyntaxElement>,
    pub span: Span,
}
