use std::path::PathBuf;

use miette::Result;
use type_down::{
    compile::{docx::DocxCompiler, html::HtmlCompiler, pdf::PdfCompiler, Compiler, Context},
    parse::{parse, Ast},
};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Check {
        path: PathBuf,
    },
    Format {
        path: PathBuf,
    },
    Compile {
        compiler: CompilerBackend,
        input: PathBuf,
        output: PathBuf,
    },
}

#[derive(clap::ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilerBackend {
    #[default]
    Html,
    Pdf,
    Docx,
}

fn main() -> Result<()> {
    let args: Args = clap::Parser::parse();

    match args.command {
        Commands::Check { path } => {
            let cst = parse(path)?;

            println!("{cst:?}");
        }
        Commands::Format { path } => {
            let cst = parse(path)?;

            println!("{cst}");
        }
        Commands::Compile {
            input,
            output,
            compiler,
        } => {
            let cst = parse(&input)?;
            let ast = Ast::from(cst);

            let ctx = Context::new("Testtitle".to_owned(), input, output);

            match compiler {
                CompilerBackend::Html => HtmlCompiler::compile(&ctx, &ast)?,
                CompilerBackend::Pdf => PdfCompiler::compile(&ctx, &ast)?,
                CompilerBackend::Docx => DocxCompiler::compile(&ctx, &ast)?,
            }
        }
    }

    Ok(())
}
