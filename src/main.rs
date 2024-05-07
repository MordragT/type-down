use miette::{Diagnostic, NamedSource, Result};
use std::{io, path::PathBuf};
use thiserror::Error;

use tyd_eval::{builtin, prelude::*};
#[cfg(feature = "html")]
use tyd_html::HtmlCompiler;
#[cfg(not(feature = "html"))]
use tyd_pandoc::format::HtmlCompiler;
use tyd_pandoc::{
    format::{DocxCompiler, PandocCompiler, PdfCompiler},
    plugin,
};
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

    let scope = plugin::plugin()
        .into_scope()
        .register_symbol("title", "Default title")
        .register_symbol("author", vec![Value::from("Max Mustermann")])
        // Builtins
        .register_func("let", builtin::Let)
        .register_func("List", builtin::List)
        .register_func("Map", builtin::Map);

    match args.command {
        Commands::Check { path } => {
            let world = World::new(path, scope).map_err(TydError::Io)?;
            let mut parser = Parser::new(world.source());
            let result = parser.try_parse();

            if result.has_errors() {
                let related = result.errors().cloned().map(Into::into).collect::<Vec<_>>();
                let report: miette::Report = SyntaxErrors {
                    related,
                    src: world.named_source(),
                }
                .into();
                println!("{report:?}");
            }

            if let Some(ast) = result.into_output() {
                println!("{ast:?}");
            }
        }
        Commands::Format { path } => {
            todo!()
        }
        Commands::Compile {
            input,
            output,
            format,
        } => {
            let world = World::new(input, scope).map_err(TydError::Io)?;
            let mut parser = Parser::new(world.source());
            let node = parser.parse()?;
            let doc = Document::from_node(&node).expect("what the hell happened here ? ;)");

            let output = match output {
                Some(path) => Output::File(path),
                None => Output::Stdout,
            };

            match format {
                Format::Html => HtmlCompiler::render(doc, world, output)?,
                Format::Pdf => PdfCompiler::render(doc, world, output)?,
                Format::Docx => DocxCompiler::render(doc, world, output)?,
                Format::Json => PandocCompiler::render(doc, world, output)?,
            }
        }
    }

    Ok(())
}
