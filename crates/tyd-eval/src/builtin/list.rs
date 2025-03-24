use tyd_syntax::{source::Source, Span};

use crate::{
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents the 'list' function which creates a list from stack arguments.
///
/// This struct is used as a type to represent the list creation function.
#[derive(Debug, Clone, Copy)]
pub struct List;

impl Into<Value> for List {
    /// Converts this List instance into a Value containing the list function.
    ///
    /// # Returns
    ///
    /// A Value::Func containing the list function implementation.
    fn into(self) -> Value {
        Value::Func(list)
    }
}

/// Creates a list containing all positional arguments.
///
/// This function converts all positional arguments into a list value.
/// Any named arguments will generate warnings as they are ignored.
///
/// # Arguments
///
/// * `stack` - Stack of positional arguments that will become list elements
/// * `scope` - Scope containing named arguments (all will be warned as unknown)
/// * `_source` - Source information (unused)
/// * `span` - Source span for error reporting
/// * `tracer` - Error tracer for reporting warnings
///
/// # Returns
///
/// A Value::List containing all the positional arguments
pub fn list(stack: Stack, scope: Scope, _source: Source, span: Span, tracer: &mut Tracer) -> Value {
    let mut checker = TypeChecker::new(tracer, span);
    checker.warn_unknown_named(scope);
    Value::List(stack.into_inner())
}
