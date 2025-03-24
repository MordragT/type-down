use std::env::current_dir;

use ecow::EcoString;
use tyd_syntax::{source::Source, Span};

use crate::{
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    value::{TypeChecker, Value},
};

/// Represents an image that can be embedded in a document.
///
/// This struct is used as a constructor for the `image` function.
#[derive(Debug, Clone, Copy)]
pub struct Image;

impl Into<Value> for Image {
    /// Converts the Image struct into a function Value.
    fn into(self) -> Value {
        Value::Func(image)
    }
}

/// Creates an image element with specified attributes.
///
/// # Parameters
///
/// * `stack` - Stack of positional arguments (not used by this function)
/// * `scope` - Named parameters including:
///   * `src` - (required) Path to the image file, relative to the current document
///   * `alt` - (optional) Alternative text for the image, defaults to empty string
///   * `width` - (optional) Width specification for the image, defaults to "auto"
///   * `height` - (optional) Height specification for the image, defaults to "auto"
/// * `source` - Source information for the current document being processed
/// * `span` - Span in the source code where this function is called
/// * `tracer` - Error tracer for reporting issues
///
/// # Returns
///
/// Returns a Value containing the inline image element or Value::None if an error occurred.
pub fn image(
    stack: Stack,
    mut scope: Scope,
    source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let mut checker = TypeChecker::new(tracer, span);

    // Required 'src' parameter
    let src = match checker.remove_from_scope::<EcoString>(&mut scope, "src") {
        Some(src) => src,
        None => return Value::None,
    };

    // Optional 'alt' parameter (with empty default)
    let alt = checker
        .remove_from_scope::<EcoString>(&mut scope, "alt")
        .unwrap_or(EcoString::new());

    // Optional 'width' parameter (with "auto" default)
    let width = checker
        .remove_from_scope::<EcoString>(&mut scope, "width")
        .unwrap_or("auto".into());

    // Optional 'height' parameter (with "auto" default)
    let height = checker
        .remove_from_scope::<EcoString>(&mut scope, "height")
        .unwrap_or("auto".into());

    // Warn about unknown positional arguments
    checker.warn_unknown_positional(stack, 0);

    // Warn about unknown named arguments
    checker.warn_unknown_named(scope);

    // work_path is the parent path of the file which is compiled at the moment
    let path = source.work_path().join(src.as_str());

    if !path.exists() {
        tracer.source_error(span, format!("Path not found: {path:?}"));
        return Value::None;
    }

    // FIXME does some magic here to get the src path relative to the working directory
    // from where the executable was called,
    // as pandoc it seems like does not like absolute paths when generating e.g. pdfs
    let src = path
        .canonicalize()
        .unwrap()
        .strip_prefix(current_dir().unwrap())
        .unwrap()
        .to_string_lossy()
        .to_string();

    let attrs = ir::AttrBuilder::new()
        .attr("width", width)
        .attr("height", height)
        .build();
    let target = (src, String::new());

    let image = ir::Inline::Image(attrs, vec![ir::Inline::Str(alt.to_string())], target);

    Value::Inline(image)
}
