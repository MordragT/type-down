use miette::{Diagnostic, NamedSource, Result};
use std::{io, path::PathBuf};
use thiserror::Error;

use tyd_eval::{builtin, engine::Engine, prelude::*, scope::Scope};

use tyd_pandoc::{plugin, DocxCompiler, HtmlCompiler, PandocCompiler, PdfCompiler};
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

    let global_scope = plugin::plugin()
        .with("title", "Default title")
        .with("author", vec![Value::from("Max Mustermann")])
        .with("List", builtin::List)
        .with("Map", builtin::Map);

    match args.command {
        Commands::Check { path } => {
            let source = Source::from_path(path).map_err(TydError::Io)?;

            let ParseResult { doc, spans, errors } = parse(&source);

            let doc = doc.ok_or(errors)?;

            let pandoc = Engine::new(global_scope, spans, source).run(doc)?;

            PandocCompiler::render(pandoc, Output::Stdout)?;
        }
        Commands::Format { path } => {
            todo!()
        }
        Commands::Compile {
            input,
            output,
            format,
        } => {
            let source = Source::from_path(input).map_err(TydError::Io)?;

            let ParseResult { doc, spans, errors } = parse(&source);

            let doc = doc.ok_or(errors)?;

            let pandoc = Engine::new(global_scope, spans, source).run(doc)?;

            let output = match output {
                Some(path) => Output::File(path),
                None => Output::Stdout,
            };

            match format {
                Format::Html => HtmlCompiler::render(pandoc, output)?,
                Format::Pdf => PdfCompiler::render(pandoc, output)?,
                Format::Docx => DocxCompiler::render(pandoc, output)?,
                Format::Json => PandocCompiler::render(pandoc, output)?,
            }
        }
    }

    Ok(())
}
