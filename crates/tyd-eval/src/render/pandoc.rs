use std::fs;

use crate::{ir, tracer::Tracer};

use super::{Output, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PandocCompiler;

impl Render for PandocCompiler {
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
