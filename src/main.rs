use std::path::PathBuf;

use miette::Result;
use type_down::{
    compile::{
        docx::DocxCompiler, html::HtmlCompiler, image, pandoc::PandocCompiler, pdf::PdfCompiler,
        Compiler, ContextBuilder, Output,
    },
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
        format: Format,
        input: PathBuf,
        output: Option<PathBuf>,
    },
}

#[derive(clap::ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
    #[default]
    Html,
    Pdf,
    Docx,
    Json,
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
            format,
        } => {
            let cst = parse(&input)?;
            let ast = Ast::from(cst);

            let ctx = ContextBuilder::new("Testtitle".to_owned())
                .register_func("image", image)
                .build();

            let output = match output {
                Some(path) => Output::File(path),
                None => Output::Stdout,
            };

            match format {
                Format::Html => HtmlCompiler::compile(&ast, ctx, output)?,
                Format::Pdf => PdfCompiler::compile(&ast, ctx, output)?,
                Format::Docx => DocxCompiler::compile(&ast, ctx, output)?,
                Format::Json => PandocCompiler::compile(&ast, ctx, output)?,
            }
        }
    }

    Ok(())
}
