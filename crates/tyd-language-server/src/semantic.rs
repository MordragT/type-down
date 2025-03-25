use ropey::Rope;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType};
use tyd_core::prelude::*;
use tyd_syntax::{Span, Spanned, Spans};

pub const LEGEND: &[SemanticTokenType] = &[
    SemanticTokenType::VARIABLE,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::COMMENT,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::DECORATOR,
    SemanticTokenType::MODIFIER,
    SemanticTokenType::OPERATOR,
];

#[repr(u32)]
pub enum SemanticTokenKind {
    Variable = 0,
    Function = 1,
    Parameter = 2,
    Keyword = 3,
    Comment = 4,
    String = 5,
    Number = 6,
    Decorator = 7,
    Modifier = 8,
    Operator = 9,
}

impl SemanticTokenKind {
    pub fn from_node(node: &Node, span: Span) -> Option<Spanned<Self>> {
        let semantic = match node {
            Node::Ident(_) => SemanticTokenKind::Variable,
            // CallIdent => SemanticTokenKind::Function,
            // ArgIdent | RawLang => SemanticTokenKind::Parameter,
            Node::Comment(_) => SemanticTokenKind::Comment,
            Node::Literal(literal) => match literal {
                tree::Literal::Str(_) => SemanticTokenKind::String,
                tree::Literal::Int(_) | tree::Literal::Float(_) => SemanticTokenKind::Number,
                tree::Literal::Bool(_) => SemanticTokenKind::Keyword,
            },
            Node::Label(_) | Node::Ref(_) => SemanticTokenKind::Decorator,
            Node::Escape(_) => SemanticTokenKind::Modifier,
            Node::HeadingMarker(_) => SemanticTokenKind::Operator,
            // HeadingMarker | ListMarker | EnumMarker | TermMarker | SubscriptMarker
            // | SupscriptMarker | CodeMarker => SemanticTokenKind::Operator,
            _ => return None,
        };

        Some((semantic, span))
    }
}

pub struct SemanticAnalyzer {
    rope: Rope,
    doc: Doc,
    spans: Spans,
}

impl SemanticAnalyzer {
    pub fn new(rope: Rope, doc: Doc, spans: Spans) -> Self {
        Self { rope, doc, spans }
    }

    pub fn tokens(&self) -> Vec<SemanticToken> {
        let mut last_line = 0;
        let mut last_start = 0;

        let mut tokens = self
            .doc
            .iter_full()
            .filter_map(|(node, id)| {
                SemanticTokenKind::from_node(node, self.spans.get(id).inner_copied())
            })
            .collect::<Vec<_>>();

        tokens.sort_by_key(|(_, span)| span.start);

        tokens
            .into_iter()
            .filter_map(|(kind, span)| {
                let line = self.rope.try_byte_to_line(span.start).ok()? as u32;
                let first = self.rope.try_line_to_char(line as usize).ok()? as u32;
                let start =
                    (self.rope.try_byte_to_char(span.start).ok()? as u32).checked_sub(first)?;

                let delta_line = line.checked_sub(last_line)?;
                let delta_start = if delta_line == 0 {
                    start.checked_sub(last_start)?
                } else {
                    start
                };
                let token = Some(SemanticToken {
                    delta_line,
                    delta_start,
                    length: (span.end - span.start) as u32,
                    token_type: kind as u32,
                    token_modifiers_bitset: 0,
                });
                last_line = line;
                last_start = start;
                token
            })
            .collect()
    }
}
