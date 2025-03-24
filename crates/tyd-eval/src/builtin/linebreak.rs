use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents a line break in the document.
///
/// This struct is used to create line breaks within the output document.
/// When converted to a Value, it becomes a function that produces a line break.
#[derive(Debug, Clone, Copy)]
pub struct LineBreak;

impl Into<Value> for LineBreak {
    /// Converts the LineBreak into a Value containing the linebreak function.
    ///
    /// # Returns
    ///
    /// A Value of the Func variant containing the linebreak function.
    fn into(self) -> Value {
        Value::Func(linebreak)
    }
}

/// Creates a line break in the document.
///
/// This function takes no arguments. Any provided positional or named arguments
/// will be reported as warnings but otherwise ignored.
///
/// # Arguments
///
/// * `stack` - Stack of positional arguments (expected to be empty)
/// * `scope` - Scope of named arguments (expected to be empty)
/// * `_source` - Source reference (unused)
/// * `span` - Span in the source code where this function was called
/// * `tracer` - Tracer for error reporting
///
/// # Returns
///
/// A Value representing a line break in the intermediate representation.
pub fn linebreak(
    stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut checker = TypeChecker::new(tracer, span);

    checker.warn_unknown_positional(stack, 0);
    checker.warn_unknown_named(scope);

    Value::Inline(ir::Inline::LineBreak)
}
