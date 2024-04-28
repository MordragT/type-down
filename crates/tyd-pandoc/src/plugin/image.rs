use std::env::current_dir;

use ecow::EcoString;
use pandoc_ast as ir;
use tyd_eval::{
    error::{EngineError, EngineMessage},
    eval::Engine,
    hir,
    plugin::{PluginFunc, Signature},
    ty::Type,
    value::Value,
};

use crate::{attr::AttrBuilder, engine::PandocEngine, visitor::PandocVisitor};

#[derive(Debug, Clone, Copy)]
pub struct Image;

impl PluginFunc<PandocEngine> for Image {
    fn signature() -> Signature<PandocEngine> {
        Signature::new("image")
            .required("src", Type::Str)
            .optional("alt", String::new())
            .optional("width", "auto")
            .optional("height", "auto")
    }

    fn call(
        mut args: hir::Args<PandocEngine>,
        engine: &mut PandocEngine,
        _visitor: &PandocVisitor,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let src = args.remove_named::<EcoString>("src");
        let alt = args.remove_named::<EcoString>("alt");
        let width = args.remove_named::<EcoString>("width");
        let height = args.remove_named::<EcoString>("height");

        // work_path is the parent path of the file which is compiled at the moment
        let path = engine.world().work_path().join(src.as_str());

        if !path.exists() {
            return Err(EngineError::new(
                args.span,
                EngineMessage::FileNotFound(path),
            ));
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

        let image = ir::Inline::Image(attrs, vec![ir::Inline::Str(alt.to_string())], target);

        Ok(Value::Inline(image))
    }
}
