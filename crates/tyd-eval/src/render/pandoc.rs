use std::fs;

use crate::{ir, tracer::Tracer};

use super::{Output, Render};

/// A compiler that renders Pandoc IR to JSON format.
///
/// This compiler takes a Pandoc intermediate representation and serializes it
/// to a JSON string using `serde_json`. The output can be directed to either
/// standard output or a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
    /// Renders Pandoc IR to JSON and outputs it according to the specified output target.
    ///
    /// # Arguments
    ///
    /// * `pandoc` - The Pandoc intermediate representation to render
    /// * `output` - Where to direct the rendered output (stdout or a file)
    /// * `tracer` - Error tracer for reporting any issues during rendering
    ///
    /// # Behavior
    ///
    /// This method serializes the Pandoc IR to a prettified JSON string and then:
    /// - If `output` is `Output::Stdout`, prints the JSON to standard output
    /// - If `output` is `Output::File(path)`, writes the JSON to the specified file path
    ///   and reports any file writing errors to the tracer
    fn render(pandoc: ir::Pandoc, output: Output, tracer: &mut Tracer) {
        let contents = serde_json::to_string_pretty(&pandoc).unwrap();

        match output {
            Output::Stdout => println!("{contents}"),
            Output::File(path) => {
                if let Err(e) = fs::write(path, contents) {
                    tracer.error(e);
                }
            }
        }
    }
}
