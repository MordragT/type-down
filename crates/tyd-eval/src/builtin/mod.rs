mod figure;
mod highlight;
mod hrule;
mod image;
mod linebreak;
mod list;
mod map;
mod smallcaps;
mod underline;

pub use figure::Figure;
pub use highlight::Highlight;
pub use hrule::HorizontalRule;
pub use image::Image;
pub use linebreak::LineBreak;
pub use list::List;
pub use map::Map;
pub use smallcaps::SmallCaps;
pub use underline::Underline;

use crate::{scope::Scope, Plugin};

#[derive(Debug, Clone, Copy)]
pub struct BuiltinPlugin;

impl Plugin for BuiltinPlugin {
    fn init(scope: &mut Scope) {
        scope
            .with("Map", Map)
            .with("List", List)
            .with("figure", Figure)
            .with("highlight", Highlight)
            .with("hrule", HorizontalRule)
            .with("image", Image)
            .with("linebreak", LineBreak)
            .with("smallcaps", SmallCaps)
            .with("underline", Underline);
    }
}
