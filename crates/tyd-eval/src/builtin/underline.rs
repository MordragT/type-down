use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents the underline function that creates underlined text.
///
/// The underline function converts its content argument into underlined text.
#[derive(Debug, Clone, Copy)]
pub struct Underline;

impl Into<Value> for Underline {
    /// Converts an Underline instance into a Value containing the underline function.
    fn into(self) -> Value {
        Value::Func(underline)
    }
}

/// Creates underlined text from the provided content.
///
/// # Arguments
///
/// * `stack` - The stack of positional arguments, expected to contain content to be underlined
/// * `scope` - The scope containing named arguments (none are used by this function)
/// * `_source` - The source document (unused)
/// * `span` - The source span where this function is being called from
/// * `tracer` - Error tracer for reporting issues
///
/// # Returns
///
/// * `Value::Inline` containing the underlined content if successful
/// * `Value::None` if required arguments are missing or of incorrect types
///
/// # Example
///
/// ```tyd
/// @underline("text to underline")
/// ```
pub fn underline(
    mut stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut checker = TypeChecker::new(tracer, span);

    // Extract the content to underline from the first position
    let content = match checker.pop_from_stack::<ir::Content>(&mut stack, 0) {
        Some(content) => content,
        None => return Value::None,
    };

    // Warn about any unused arguments
    checker.warn_unknown_positional(stack, 1);
    checker.warn_unknown_named(scope);

    // Create an underlined inline element with the content
    let inline = ir::Inline::Underline(content);

    Value::Inline(inline)
}
