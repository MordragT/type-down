use std::env::current_dir;

use ecow::EcoString;
use pandoc_ast as ir;
use tyd_eval::{
    error::{EngineError, EngineMessage},
    eval::Machine,
    foundations::{Func, Signature, VerifiedCall},
    ty::Type,
    value::Value,
};

use crate::{attr::AttrBuilder, engine::PandocEngine};

#[derive(Debug, Clone, Copy)]
pub struct Image;

impl Func<PandocEngine> for Image {
    fn signature(&self) -> Signature<PandocEngine> {
        Signature::new("image")
            .required("src", Type::Str)
            .optional("alt", String::new())
            .optional("width", "auto")
            .optional("height", "auto")
    }

    fn run(
        &self,
        call: VerifiedCall<PandocEngine>,
        machine: &Machine<PandocEngine>,
    ) -> Result<Value<PandocEngine>, EngineError> {
        let VerifiedCall { span, mut args } = call;

        let src = args.remove_named::<EcoString>("src");
        let alt = args.remove_named::<EcoString>("alt");
        let width = args.remove_named::<EcoString>("width");
        let height = args.remove_named::<EcoString>("height");

        // work_path is the parent path of the file which is compiled at the moment
        let path = machine.world.work_path().join(src.as_str());

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

        let image = ir::Inline::Image(attrs, vec![ir::Inline::Str(alt.to_string())], target);

        Ok(Value::Inline(image))
    }
}
