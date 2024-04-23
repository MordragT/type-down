use tyd_syntax::{ast, Span};

use super::{SyntaxElement, SyntaxKind, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub span: Span,
    pub children: Vec<SyntaxElement>,
}

impl SyntaxNode {
    pub fn flatten(self) -> Vec<(SyntaxKind, Span)> {
        let mut stack = vec![SyntaxElement::Node(self)];
        let mut collector = Vec::new();

        while let Some(el) = stack.pop() {
            match el {
                SyntaxElement::Node(SyntaxNode {
                    kind,
                    span,
                    mut children,
                }) => {
                    collector.push((kind, span));
                    stack.append(&mut children);
                }
                SyntaxElement::Token(SyntaxToken { kind, span }) => {
                    collector.push((kind, span));
                }
            }
        }

        collector
    }
}

impl From<&ast::Ast> for SyntaxNode {
    fn from(ast::Ast { blocks, span }: &ast::Ast) -> Self {
        let children = blocks
            .into_iter()
            .map(|block| SyntaxNode::from(block).into())
            .collect();

        SyntaxNode {
            kind: SyntaxKind::Document,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Block> for SyntaxNode {
    fn from(value: &ast::Block) -> Self {
        use ast::Block::*;

        match value {
            Raw(r) => r.into(),
            Heading(h) => h.into(),
            Table(t) => t.into(),
            List(l) => l.into(),
            Enum(e) => e.into(),
            Terms(t) => t.into(),
            Paragraph(p) => p.into(),
            Plain(p) => p.into(),
        }
    }
}

impl From<&ast::Block> for SyntaxElement {
    fn from(value: &ast::Block) -> Self {
        let node = SyntaxNode::from(value);
        node.into()
    }
}

impl From<&ast::Raw> for SyntaxNode {
    fn from(
        ast::Raw {
            content,
            lang,
            span,
        }: &ast::Raw,
    ) -> Self {
        let mut children = vec![SyntaxToken::from(content).into()];

        if let Some(lang) = lang {
            children.push(SyntaxToken::from(lang).into());
        }

        SyntaxNode {
            kind: SyntaxKind::Raw,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Heading> for SyntaxNode {
    fn from(
        ast::Heading {
            level,
            content,
            label,
            span,
        }: &ast::Heading,
    ) -> Self {
        let mut children = vec![SyntaxToken::from(level).into()];
        children.extend(content.into_iter().map(Into::into));

        if let Some(label) = label {
            children.push(SyntaxToken::from(label).into());
        }

        SyntaxNode {
            kind: SyntaxKind::Heading,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Table> for SyntaxNode {
    fn from(
        ast::Table {
            rows,
            label,
            span,
            col_count: _,
        }: &ast::Table,
    ) -> Self {
        let mut children = rows
            .into_iter()
            .map(Into::into)
            .collect::<Vec<SyntaxElement>>();

        if let Some(label) = label {
            children.push(SyntaxToken::from(label).into());
        }

        SyntaxNode {
            kind: SyntaxKind::Table,
            children,
            span: *span,
        }
    }
}

impl From<&ast::TableRow> for SyntaxNode {
    fn from(ast::TableRow { cells, span }: &ast::TableRow) -> Self {
        let children = cells.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::TableRow,
            children,
            span: *span,
        }
    }
}

impl From<&ast::TableRow> for SyntaxElement {
    fn from(value: &ast::TableRow) -> Self {
        let node = SyntaxNode::from(value);
        node.into()
    }
}

impl From<&ast::List> for SyntaxNode {
    fn from(ast::List { items, span }: &ast::List) -> Self {
        let children = items.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::List,
            children,
            span: *span,
        }
    }
}

impl From<&ast::ListItem> for SyntaxNode {
    fn from(ast::ListItem { content, span }: &ast::ListItem) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::ListItem,
            children,
            span: *span,
        }
    }
}

impl From<&ast::ListItem> for SyntaxElement {
    fn from(value: &ast::ListItem) -> Self {
        let node = SyntaxNode::from(value);
        node.into()
    }
}

impl From<&ast::Enum> for SyntaxNode {
    fn from(ast::Enum { items, span }: &ast::Enum) -> Self {
        let children = items.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Enum,
            children,
            span: *span,
        }
    }
}

impl From<&ast::EnumItem> for SyntaxNode {
    fn from(ast::EnumItem { content, span }: &ast::EnumItem) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::EnumItem,
            children,
            span: *span,
        }
    }
}

impl From<&ast::EnumItem> for SyntaxElement {
    fn from(value: &ast::EnumItem) -> Self {
        let node = SyntaxNode::from(value);
        node.into()
    }
}

impl From<&ast::Terms> for SyntaxNode {
    fn from(ast::Terms { content, span }: &ast::Terms) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Terms,
            children,
            span: *span,
        }
    }
}

impl From<&ast::TermItem> for SyntaxNode {
    fn from(
        ast::TermItem {
            term,
            content,
            span,
        }: &ast::TermItem,
    ) -> Self {
        let children = term
            .into_iter()
            .chain(content.into_iter())
            .map(Into::into)
            .collect();

        SyntaxNode {
            kind: SyntaxKind::TermItem,
            children,
            span: *span,
        }
    }
}

impl From<&ast::TermItem> for SyntaxElement {
    fn from(value: &ast::TermItem) -> Self {
        let node = SyntaxNode::from(value);
        node.into()
    }
}

impl From<&ast::Paragraph> for SyntaxNode {
    fn from(ast::Paragraph { content, span }: &ast::Paragraph) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Paragraph,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Plain> for SyntaxNode {
    fn from(ast::Plain { content, span }: &ast::Plain) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Plain,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Inline> for SyntaxElement {
    fn from(value: &ast::Inline) -> Self {
        use ast::Inline::*;

        match value {
            Quote(el) => SyntaxNode::from(el).into(),
            Strikeout(el) => SyntaxNode::from(el).into(),
            Emphasis(el) => SyntaxNode::from(el).into(),
            Strong(el) => SyntaxNode::from(el).into(),
            Subscript(el) => SyntaxNode::from(el).into(),
            Supscript(el) => SyntaxNode::from(el).into(),
            Link(el) => SyntaxNode::from(el).into(),
            Cite(el) => SyntaxToken::from(el).into(),
            RawInline(el) => SyntaxToken::from(el).into(),
            MathInline(el) => SyntaxToken::from(el).into(),
            Comment(el) => SyntaxToken::from(el).into(),
            Escape(el) => SyntaxToken::from(el).into(),
            Word(el) => SyntaxToken::from(el).into(),
            Spacing(el) => SyntaxToken::from(el).into(),
            SoftBreak(el) => SyntaxToken::from(el).into(),
            Code(el) => SyntaxNode::from(el).into(),
        }
    }
}

impl From<&ast::Quote> for SyntaxNode {
    fn from(ast::Quote { content, span }: &ast::Quote) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Quote,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Strikeout> for SyntaxNode {
    fn from(ast::Strikeout { content, span }: &ast::Strikeout) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Strikeout,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Emphasis> for SyntaxNode {
    fn from(ast::Emphasis { content, span }: &ast::Emphasis) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Emphasis,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Strong> for SyntaxNode {
    fn from(ast::Strong { content, span }: &ast::Strong) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Strong,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Subscript> for SyntaxNode {
    fn from(ast::Subscript { content, span }: &ast::Subscript) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Subscript,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Supscript> for SyntaxNode {
    fn from(ast::Supscript { content, span }: &ast::Supscript) -> Self {
        let children = content.into_iter().map(Into::into).collect();

        SyntaxNode {
            kind: SyntaxKind::Supscript,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Link> for SyntaxNode {
    fn from(
        ast::Link {
            href,
            content,
            span,
        }: &ast::Link,
    ) -> Self {
        let mut children = vec![SyntaxToken::from(href).into()];

        if let Some(content) = content {
            children.extend(content.into_iter().map(Into::into));
        }

        SyntaxNode {
            kind: SyntaxKind::Link,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Code> for SyntaxNode {
    fn from(ast::Code { expr, span }: &ast::Code) -> Self {
        SyntaxNode {
            kind: SyntaxKind::Expr,
            children: vec![expr.into()],
            span: *span,
        }
    }
}

impl From<&ast::Expr> for SyntaxElement {
    fn from(value: &ast::Expr) -> Self {
        use ast::Expr::*;

        match value {
            Content(content) => SyntaxNode::from(content).into(),
            Ident(ident) => SyntaxToken::from(ident).into(),
            Call(call) => SyntaxNode::from(call).into(),
            Literal(_, span) => SyntaxToken {
                kind: SyntaxKind::Literal,
                span: *span,
            }
            .into(),
            Block(exprs, span) => SyntaxNode {
                kind: SyntaxKind::ExprBlock,
                children: exprs.into_iter().map(Into::into).collect(),
                span: *span,
            }
            .into(),
        }
    }
}

impl From<&ast::Content> for SyntaxNode {
    fn from(ast::Content { content, span }: &ast::Content) -> Self {
        SyntaxNode {
            kind: SyntaxKind::Content,
            children: content.into_iter().map(Into::into).collect(),
            span: *span,
        }
    }
}

impl From<&ast::Call> for SyntaxNode {
    fn from(ast::Call { ident, args, span }: &ast::Call) -> Self {
        SyntaxNode {
            kind: SyntaxKind::Call,
            children: vec![
                SyntaxToken::from(ident).into(),
                SyntaxNode::from(args).into(),
            ],
            span: *span,
        }
    }
}

impl From<&ast::Args> for SyntaxNode {
    fn from(
        ast::Args {
            args,
            content,
            span,
        }: &ast::Args,
    ) -> Self {
        let mut children: Vec<SyntaxElement> = args
            .into_iter()
            .map(|arg| SyntaxNode::from(arg).into())
            .collect();

        if let Some(content) = content {
            children.push(SyntaxNode::from(content).into());
        }

        SyntaxNode {
            kind: SyntaxKind::Args,
            children,
            span: *span,
        }
    }
}

impl From<&ast::Arg> for SyntaxNode {
    fn from(ast::Arg { name, value, span }: &ast::Arg) -> Self {
        let mut children = if let Some(name) = name {
            vec![SyntaxToken::from(name).into()]
        } else {
            Vec::new()
        };

        children.push(value.into());

        SyntaxNode {
            kind: SyntaxKind::Arg,
            children,
            span: *span,
        }
    }
}
