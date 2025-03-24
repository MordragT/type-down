use tyd_syntax::{source::Source, Span};

use crate::{
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents a Map constructor function.
///
/// This struct implements the `Into<Value>` trait, allowing it to be
/// converted into a function value that creates maps from named arguments.
#[derive(Debug, Clone, Copy)]
pub struct Map;

impl Into<Value> for Map {
    /// Converts this Map constructor into a Value::Func
    fn into(self) -> Value {
        Value::Func(map)
    }
}

/// Creates a map value from named arguments.
///
/// This function takes all named arguments from the scope and converts them
/// into a map value. Any positional arguments will generate warnings.
///
/// # Arguments
///
/// * `stack` - Positional arguments (not used, will generate warnings)
/// * `scope` - Named arguments that will become map key-value pairs
/// * `_source` - Source information (not used)
/// * `span` - Span for error reporting
/// * `tracer` - Error tracer for reporting warnings
///
/// # Returns
///
/// A Value::Map containing all the named arguments from the scope
pub fn map(stack: Stack, scope: Scope, _source: Source, span: Span, tracer: &mut Tracer) -> Value {
    let mut checker = TypeChecker::new(tracer, span);
    checker.warn_unknown_positional(stack, 0);

    Value::Map(scope.into_inner())
}
