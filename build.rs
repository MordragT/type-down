use std::process;

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::{ParolError, Report};

fn main() {
    // CLI equivalent is:
    // parol -f ./type_down.par -e ./type_down-exp.par -p ./src/type_down_parser.rs -a ./src/type_down_grammar_trait.rs -t TypeDownGrammar -m type_down_grammar -g
    if let Err(err) = build() {
        ParolErrorReporter::report_error(&err, "type_down.par").unwrap_or_default();
        process::exit(1);
    }
}

fn build() -> Result<(), ParolError> {
    Builder::with_explicit_output_dir("src")
        .grammar_file("type_down.par")
        .expanded_grammar_output_file("../type_down-exp.par")
        .parser_output_file("parser.rs")
        .actions_output_file("grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("Grammar")
        .user_trait_module_name("grammar")
        .trim_parse_tree()
        .max_lookahead(1)
        .unwrap()
        .generate_parser()
}
