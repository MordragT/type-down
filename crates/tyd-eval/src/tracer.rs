use std::fmt;

use miette::{Diagnostic, MietteDiagnostic, Severity};
use tyd_core::{id::NodeId, meta::MetaContainer};
use tyd_syntax::{error::SourceDiagnostic, source::Source, Span, SpanMetadata};

#[derive(Debug, Diagnostic)]
pub struct Tracer {
    #[source_code]
    pub(crate) source: Source,
    pub(crate) spans: SpanMetadata,
    #[related]
    source_diags: Vec<SourceDiagnostic>,
    #[related]
    diags: Vec<Box<dyn Diagnostic + Send + Sync>>,
}

impl Tracer {
    pub fn new(source: Source, spans: SpanMetadata) -> Self {
        Self {
            source,
            spans,
            source_diags: Vec::new(),
            diags: Vec::new(),
        }
    }

    pub fn with_diagnostics(
        diagnostics: Vec<SourceDiagnostic>,
        source: Source,
        spans: SpanMetadata,
    ) -> Self {
        Self {
            source,
            spans,
            source_diags: diagnostics,
            diags: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.source_diags.iter().any(SourceDiagnostic::is_error)
            || self.diags.iter().any(|diag| {
                diag.severity()
                    .map(|s| s == Severity::Error)
                    .unwrap_or(false)
            })
    }

    #[inline]
    pub fn diagnose(&mut self, diag: impl Diagnostic + Send + Sync + 'static) {
        let diag = Box::new(diag) as Box<dyn Diagnostic + Send + Sync>;
        self.diags.push(diag)
    }

    #[inline]
    pub fn error(&mut self, message: impl ToString) {
        let diag = MietteDiagnostic::new(message.to_string()).with_severity(Severity::Error);
        self.diagnose(diag);
    }

    #[inline]
    pub fn warn(&mut self, message: impl ToString) {
        let diag = MietteDiagnostic::new(message.to_string()).with_severity(Severity::Warning);
        self.diagnose(diag);
    }

    #[inline]
    pub fn info(&mut self, message: impl ToString) {
        let diag = MietteDiagnostic::new(message.to_string()).with_severity(Severity::Advice);
        self.diagnose(diag);
    }

    #[inline]
    pub fn source_error(&mut self, span: Span, message: impl ToString) {
        let diag = SourceDiagnostic::error(span, message.to_string());
        self.source_diags.push(diag);
    }

    #[inline]
    pub fn source_warn(&mut self, span: Span, message: impl ToString) {
        let diag = SourceDiagnostic::warn(span, message.to_string());
        self.source_diags.push(diag);
    }

    #[inline]
    pub fn source_info(&mut self, span: Span, message: impl ToString) {
        let diag = SourceDiagnostic::info(span, message.to_string());
        self.source_diags.push(diag);
    }

    #[inline]
    pub fn diagnose_source(&mut self, diag: impl Into<SourceDiagnostic>) {
        let diag = diag.into();
        self.source_diags.push(diag);
    }

    #[inline]
    pub fn node_error<T>(&mut self, id: NodeId<T>, message: impl ToString) {
        let span = self.spans.get(id).inner_copied();
        self.source_error(span, message);
    }

    #[inline]
    pub fn node_warn<T>(&mut self, id: NodeId<T>, message: impl ToString) {
        let span = self.spans.get(id).inner_copied();
        self.source_warn(span, message);
    }

    #[inline]
    pub fn node_info<T>(&mut self, id: NodeId<T>, message: impl ToString) {
        let span = self.spans.get(id).inner_copied();
        self.source_info(span, message);
    }
}

impl fmt::Display for Tracer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Trace:")
    }
}

impl std::error::Error for Tracer {}
