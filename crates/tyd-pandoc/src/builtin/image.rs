use pandoc_ast as ir;
use tyd_render::{error::EngineError, Command, Type};

use crate::{attr::AttrBuilder, PandocShape, Signature, ValidArgs, Value};

pub struct Image;

impl Command<PandocShape> for Image {
    fn signature(&self) -> Signature {
        Signature::new("image")
            .required("src", Type::Str)
            .optional("alt", Type::Str, String::new())
            .optional("width", Type::Str, "auto")
            .optional("height", Type::Str, "auto")
    }

    fn run(&self, args: &mut ValidArgs) -> Result<Value, EngineError> {
        let src = args.str("src");
        let alt = args.str("alt");
        let width = args.str("width");
        let height = args.str("height");

        let attrs = AttrBuilder::new()
            .attr("width", width)
            .attr("height", height)
            .build();
        let target = (src, String::new());

        let image = ir::Inline::Image(attrs, vec![ir::Inline::Str(alt)], target);

        Ok(Value::Inline(image))
    }
}
