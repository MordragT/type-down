use std::{
    fmt::Debug,
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use tyd_syntax::source::Source;

use crate::eval::Scope;

/// Environment in which typesetting occurs
#[derive(Debug, Clone)]
pub struct World(Arc<Repr>);

#[derive(Debug, Clone)]
struct Repr {
    source: Source,
    path: PathBuf,
    scope: Arc<Scope>,
}

impl World {
    pub fn new(path: impl AsRef<Path>, scope: impl Into<Arc<Scope>>) -> io::Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let name = path.file_name().unwrap().to_string_lossy();
        let source = fs::read_to_string(&path)?;
        let source = Source::new(&path, name, source);
        let repr = Repr {
            source,
            path,
            scope: scope.into(),
        };

        Ok(Self(Arc::new(repr)))
    }

    pub fn source(&self) -> Source {
        self.0.source.clone()
    }

    pub fn file_path(&self) -> &Path {
        &self.0.path
    }

    pub fn work_path(&self) -> &Path {
        self.0.path.parent().unwrap()
    }

    pub fn global_scope(&self) -> Arc<Scope> {
        self.0.scope.clone()
    }
}
