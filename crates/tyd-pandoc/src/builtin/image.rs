use pandoc_ast as ir;
use tyd_render::{error::EngineError, Command, Type};

use crate::{attr::AttrBuilder, Arguments, PandocShape, Signature, Value};

pub struct Image;

impl Command<PandocShape> for Image {
    fn signature(&self) -> Signature {
        Signature::new("image")
            .required("src", Type::Str)
            .optional("alt", String::new())
            .optional("width", "auto")
            .optional("height", "auto")
    }

    fn run(&self, args: &mut Arguments) -> Result<Value, EngineError> {
        let src = args.remove_named("src");
        let alt = args.remove_named("alt");
        let width = args.remove_named::<String>("width");
        let height = args.remove_named::<String>("height");

        let attrs = AttrBuilder::new()
            .attr("width", width)
            .attr("height", height)
            .build();
        let target = (src, String::new());

        let image = ir::Inline::Image(attrs, vec![ir::Inline::Str(alt)], target);

        Ok(Value::Inline(image))
    }
}
