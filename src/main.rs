use miette::Result;
use std::path::PathBuf;

#[cfg(feature = "html")]
use tyd_html::HtmlCompiler;
#[cfg(not(feature = "html"))]
use tyd_pandoc::format::HtmlCompiler;
use tyd_pandoc::{
    builtin,
    engine::PandocState,
    format::{DocxCompiler, PandocCompiler, PdfCompiler},
};
use tyd_render::{Output, Render, Value};
use tyd_syntax::prelude::*;

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
            let name = path.file_name().unwrap().to_string_lossy();
            let src = std::fs::read_to_string(&path).map_err(SyntaxError::Io)?;

            let ast = parse(&src, name)?;

            println!("{ast:?}");
        }
        Commands::Format { path } => {
            todo!()
        }
        Commands::Compile {
            input,
            output,
            format,
        } => {
            let name = input.file_name().unwrap().to_string_lossy();
            let src = std::fs::read_to_string(&input).map_err(SyntaxError::Io)?;

            let ast = parse(&src, name.clone())?;

            // TODO highlight, smallcaps, underline only all working in html
            // therefor only add them in html and add a default case
            // which will throw a warning and skip the functions
            let ctx = PandocState::new(src, name)
                .insert("title", "Default title")
                .insert("author", vec![Value::from("Max Mustermann")])
                .register("image", builtin::Image);
            // .function("linebreak", builtin::linebreak)
            // .function("highlight", builtin::highlight)
            // .function("smallcaps", builtin::smallcaps)
            // .function("underline", builtin::underline);

            let output = match output {
                Some(path) => Output::File(path),
                None => Output::Stdout,
            };

            match format {
                Format::Html => HtmlCompiler::render(&ast, ctx, output)?,
                Format::Pdf => PdfCompiler::render(&ast, ctx, output)?,
                Format::Docx => DocxCompiler::render(&ast, ctx, output)?,
                Format::Json => PandocCompiler::render(&ast, ctx, output)?,
            }
        }
    }

    Ok(())
}
