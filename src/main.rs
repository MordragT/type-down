use miette::{Diagnostic, Result};
use std::{io, path::PathBuf};
use thiserror::Error;

#[cfg(feature = "html")]
use tyd_html::HtmlCompiler;
#[cfg(not(feature = "html"))]
use tyd_pandoc::format::HtmlCompiler;
use tyd_pandoc::{
    builtin,
    engine::PandocState,
    format::{DocxCompiler, PandocCompiler, PdfCompiler},
    Value,
};
use tyd_render::render::{Output, Render};
use tyd_syntax::{parser::error::SyntaxErrors, prelude::*};

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

#[derive(Debug, Error, Diagnostic)]
pub enum TydError {
    #[diagnostic(transparent)]
    #[error(transparent)]
    Parse(#[from] SyntaxErrors),
    #[error(transparent)]
    #[diagnostic(code(type_down::TydError::Io))]
    Io(#[from] io::Error),
}

fn main() -> Result<()> {
    let args: Args = clap::Parser::parse();

    match args.command {
        Commands::Check { path } => {
            let name = path.file_name().unwrap().to_string_lossy();
            let src = std::fs::read_to_string(&path).map_err(TydError::Io)?;

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
            let src = std::fs::read_to_string(&input).map_err(TydError::Io)?;

            let ast = parse(&src, name.clone())?;

            // TODO highlight only all working in html
            // therefor only add them in html and add a default case
            // which will throw a warning and skip the functions
            let ctx = PandocState::new(src, name, &input)
                .insert("title", "Default title")
                .insert("author", vec![Value::from("Max Mustermann")])
                //Blocks
                .register("hrule", builtin::HorizontalRule)
                .register("figure", builtin::Figure)
                // Inlines
                .register("image", builtin::Image)
                .register("linebreak", builtin::LineBreak)
                .register("highlight", builtin::Highlight)
                .register("smallcaps", builtin::SmallCaps)
                .register("underline", builtin::Underline)
                // Builtins
                .register("let", builtin::Let)
                .register("List", builtin::List)
                .register("Map", builtin::Map);

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
