use std::env::current_dir;

use pandoc_ast as ir;
use tyd_render::{
    command::Command,
    context::Context,
    error::{EngineError, EngineMessage},
    ty::Type,
};

use crate::{attr::AttrBuilder, engine::PandocState, Call, PandocShape, Signature, Value};

pub struct Image;

impl Command<PandocShape, PandocState> for Image {
    fn signature(&self) -> Signature {
        Signature::new("image")
            .required("src", Type::Str)
            .optional("alt", String::new())
            .optional("width", "auto")
            .optional("height", "auto")
    }

    fn run(&self, call: Call, ctx: &PandocState) -> Result<Value, EngineError> {
        let Call { span, mut args } = call;

        let src = args.remove_named::<String>("src");
        let alt = args.remove_named::<String>("alt");
        let width = args.remove_named::<String>("width");
        let height = args.remove_named::<String>("height");

        // work_path is the parent path of the file which is compiled at the moment
        let path = ctx.work_path().join(src);

        if !path.exists() {
            return Err(EngineError::new(span, EngineMessage::FileNotFound(path)));
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

        let attrs = AttrBuilder::new()
            .attr("width", width)
            .attr("height", height)
            .build();
        let target = (src, String::new());

        let image = ir::Inline::Image(attrs, vec![ir::Inline::Str(alt)], target);

        Ok(Value::Inline(image))
    }
}
