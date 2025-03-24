use miette::{IntoDiagnostic, Report, Result};
use std::path::PathBuf;

use tyd_eval::prelude::*;
use tyd_syntax::prelude::*;

/// Command line arguments for the TypeDown document processor
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The command to execute
    #[command(subcommand)]
    command: Commands,
}

/// Available commands for processing TypeDown documents
#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Check a TYD document for errors
    Check {
        /// Path to the document to check
        path: PathBuf,
    },
    /// Format a TYD document
    Format {
        /// Path to the document to format
        path: PathBuf,
    },
    /// Compile a TYD document to another format
    Compile {
        /// Output format to compile to
        format: Format,
        /// Path to the input document
        input: PathBuf,
        /// Optional path for the output file (defaults to stdout)
        output: Option<PathBuf>,
    },
}

/// Supported output formats for document compilation
#[derive(clap::ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
    /// HTML format (default)
    #[default]
    Html,
    /// PDF format
    Pdf,
    /// Microsoft Word DOCX format
    Docx,
    /// JSON format (Pandoc intermediate representation)
    Json,
}

/// Entry point for the TYD document processor
fn main() -> Result<()> {
    // Parse command line arguments
    let args: Args = clap::Parser::parse();

    // Initialize the global scope with default values
    let mut global_scope = Scope::empty();

    global_scope
        .register::<BuiltinPlugin>()
        .with("title", "Default title")
        .with("author", vec![Value::from("Max Mustermann")]);

    match args.command {
        Commands::Check { path } => {
            // Load the source document from the specified path
            let source = Source::from_path(path).into_diagnostic()?;

            // Parse the document
            let ParseResult { doc, spans, errors } = parse(&source);

            // Initialize the tracer with any parse errors
            let tracer = Tracer::with_diagnostics(errors, source, spans);

            // Return error if parsing failed
            let doc = if let Some(doc) = doc {
                doc
            } else {
                return Err(tracer.into());
            };

            // Run the evaluation engine
            let EngineResult { pandoc, mut tracer } = Engine::new(global_scope, tracer).run(doc);

            // Return error if evaluation failed
            let pandoc = if let Some(pandoc) = pandoc {
                pandoc
            } else {
                return Err(tracer.into());
            };

            // Render to Pandoc format
            PandocCompiler::render(pandoc, Output::Stdout, &mut tracer);

            // Display any warnings or non-fatal errors
            eprintln!("{:?}", Report::new(tracer))
        }
        Commands::Format { path: _ } => {
            // Not yet implemented
            todo!()
        }
        Commands::Compile {
            input,
            output,
            format,
        } => {
            // Load the source document from the specified path
            let source = Source::from_path(input).into_diagnostic()?;

            // Parse the document
            let ParseResult { doc, spans, errors } = parse(&source);

            // Initialize the tracer with any parse errors
            let tracer = Tracer::with_diagnostics(errors, source, spans);

            // Return error if parsing failed
            let doc = if let Some(doc) = doc {
                doc
            } else {
                return Err(tracer.into());
            };

            // Run the evaluation engine
            let EngineResult { pandoc, mut tracer } = Engine::new(global_scope, tracer).run(doc);

            // Return error if evaluation failed
            let pandoc = if let Some(pandoc) = pandoc {
                pandoc
            } else {
                return Err(tracer.into());
            };

            // Determine output destination
            let output = match output {
                Some(path) => Output::File(path),
                None => Output::Stdout,
            };

            // Compile to the specified format
            match format {
                Format::Html => HtmlCompiler::render(pandoc, output, &mut tracer),
                Format::Pdf => PdfCompiler::render(pandoc, output, &mut tracer),
                Format::Docx => DocxCompiler::render(pandoc, output, &mut tracer),
                Format::Json => PandocCompiler::render(pandoc, output, &mut tracer),
            }

            // Display any warnings or non-fatal errors
            eprintln!("{:?}", Report::new(tracer))
        }
    }

    Ok(())
}
