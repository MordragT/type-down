use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use ariadne::{Color, Label, Report, ReportKind, Source};
use parasite::chumsky::{Parseable, Parser};
use type_down::{ast::TypeDown, html::to_html};

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

            let parser = TypeDown::parser();
            match parser.parse(input.as_str()) {
                Ok(ast) => {
                    println!("{ast:#?}");
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
            let mut file = File::open(input)?;

            let mut input = String::new();
            file.read_to_string(&mut input)?;

            input = input.trim().to_owned();
            input.push('\n');
            input.push('\n');

            let parser = TypeDown::parser();
            match parser.parse(input.as_str()) {
                Ok(ast) => {
                    let body = ast.into();
                    let html = to_html(body);

                    std::fs::write(output, html)?;
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
