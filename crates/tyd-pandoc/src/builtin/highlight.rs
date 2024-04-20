pub fn linebreak(args: Args) -> Result<Value, EngineError> {
    use EngineError::*;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::LineBreak]))
}

pub fn underline(mut args: Args) -> Result<Value, EngineError> {
    use EngineError::*;

    let content = args
        .remove("content")
        .ok_or(MissingArgument("content".to_owned()))?;

    let content = content.into_content().ok_or(WrongArgType {
        arg: "content".to_owned(),
        expected: ValueKind::Content,
    })?;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::Underline(content)]))
}

pub fn smallcaps(mut args: Args) -> Result<Value, EngineError> {
    use EngineError::*;

    let content = args
        .remove("content")
        .ok_or(MissingArgument("content".to_owned()))?;

    let content = content.into_content().ok_or(WrongArgType {
        arg: "content".to_owned(),
        expected: ValueKind::Content,
    })?;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::SmallCaps(content)]))
}

pub fn highlight(args: Args) -> Result<Value, EngineError> {
    add_class("mark", args)
}

fn add_class(class: impl Into<String>, mut args: Args) -> Result<Value, EngineError> {
    use EngineError::*;

    let content = args
        .remove("content")
        .ok_or(MissingArgument("content".to_owned()))?;

    let content = content.into_content().ok_or(WrongArgType {
        arg: "content".to_owned(),
        expected: ValueKind::Content,
    })?;

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    Ok(Value::Content(vec![Inline::Span(
        AttrBuilder::new().class(class).build(),
        content,
    )]))
}
