use pandoc_ast::Inline;
use tyd_render::{error::ContextError, ValueKind};

use crate::{attr::AttrBuilder, Content};

pub type Args = tyd_render::Args<Content>;
pub type Value = tyd_render::Value<Content>;

pub fn image(mut args: Args) -> Result<Value, ContextError> {
    // width, height key=value pairs in attrs
    use ContextError::*;

    let src = args
        .remove("src")
        .ok_or(MissingArgument("src".to_owned()))?;

    let src = src.into_string().ok_or(WrongArgType {
        arg: "src".to_owned(),
        expected: ValueKind::Str,
    })?;

    let mut content = if let Some(content) = args.remove("content") {
        content.into_content().ok_or(WrongArgType {
            arg: "content".to_owned(),
            expected: ValueKind::Content,
        })?
    } else {
        Vec::new()
    };

    if let Some(alt) = args.remove("alt") {
        let alt = alt.into_string().ok_or(WrongArgType {
            arg: "alt".to_owned(),
            expected: ValueKind::Str,
        })?;

        content.push(Inline::Str(alt.to_string()));
    }

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    let target = (src.to_string(), String::new());
    let image = Inline::Image(AttrBuilder::empty(), content, target);

    Ok(Value::Content(vec![image]))
}

// raw/code .numberLines
// highlight text: .mark

pub fn linebreak(args: Args) -> Result<Value, ContextError> {
    use ContextError::*;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::LineBreak]))
}

pub fn highlight(mut args: Args) -> Result<Value, ContextError> {
    use ContextError::*;

    // TODO use content when able

    let text = args
        .remove("text")
        .ok_or(MissingArgument("text".to_owned()))?;

    let text = text.into_string().ok_or(WrongArgType {
        arg: "text".to_owned(),
        expected: ValueKind::Str,
    })?;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::Span(
        AttrBuilder::new().class("mark").build(),
        vec![Inline::Str(text)],
    )]))
}

pub fn underline(mut args: Args) -> Result<Value, ContextError> {
    use ContextError::*;

    // TODO use content when able

    let text = args
        .remove("text")
        .ok_or(MissingArgument("text".to_owned()))?;

    let text = text.into_string().ok_or(WrongArgType {
        arg: "text".to_owned(),
        expected: ValueKind::Str,
    })?;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::Span(
        AttrBuilder::new().class("underline").build(),
        vec![Inline::Str(text)],
    )]))
}

pub fn smallcaps(mut args: Args) -> Result<Value, ContextError> {
    use ContextError::*;

    // TODO use content when able

    let text = args
        .remove("text")
        .ok_or(MissingArgument("text".to_owned()))?;

    let text = text.into_string().ok_or(WrongArgType {
        arg: "text".to_owned(),
        expected: ValueKind::Str,
    })?;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::Span(
        AttrBuilder::new().class("smallcaps").build(),
        vec![Inline::Str(text)],
    )]))
}
