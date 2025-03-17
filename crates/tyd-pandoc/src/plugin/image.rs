use std::env::current_dir;

use ecow::EcoString;
use tyd_eval::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Image;

impl Plugin for Image {
    fn signature() -> Signature {
        Signature::new("image")
            .required("src", Type::Str)
            .optional("alt", EcoString::new())
            .optional("width", "auto")
            .optional("height", "auto")
    }

    fn call(mut args: ir::Arguments, tracer: &mut Tracer) -> Value {
        let src = args.remove::<EcoString>("src").unwrap();
        let alt = args.remove::<EcoString>("alt").unwrap();
        let width = args.remove::<EcoString>("width").unwrap();
        let height = args.remove::<EcoString>("height").unwrap();

        // work_path is the parent path of the file which is compiled at the moment
        let path = args.source.work_path().join(src.as_str());

        if !path.exists() {
            tracer.error(args.span, EngineMessage::FileNotFound(path));
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
}
