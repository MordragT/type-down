use std::{
    fmt::Debug,
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use miette::NamedSource;
use tyd_syntax::Source;

use crate::eval::{Engine, Scope};

/// Environment in which typesetting occurs
#[derive(Debug, Clone)]
pub struct World<E: Engine>(Arc<Repr<E>>);

#[derive(Debug, Clone)]
struct Repr<E: Engine> {
    source: Source,
    path: PathBuf,
    scope: Scope<E>,
}

impl<E: Engine> World<E> {
    pub fn new(path: impl AsRef<Path>, scope: Scope<E>) -> io::Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let name = path.file_name().unwrap().to_string_lossy();
        let source = fs::read_to_string(&path)?;
        let source = Source::new(name, source);
        let repr = Repr {
            source,
            path,
            scope,
        };

        Ok(Self(Arc::new(repr)))
    }

    pub fn source(&self) -> Source {
        self.0.source.clone()
    }

    pub fn named_source(&self) -> NamedSource<Arc<str>> {
        self.0.source.named_source()
    }

    pub fn file_path(&self) -> &Path {
        &self.0.path
    }

    pub fn work_path(&self) -> &Path {
        self.0.path.parent().unwrap()
    }

    pub fn global_scope(&self) -> &Scope<E> {
        &self.0.scope
    }
}
