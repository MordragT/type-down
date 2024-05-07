use ropey::Rope;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType};
use tyd_syntax::{kind::SyntaxKind, node::Node};

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
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;

        let semantic = match kind {
            Ident => SemanticTokenKind::Variable,
            CallIdent => SemanticTokenKind::Function,
            ArgIdent | RawLang => SemanticTokenKind::Parameter,
            Bool => SemanticTokenKind::Keyword,
            Comment => SemanticTokenKind::Comment,
            Str => SemanticTokenKind::String,
            Int | Float => SemanticTokenKind::Number,
            Label | Ref => SemanticTokenKind::Decorator,
            Escape => SemanticTokenKind::Modifier,
            HeadingMarker | ListMarker | EnumMarker | TermMarker | SubscriptMarker
            | SupscriptMarker | CodeMarker => SemanticTokenKind::Operator,
            _ => return None,
        };

        Some(semantic)
    }
}

pub fn semantic_tokens_full_from_node(node: &Node, rope: &Rope) -> Vec<SemanticToken> {
    let mut last_line = 0;
    let mut last_start = 0;

    let mut tokens = node.filter_map(SemanticTokenKind::from_syntax);
    tokens.sort_by_key(|(_, span)| span.start);

    tokens
        .into_iter()
        .filter_map(|(kind, span)| {
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
                token_type: kind as u32,
                token_modifiers_bitset: 0,
            });
            last_line = line;
            last_start = start;
            token
        })
        .collect()
}

pub fn semantic_tokens_range_from_node(node: &Node, rope: &Rope) -> Vec<SemanticToken> {
    let mut pre_line = 0;
    let mut pre_start = 0;

    let mut tokens = node.filter_map(SemanticTokenKind::from_syntax);
    tokens.sort_by_key(|(_, span)| span.start);

    tokens
        .into_iter()
        .filter_map(|(kind, span)| {
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
                token_type: kind as u32,
                token_modifiers_bitset: 0,
            });
            pre_line = line;
            pre_start = start;
            token
        })
        .collect()
}
