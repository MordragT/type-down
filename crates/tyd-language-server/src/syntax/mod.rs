pub use kind::SyntaxKind;
pub use node::SyntaxNode;
pub use token::SyntaxToken;

mod kind;
mod node;
mod token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxElement {
    Node(SyntaxNode),
    Token(SyntaxToken),
}

impl From<SyntaxNode> for SyntaxElement {
    fn from(value: SyntaxNode) -> Self {
        Self::Node(value)
    }
}

impl From<SyntaxToken> for SyntaxElement {
    fn from(value: SyntaxToken) -> Self {
        Self::Token(value)
    }
}
