use pandoc_ast::Inline;
use tyd_render::{error::ContextError, ValueKind};

use crate::{attr::AttrBuilder, Content};

pub type Args = tyd_render::Args<Content>;
pub type Value = tyd_render::Value<Content>;

pub fn image(mut args: Args) -> Result<Value, ContextError> {
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

        content.push(Inline::Str(alt));
    }

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    let target = (src, String::new());
    let image = Inline::Image(AttrBuilder::empty(), content, target);

    Ok(Value::Content(vec![image]))
}
