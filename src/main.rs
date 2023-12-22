extern crate parol_runtime;

use anyhow::{anyhow, Context, Result};
use parol_runtime::{log::debug, Report};
use std::{env, fs, time::Instant};
use type_down::prelude::*;

// To generate:
// parol -f ./type_down.par -e ./type_down-exp.par -p ./src/type_down_parser.rs -a ./src/type_down_grammar_trait.rs -t TypeDownGrammar -m type_down_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .with_context(|| format!("Can't read file {}", file_name))?;
        let mut type_down_grammar = Grammar::new();
        let now = Instant::now();
        match parse(&input, &file_name, &mut type_down_grammar) {
            Ok(_) => {
                let elapsed_time = now.elapsed();
                println!("Parsing took {} milliseconds.", elapsed_time.as_millis());
                if args.len() > 2 && args[2] == "-q" {
                    Ok(())
                } else {
                    println!("Success!\n{}", type_down_grammar);
                    Ok(())
                }
            }
            Err(e) => ErrorReporter::report_error(&e, file_name),
        }
    } else {
        Err(anyhow!("Please provide a file name as first parameter!"))
    }
}
