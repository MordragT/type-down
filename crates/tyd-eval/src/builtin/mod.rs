//! Built-in module components for document formatting
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

/// Built-in plugin that provides the standard document components
///
/// This plugin registers all the basic components:
/// - Map: Creates key-value mappings
/// - List: Creates lists
/// - Figure: Adds figure elements with required captions
/// - Highlight: Highlights text
/// - HorizontalRule: Adds horizontal rule separators
/// - Image: Embeds images with optional sizing
/// - LineBreak: Inserts line breaks in the document flow
/// - SmallCaps: Formats text in small capitals
/// - Underline: Underlines the wrapped content
#[derive(Debug, Clone, Copy)]
pub struct BuiltinPlugin;

impl Plugin for BuiltinPlugin {
    /// Initializes the builtin plugin by registering all standard components
    /// with the provided scope
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
