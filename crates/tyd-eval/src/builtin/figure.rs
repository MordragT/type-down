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
pub struct Figure;

impl Into<Value> for Figure {
    fn into(self) -> Value {
        Value::Func(figure)
    }
}

pub fn figure(
    mut stack: Stack,
    mut scope: Scope,
    _source: Source,
    span: Span,
    tracer: &mut Tracer,
) -> Value {
    let caption = match scope.try_remove::<ir::Content>("caption") {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Content,
                },
            );
            return Value::None;
        }
        None => {
            tracer.source_error(
                span,
                ArgumentError::MissingRequired {
                    name: "caption".into(),
                    ty: Type::Content,
                },
            );
            return Value::None;
        }
    };

    let content = match stack.try_pop::<ir::Content>() {
        Some(Ok(c)) => c,
        Some(Err(got)) => {
            tracer.source_error(
                span,
                TypeError::WrongType {
                    got,
                    expected: Type::Content,
                },
            );
            return Value::None;
        }
        None => {
            tracer.source_error(
                span,
                ArgumentError::MissingPositional {
                    pos: 0,
                    ty: Type::Content,
                },
            );
            return Value::None;
        }
    };

    let caption = (None, vec![ir::Block::Plain(caption)]);
    let content = ir::Block::Plain(content);
    let block = ir::Block::Figure(ir::AttrBuilder::empty(), caption, vec![content]);

    Value::Block(block)
}
