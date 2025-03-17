use tyd_eval::{plugin::dispatch, scope::Scope, value::Value};

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

pub fn plugin() -> Scope {
    Scope::empty()
        .with("figure", Value::Func(dispatch::<Figure>))
        .with("highlight", Value::Func(dispatch::<Highlight>))
        .with("hrule", Value::Func(dispatch::<HorizontalRule>))
        .with("image", Value::Func(dispatch::<Image>))
        .with("linebreak", Value::Func(dispatch::<LineBreak>))
        .with("smallcaps", Value::Func(dispatch::<SmallCaps>))
        .with("underline", Value::Func(dispatch::<Underline>))
}
