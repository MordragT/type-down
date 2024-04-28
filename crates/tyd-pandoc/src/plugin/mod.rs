use crate::engine::PandocEngine;
use tyd_eval::plugin::Plugin;

mod figure;
mod highlight;
mod hrule;
mod image;
mod linebreak;
mod smallcaps;
mod underline;

pub use tyd_eval::builtin::*;

pub use figure::*;
pub use highlight::*;
pub use hrule::*;
pub use image::*;
pub use linebreak::*;
pub use smallcaps::*;
pub use underline::*;

pub fn plugin() -> Plugin<PandocEngine> {
    Plugin::new()
        .register_func::<Figure>("figure")
        .register_func::<Highlight>("highlight")
        .register_func::<HorizontalRule>("hrule")
        .register_func::<Image>("image")
        .register_func::<LineBreak>("linebreak")
        .register_func::<SmallCaps>("smallcaps")
        .register_func::<Underline>("underline")
}
