use ropey::Rope;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType};

use crate::syntax::{SyntaxKind, SyntaxNode};

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
}

impl SemanticTokenKind {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;

        let semantic = match kind {
            Call => SemanticTokenKind::Function,
            Arg | RawLang => SemanticTokenKind::Parameter,
            Comment => SemanticTokenKind::Comment,
            Label | Cite => SemanticTokenKind::Decorator,
            Ident => SemanticTokenKind::Variable,
            Escape => SemanticTokenKind::Modifier,
            Literal => SemanticTokenKind::Keyword,
            _ => return None,
        };

        Some(semantic)
    }
}

pub fn semantic_tokens_full_from_node(node: SyntaxNode, rope: &Rope) -> Vec<SemanticToken> {
    let mut last_line = 0;
    let mut last_start = 0;

    let mut nodes = node.flatten();
    nodes.sort_by(|(_, a), (_, b)| a.start.cmp(&b.start));

    let tokens = nodes
        .into_iter()
        .filter_map(|(kind, span)| {
            if let Some(token_kind) = SemanticTokenKind::from_syntax(kind) {
                let line = rope.try_byte_to_line(span.start).ok()? as u32;
                let first = rope.try_line_to_char(line as usize).ok()? as u32;
                let start = (rope.try_byte_to_char(span.start).ok()? as u32).checked_sub(first)?;

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
                    token_type: token_kind as u32,
                    token_modifiers_bitset: 0,
                });
                last_line = line;
                last_start = start;
                token
            } else {
                None
            }
        })
        .collect();

    tokens
}

pub fn semantic_tokens_range_from_node(node: SyntaxNode, rope: &Rope) -> Vec<SemanticToken> {
    let mut pre_line = 0;
    let mut pre_start = 0;

    let tokens = node
        .flatten()
        .into_iter()
        .filter_map(|(kind, span)| {
            if let Some(token_kind) = SemanticTokenKind::from_syntax(kind) {
                let line = rope.try_byte_to_line(span.start).ok()? as u32;
                let first = rope.try_line_to_char(line as usize).ok()? as u32;
                let start = rope.try_byte_to_char(span.start).ok()? as u32 - first;
                let token = Some(SemanticToken {
                    delta_line: line - pre_line,
                    delta_start: if start >= pre_start {
                        start - pre_start
                    } else {
                        start
                    },
                    length: (span.end - span.start) as u32,
                    token_type: token_kind as u32,
                    token_modifiers_bitset: 0,
                });
                pre_line = line;
                pre_start = start;
                token
            } else {
                None
            }
        })
        .collect();

    tokens
}
