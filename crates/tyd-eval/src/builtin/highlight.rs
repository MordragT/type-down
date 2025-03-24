use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents a function that highlights text by applying a "mark" class to it.
///
/// When used in a document, this function wraps the provided content in a span
/// with the CSS class "mark", which typically renders as highlighted text.
#[derive(Debug, Clone, Copy)]
pub struct Highlight;

impl Into<Value> for Highlight {
    /// Converts this highlighter into a Value containing the highlight function.
    fn into(self) -> Value {
        Value::Func(highlight)
    }
}

/// Highlights the given content by wrapping it in a span with the "mark" class.
///
/// # Arguments
///
/// * `stack` - Stack containing positional arguments, where the first position should be the content to highlight
/// * `scope` - Scope containing named arguments (none are used by this function)
/// * `_source` - Source information (unused)
/// * `span` - The span in the source document for error reporting
/// * `tracer` - Error tracer for reporting type or argument errors
///
/// # Returns
///
/// * `Value::Inline` - An inline element wrapping the content with highlighting
/// * `Value::None` - If required arguments are missing or of incorrect type
pub fn highlight(
    mut stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut type_checker = TypeChecker::new(tracer, span);

    // Extract the content to highlight from the first positional argument
    let content = match type_checker.pop_from_stack::<ir::Content>(&mut stack, 0) {
        Some(content) => content,
        None => return Value::None,
    };

    // Warn about any unused arguments
    type_checker.warn_unknown_positional(stack, 1);
    type_checker.warn_unknown_named(scope);

    // Create an inline span with the "mark" class containing the content
    let inline = ir::Inline::Span(ir::AttrBuilder::new().class("mark").build(), content);

    Value::Inline(inline)
}
