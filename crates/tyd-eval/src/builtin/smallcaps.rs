use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents the small-caps formatting function.
///
/// When used, this transforms text to display in small capital letters.
#[derive(Debug, Clone, Copy)]
pub struct SmallCaps;

impl Into<Value> for SmallCaps {
    /// Converts the SmallCaps struct into a Value by wrapping the smallcaps function.
    ///
    /// # Returns
    ///
    /// * `Value` - A function value that can be executed to format text in small caps.
    fn into(self) -> Value {
        Value::Func(smallcaps)
    }
}

/// Applies small-caps formatting to the provided content.
///
/// This function takes content as its first positional argument and wraps it in
/// small-caps formatting. It ignores any additional arguments and warns about their presence.
///
/// # Arguments
///
/// * `stack` - The positional arguments stack, expected to contain content to format
/// * `scope` - Named arguments (all will be warned as unknown)
/// * `_source` - Source information (unused)
/// * `span` - The span in the source where this function is called
/// * `tracer` - Error and warning tracer
///
/// # Returns
///
/// * `Value::Inline` - An inline element with small-caps formatting applied to the content
/// * `Value::None` - If the required content argument is missing or of the wrong type
pub fn smallcaps(
    mut stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut checker = TypeChecker::new(tracer, span);

    let content = match checker.pop_from_stack::<ir::Content>(&mut stack, 0) {
        Some(content) => content,
        None => return Value::None,
    };

    checker.warn_unknown_positional(stack, 1);
    checker.warn_unknown_named(scope);

    let inline = ir::Inline::SmallCaps(content);

    Value::Inline(inline)
}
