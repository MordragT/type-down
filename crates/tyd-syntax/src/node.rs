use ecow::EcoString;

use crate::{kind::SyntaxKind, Span};

static EMPTY: EcoString = EcoString::new();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeafNode {
    pub span: Span,
    pub kind: SyntaxKind,
    pub text: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BranchNode {
    pub span: Span,
    pub kind: SyntaxKind,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorNode {
    pub span: Span,
    pub text: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Branch(BranchNode),
    Leaf(LeafNode),
    Error(ErrorNode),
}

impl Node {
    pub fn flatten(&self) -> Vec<(SyntaxKind, Span)> {
        let mut collector = Vec::new();
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            match node {
                Self::Branch(node) => {
                    collector.push((node.kind, node.span));
                    stack.extend(&node.children);
                }
                Self::Error(e) => collector.push((SyntaxKind::Error, e.span)),
                Self::Leaf(leaf) => collector.push((leaf.kind, leaf.span)),
            }
        }

        collector
    }

    pub fn filter_map<T>(&self, f: impl Fn(SyntaxKind) -> Option<T>) -> Vec<(T, Span)> {
        let mut collector = Vec::new();
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            if let Some(val) = f(node.kind()) {
                collector.push((val, node.span()));
            } else if let Self::Branch(branch) = node {
                stack.extend(&branch.children)
            }
        }

        collector
    }

    pub fn kind(&self) -> SyntaxKind {
        match self {
            Self::Branch(b) => b.kind,
            Self::Leaf(l) => l.kind,
            Self::Error(_) => SyntaxKind::Error,
        }
    }

    pub fn children(&self) -> std::slice::Iter<Node> {
        match self {
            Self::Branch(tree) => tree.children.iter(),
            _ => [].iter(),
        }
    }

    pub fn text(&self) -> &EcoString {
        match self {
            Self::Leaf(token) => &token.text,
            Self::Error(error) => &error.text,
            _ => &EMPTY,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Branch(tree) => tree.span,
            Self::Leaf(token) => token.span,
            Self::Error(e) => e.span,
        }
    }

    pub fn is_branch(&self) -> bool {
        match self {
            Self::Branch(_) => true,
            _ => false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf(_) => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false,
        }
    }
}

impl From<BranchNode> for Node {
    fn from(value: BranchNode) -> Self {
        Self::Branch(value)
    }
}

impl From<LeafNode> for Node {
    fn from(value: LeafNode) -> Self {
        Self::Leaf(value)
    }
}

impl From<ErrorNode> for Node {
    fn from(value: ErrorNode) -> Self {
        Self::Error(value)
    }
}
