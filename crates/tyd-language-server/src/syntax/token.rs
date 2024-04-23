use std::fmt;

use tyd_syntax::{ast, Span};

use super::SyntaxKind;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
    pub kind: SyntaxKind,
    pub span: Span,
}

impl fmt::Debug for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl From<&ast::Label> for SyntaxToken {
    fn from(value: &ast::Label) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Label,
            span: value.span,
        }
    }
}

impl From<&ast::RawContent> for SyntaxToken {
    fn from(value: &ast::RawContent) -> Self {
        SyntaxToken {
            kind: SyntaxKind::RawContent,
            span: value.span,
        }
    }
}

impl From<&ast::RawLang> for SyntaxToken {
    fn from(value: &ast::RawLang) -> Self {
        SyntaxToken {
            kind: SyntaxKind::RawLang,
            span: value.span,
        }
    }
}

impl From<&ast::HeadingLevel> for SyntaxToken {
    fn from(value: &ast::HeadingLevel) -> Self {
        SyntaxToken {
            kind: SyntaxKind::HeadingMarker(value.level),
            span: value.span,
        }
    }
}

impl From<&ast::Href> for SyntaxToken {
    fn from(value: &ast::Href) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Href,
            span: value.span,
        }
    }
}

impl From<&ast::Cite> for SyntaxToken {
    fn from(value: &ast::Cite) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Cite,
            span: value.span,
        }
    }
}

impl From<&ast::RawInline> for SyntaxToken {
    fn from(value: &ast::RawInline) -> Self {
        SyntaxToken {
            kind: SyntaxKind::RawInline,
            span: value.span,
        }
    }
}

impl From<&ast::MathInline> for SyntaxToken {
    fn from(value: &ast::MathInline) -> Self {
        SyntaxToken {
            kind: SyntaxKind::MathInline,
            span: value.span,
        }
    }
}

impl From<&ast::Comment> for SyntaxToken {
    fn from(value: &ast::Comment) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Comment,
            span: value.span,
        }
    }
}

impl From<&ast::Escape> for SyntaxToken {
    fn from(value: &ast::Escape) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Escape,
            span: value.span,
        }
    }
}

impl From<&ast::Word> for SyntaxToken {
    fn from(value: &ast::Word) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Word,
            span: value.span,
        }
    }
}

impl From<&ast::Spacing> for SyntaxToken {
    fn from(value: &ast::Spacing) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Spacing,
            span: value.span,
        }
    }
}

impl From<&ast::SoftBreak> for SyntaxToken {
    fn from(value: &ast::SoftBreak) -> Self {
        SyntaxToken {
            kind: SyntaxKind::SoftBreak,
            span: value.span,
        }
    }
}

impl From<&ast::Ident> for SyntaxToken {
    fn from(value: &ast::Ident) -> Self {
        SyntaxToken {
            kind: SyntaxKind::Ident,
            span: value.span,
        }
    }
}
