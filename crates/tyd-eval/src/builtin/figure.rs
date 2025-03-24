use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents a figure element that can contain content with a caption.
///
/// A figure is a self-contained content element that typically includes
/// an image, diagram, or other visual content along with an optional caption.
#[derive(Debug, Clone, Copy)]
pub struct Figure;

impl Into<Value> for Figure {
    /// Converts this Figure into a Value containing the figure function.
    fn into(self) -> Value {
        Value::Func(figure)
    }
}

/// Creates a figure block with the provided content and caption.
///
/// # Arguments
///
/// * `stack` - Stack containing the figure's main content
/// * `scope` - Scope containing named parameters like caption
/// * `_source` - Source information (unused)
/// * `span` - Span information for error reporting
/// * `tracer` - Error tracer for reporting issues
///
/// # Returns
///
/// * `Value::Block` containing the figure if successful
/// * `Value::None` if required parameters are missing or have wrong types
///
/// # Expected Parameters
///
/// * First positional argument: The content to display in the figure (required)
/// * `caption`: Named parameter for the figure caption (required)
pub fn figure(
    mut stack: Stack,
    mut scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut type_checker = TypeChecker::new(tracer, span);

    let caption = match type_checker.remove_from_scope::<ir::Content>(&mut scope, "caption") {
        Some(c) => c,
        None => return Value::None,
    };

    let content = match type_checker.pop_from_stack::<ir::Content>(&mut stack, 0) {
        Some(c) => c,
        None => return Value::None,
    };

    type_checker.warn_unknown_named(scope);
    type_checker.warn_unknown_positional(stack, 1);

    let caption = (None, vec![ir::Block::Plain(caption)]);
    let content = ir::Block::Plain(content);
    let block = ir::Block::Figure(ir::AttrBuilder::empty(), caption, vec![content]);

    Value::Block(block)
}
