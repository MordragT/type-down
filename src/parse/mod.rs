use crate::error::{ParseError, TydError};
use miette::NamedSource;
use parasite::chumsky::{Context as ParseContext, Parseable, Parser};
use std::{fs::File, io::Read, path::Path};

pub use ast::Ast;
pub use cst::Cst;

pub mod ast;
pub mod cst;

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Cst, TydError> {
    let name = path.as_ref().as_os_str().to_string_lossy().into_owned();

    let mut file = File::open(path)?;
    let mut source = String::new();

    file.read_to_string(&mut source)?;

    source = source.trim().to_owned();
    source.push('\n');
    source.push('\n');

    let mut parse_ctx = ParseContext::new();
    let parser = Cst::parser(&mut parse_ctx);

    let cst = parser.parse(source.as_str()).map_err(|errs| ParseError {
        src: NamedSource::new(name, source),
        related: errs.into_iter().map(Into::into).collect(),
    })?;

    Ok(cst)
}
