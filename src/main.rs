use miette::Result;
use std::path::PathBuf;

#[cfg(feature = "html")]
use tyd_html::HtmlCompiler;
#[cfg(not(feature = "html"))]
use tyd_pandoc::html::HtmlCompiler;
use tyd_pandoc::{docx::DocxCompiler, pandoc::PandocCompiler, pdf::PdfCompiler};
use tyd_render::{Context, Output, Render};
use tyd_syntax::{parse, Ast};

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

            let ctx = Context::new().symbol("title", "Default title");

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
