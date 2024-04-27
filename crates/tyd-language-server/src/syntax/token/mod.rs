use std::fmt;
use tyd_syntax::{ast, Span};

mod kind;

pub use kind::TokenKind;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
    pub kind: TokenKind,
    pub span: Span,
}

impl SyntaxToken {
    pub fn literal(literal: &ast::Literal, span: Span) -> Self {
        use ast::Literal::*;

        let kind = match literal {
            Str(_) => TokenKind::Str,
            Boolean(_) => TokenKind::Bool,
            Int(_) => TokenKind::Int,
        };

        Self { kind, span }
    }

    pub fn call_ident(ident: &ast::Ident) -> Self {
        Self {
            kind: TokenKind::CallIdent,
            span: ident.span,
        }
    }

    pub fn arg_ident(ident: &ast::Ident) -> Self {
        Self {
            kind: TokenKind::ArgIdent,
            span: ident.span,
        }
    }
}

impl fmt::Debug for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl From<&ast::Label> for SyntaxToken {
    fn from(value: &ast::Label) -> Self {
        SyntaxToken {
            kind: TokenKind::Label,
            span: value.span,
        }
    }
}

impl From<&ast::RawContent> for SyntaxToken {
    fn from(value: &ast::RawContent) -> Self {
        SyntaxToken {
            kind: TokenKind::RawContent,
            span: value.span,
        }
    }
}

impl From<&ast::RawLang> for SyntaxToken {
    fn from(value: &ast::RawLang) -> Self {
        SyntaxToken {
            kind: TokenKind::RawLang,
            span: value.span,
        }
    }
}

impl From<&ast::HeadingLevel> for SyntaxToken {
    fn from(value: &ast::HeadingLevel) -> Self {
        SyntaxToken {
            kind: TokenKind::HeadingMarker(value.level),
            span: value.span,
        }
    }
}

impl From<&ast::Href> for SyntaxToken {
    fn from(value: &ast::Href) -> Self {
        SyntaxToken {
            kind: TokenKind::Href,
            span: value.span,
        }
    }
}

impl From<&ast::Cite> for SyntaxToken {
    fn from(value: &ast::Cite) -> Self {
        SyntaxToken {
            kind: TokenKind::Cite,
            span: value.span,
        }
    }
}

impl From<&ast::RawInline> for SyntaxToken {
    fn from(value: &ast::RawInline) -> Self {
        SyntaxToken {
            kind: TokenKind::RawInline,
            span: value.span,
        }
    }
}

impl From<&ast::MathInline> for SyntaxToken {
    fn from(value: &ast::MathInline) -> Self {
        SyntaxToken {
            kind: TokenKind::MathInline,
            span: value.span,
        }
    }
}

impl From<&ast::Comment> for SyntaxToken {
    fn from(value: &ast::Comment) -> Self {
        SyntaxToken {
            kind: TokenKind::Comment,
            span: value.span,
        }
    }
}

impl From<&ast::Escape> for SyntaxToken {
    fn from(value: &ast::Escape) -> Self {
        SyntaxToken {
            kind: TokenKind::Escape,
            span: value.span,
        }
    }
}

impl From<&ast::Word> for SyntaxToken {
    fn from(value: &ast::Word) -> Self {
        SyntaxToken {
            kind: TokenKind::Word,
            span: value.span,
        }
    }
}

impl From<&ast::Spacing> for SyntaxToken {
    fn from(value: &ast::Spacing) -> Self {
        SyntaxToken {
            kind: TokenKind::Spacing,
            span: value.span,
        }
    }
}

impl From<&ast::SoftBreak> for SyntaxToken {
    fn from(value: &ast::SoftBreak) -> Self {
        SyntaxToken {
            kind: TokenKind::SoftBreak,
            span: value.span,
        }
    }
}

impl From<&ast::Ident> for SyntaxToken {
    fn from(value: &ast::Ident) -> Self {
        SyntaxToken {
            kind: TokenKind::Ident,
            span: value.span,
        }
    }
}
