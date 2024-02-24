use tyd_syntax::ast::{Block, Image};

use crate::{Args, CallError, Object, ObjectKind};

pub fn image(mut args: Args) -> Result<Object, CallError> {
    use CallError::*;

    let src = args
        .remove("src")
        .ok_or(MissingArgument("src".to_owned()))?;

    let src = src.into_string().ok_or(WrongType {
        arg: "src".to_owned(),
        expected: ObjectKind::Str,
    })?;

    let alt = if let Some(alt) = args.remove("alt") {
        Some(alt.into_string().ok_or(WrongType {
            arg: "alt".to_owned(),
            expected: ObjectKind::Str,
        })?)
    } else {
        None
    };

    if !args.is_empty() {
        return Err(WrongArguments);
    }

    let object = Object::Block(Block::Image(Image { src, alt }));

    Ok(object)
}
