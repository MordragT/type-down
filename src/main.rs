use std::path::PathBuf;

use miette::Result;
use type_down::{context::Context, html::HtmlCompiler, parse, Compiler};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Check { path: PathBuf },
    Format { path: PathBuf },
    Compile { input: PathBuf, output: PathBuf },
}

fn main() -> Result<()> {
    let args: Args = clap::Parser::parse();

    match args.command {
        Commands::Check { path } => {
            let cst = parse(path)?;

            println!("{cst:?}");

            Ok(())
        }
        Commands::Format { path } => {
            let cst = parse(path)?;

            println!("{cst}");

            Ok(())
        }
        Commands::Compile { input, output } => {
            let cst = parse(&input)?;
            let ast = cst.into();

            let ctx = Context::new("Testtitle".to_owned(), input, output);
            HtmlCompiler::compile(&ctx, &ast)?;

            Ok(())
        }
    }
}
