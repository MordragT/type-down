use std::{fs::File, io::Read, path::PathBuf};

use ariadne::{Color, Label, Report, ReportKind, Source};
use parasite::chumsky::{Context as ParseContext, Parseable, Parser};
use type_down::{context::Context, cst::Cst, html::HtmlCompiler, Compiler};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Check { path: PathBuf },
    Compile { input: PathBuf, output: PathBuf },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = clap::Parser::parse();

    match args.command {
        Commands::Check { path } => {
            let mut file = File::open(path)?;

            let mut input = String::new();
            file.read_to_string(&mut input)?;

            input = input.trim().to_owned();
            input.push('\n');
            input.push('\n');

            let mut parse_ctx = ParseContext::new();
            let parser = Cst::parser(&mut parse_ctx);
            match parser.parse(input.as_str()) {
                Ok(cst) => {
                    println!("{cst:#?}");
                }
                Err(errs) => {
                    for err in errs {
                        Report::build(ReportKind::Error, (), err.span().start)
                            .with_code(3)
                            .with_message(err.to_string())
                            .with_label(
                                Label::new(err.span())
                                    // .with_message(err.reason().to_string())
                                    .with_color(Color::Red),
                            )
                            .finish()
                            .eprint(Source::from(&input))
                            .unwrap();
                    }
                }
            }

            Ok(())
        }
        Commands::Compile { input, output } => {
            let mut file = File::open(&input)?;

            let ctx = Context::new("Testtitle".to_owned(), input, output);

            let mut input = String::new();
            file.read_to_string(&mut input)?;

            input = input.trim().to_owned();
            input.push('\n');
            let mut parse_ctx = ParseContext::new();

            let parser = Cst::parser(&mut parse_ctx);
            match parser.parse(input.as_str()) {
                Ok(cst) => {
                    let ast = cst.into();
                    HtmlCompiler::compile(&ctx, &ast)?;
                }
                Err(errs) => {
                    for err in errs {
                        Report::build(ReportKind::Error, (), err.span().start)
                            .with_code(3)
                            .with_message(err.to_string())
                            .with_label(
                                Label::new(err.span())
                                    // .with_message(err.reason().to_string())
                                    .with_color(Color::Red),
                            )
                            .finish()
                            .eprint(Source::from(&input))
                            .unwrap();
                    }
                }
            }

            Ok(())
        }
    }
}
