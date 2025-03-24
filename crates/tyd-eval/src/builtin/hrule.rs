use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents a horizontal rule element in a document.
///
/// This type implements `Into<Value>` to convert into a function value
/// that creates a horizontal rule block when called.
#[derive(Debug, Clone, Copy)]
pub struct HorizontalRule;

impl Into<Value> for HorizontalRule {
    /// Converts this horizontal rule into a function value.
    ///
    /// # Returns
    ///
    /// A `Value::Func` containing the `hrule` function.
    fn into(self) -> Value {
        Value::Func(hrule)
    }
}

/// Creates a horizontal rule block.
///
/// This function generates a horizontal rule element in the document.
/// It takes no arguments and will generate warnings for any arguments provided.
///
/// # Arguments
///
/// * `stack` - Positional arguments (none expected)
/// * `scope` - Named arguments (none expected)
/// * `_source` - Source information (unused)
/// * `span` - The span in the source file for error reporting
/// * `tracer` - For tracking and reporting any errors
///
/// # Returns
///
/// A `Value::Block` containing a horizontal rule block
pub fn hrule(
    stack: Stack,
    scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut checker = TypeChecker::new(tracer, span);

    checker.warn_unknown_positional(stack, 0);
    checker.warn_unknown_named(scope);

    let block = ir::Block::HorizontalRule;

    Value::Block(block)
}
