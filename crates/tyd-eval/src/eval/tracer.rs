use crate::error::EngineError;

#[derive(Debug, Clone)]
pub struct Tracer {
    errors: Vec<EngineError>,
}

impl Tracer {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error(&mut self, e: EngineError) {
        self.errors.push(e);
    }

    pub fn errors(&mut self, errs: impl IntoIterator<Item = EngineError>) {
        self.errors.extend(errs)
    }

    pub fn into_errors(self) -> Vec<EngineError> {
        self.errors
    }

    pub fn drain_errors(&mut self) -> impl Iterator<Item = EngineError> + '_ {
        self.errors.drain(..)
    }
}
