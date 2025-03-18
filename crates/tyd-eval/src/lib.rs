pub mod builtin;
pub mod engine;
pub mod error;
pub mod ir;
pub mod render;
pub mod scope;
pub mod stack;
pub mod tracer;
pub mod ty;
pub mod value;

pub mod prelude {
    pub use crate::builtin::BuiltinPlugin;
    pub use crate::engine::{Engine, EngineResult};
    pub use crate::error::*;
    pub use crate::ir;
    pub use crate::render::{
        DocxCompiler, HtmlCompiler, Output, PandocCompiler, PdfCompiler, Render,
    };
    pub use crate::scope::Scope;
    pub use crate::tracer::Tracer;
    pub use crate::ty::Type;
    pub use crate::value::{TypeCast, Value};
    pub use crate::Plugin;
}

pub trait Plugin {
    fn init(scope: &mut crate::scope::Scope);
}
