use super::Mode;

#[inline]
pub fn is_newline(c: char) -> bool {
    matches!(
        c,
        // Line Feed, Vertical Tab, Form Feed, Carriage Return.
        '\n' | '\x0B' | '\x0C' | '\r' |
        // Next Line, Line Separator, Paragraph Separator.
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

#[inline]
pub fn is_identy(c: char) -> bool {
    c.is_alphabetic() || c == '-' || c == '_'
}

#[inline]
pub fn is_wordy(c: Option<char>) -> bool {
    c.map_or(false, char::is_alphanumeric)
}

#[inline]
pub fn is_special(c: char) -> bool {
    matches!(
        c,
        // escape | raw | link | cite | strong | emph | strikeout | quote | supscript | subscript | content | attr/label
        '\\' | '`' | '<' | '@' | '*' | '/' | '~' | '"' | '^' | '_' | '[' | ']' | '{' | '}'
    ) || c.is_whitespace()
}

#[inline]
pub fn is_space(c: char, mode: Mode) -> bool {
    match mode {
        Mode::Markup => matches!(c, ' ' | '\t') || is_newline(c),
        _ => c.is_whitespace(),
    }
}
