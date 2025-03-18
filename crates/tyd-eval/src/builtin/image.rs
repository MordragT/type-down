use std::env::current_dir;

use ecow::EcoString;
use tyd_syntax::{source::Source, Span};

use crate::{
    error::{ArgumentError, TypeError},
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
    ty::Type,
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct Image;

impl Into<Value> for Image {
    fn into(self) -> Value {
        Value::Func(image)
    }
}

pub fn image(
    stack: Stack,
    mut scope: Scope,
    source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let src = match scope.try_remove::<EcoString>("src") {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Str,
                },
            );
            return Value::None;
        }
        None => {
            tracer.source_error(
                span,
                ArgumentError::MissingRequired {
                    name: "src".into(),
                    ty: Type::Str,
                },
            );
            return Value::None;
        }
    };

    let alt = match scope.try_remove::<EcoString>("src") {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Str,
                },
            );
            return Value::None;
        }
        None => EcoString::new(),
    };

    let width = match scope.try_remove::<EcoString>("width") {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Str,
                },
            );
            return Value::None;
        }
        None => "auto".into(),
    };

    let height = match scope.try_remove::<EcoString>("height") {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Str,
                },
            );
            return Value::None;
        }
        None => "auto".into(),
    };

    for (pos, _) in stack.into_iter().enumerate() {
        tracer.source_warn(span, ArgumentError::UnknownPositional { pos });
    }

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
