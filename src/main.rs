use miette::{IntoDiagnostic, Report, Result};
use std::path::PathBuf;

use tyd_eval::prelude::*;
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

    let mut global_scope = Scope::empty();

    global_scope
        .register::<BuiltinPlugin>()
        .with("title", "Default title")
        .with("author", vec![Value::from("Max Mustermann")]);

    match args.command {
        Commands::Check { path } => {
            let source = Source::from_path(path).into_diagnostic()?;

            let ParseResult { doc, spans, errors } = parse(&source);

            let tracer = Tracer::with_diagnostics(errors, source, spans);

            let doc = if let Some(doc) = doc {
                doc
            } else {
                return Err(tracer.into());
            };

            let EngineResult { pandoc, mut tracer } = Engine::new(global_scope, tracer).run(doc);

            let pandoc = if let Some(pandoc) = pandoc {
                pandoc
            } else {
                return Err(tracer.into());
            };

            PandocCompiler::render(pandoc, Output::Stdout, &mut tracer);

            eprintln!("{:?}", Report::new(tracer))
        }
        Commands::Format { path } => {
            todo!()
        }
        Commands::Compile {
            input,
            output,
            format,
        } => {
            let source = Source::from_path(input).into_diagnostic()?;

            let ParseResult { doc, spans, errors } = parse(&source);

            let tracer = Tracer::with_diagnostics(errors, source, spans);

            let doc = if let Some(doc) = doc {
                doc
            } else {
                return Err(tracer.into());
            };

            let EngineResult { pandoc, mut tracer } = Engine::new(global_scope, tracer).run(doc);

            let pandoc = if let Some(pandoc) = pandoc {
                pandoc
            } else {
                return Err(tracer.into());
            };

            let output = match output {
                Some(path) => Output::File(path),
                None => Output::Stdout,
            };

            match format {
                Format::Html => HtmlCompiler::render(pandoc, output, &mut tracer),
                Format::Pdf => PdfCompiler::render(pandoc, output, &mut tracer),
                Format::Docx => DocxCompiler::render(pandoc, output, &mut tracer),
                Format::Json => PandocCompiler::render(pandoc, output, &mut tracer),
            }

            eprintln!("{:?}", Report::new(tracer))
        }
    }

    Ok(())
}
