use std::fmt;

use miette::{Diagnostic, MietteDiagnostic, Severity};
use tyd_core::{id::NodeId, meta::MetaContainer};
use tyd_syntax::{error::SourceDiagnostic, source::Source, Span, Spans};

/// `Tracer` is a diagnostic tool that collects and manages error messages,
/// warnings, and informational messages during parsing and compilation.
///
/// It maintains references to the source code and span metadata to provide
/// context-aware error reporting.
#[derive(Debug, Diagnostic)]
pub struct Tracer {
    /// The source code being analyzed
    #[source_code]
    pub(crate) source: Source,
    /// Source spans for nodes
    pub(crate) spans: Spans,
    /// Collection of source-specific diagnostics
    #[related]
    source_diags: Vec<SourceDiagnostic>,
    /// Collection of general diagnostics
    #[related]
    diags: Vec<Box<dyn Diagnostic + Send + Sync>>,
}

impl Tracer {
    /// Creates a new `Tracer` with the given source and span metadata.
    ///
    /// # Arguments
    /// * `source` - The source code being analyzed
    /// * `spans` - Metadata about the spans in the source
    pub fn new(source: Source, spans: Spans) -> Self {
        Self {
            source,
            spans,
            source_diags: Vec::new(),
            diags: Vec::new(),
        }
    }

    /// Creates a new `Tracer` with pre-existing diagnostics.
    ///
    /// # Arguments
    /// * `diagnostics` - Initial source diagnostics to include
    /// * `source` - The source code being analyzed
    /// * `spans` - Metadata about the spans in the source
    pub fn with_diagnostics(
        diagnostics: Vec<SourceDiagnostic>,
        source: Source,
        spans: Spans,
    ) -> Self {
        Self {
            source,
            spans,
            source_diags: diagnostics,
            diags: Vec::new(),
        }
    }

    /// Checks if the tracer contains any error-level diagnostics.
    ///
    /// # Returns
    /// `true` if any errors have been recorded, `false` otherwise.
    pub fn has_errors(&self) -> bool {
        self.source_diags.iter().any(SourceDiagnostic::is_error)
            || self.diags.iter().any(|diag| {
                diag.severity()
                    .map(|s| s == Severity::Error)
                    .unwrap_or(false)
            })
    }

    /// Adds a general diagnostic to the tracer.
    ///
    /// # Arguments
    /// * `diag` - The diagnostic to add
    #[inline]
    pub fn diagnose(&mut self, diag: impl Diagnostic + Send + Sync + 'static) {
        let diag = Box::new(diag) as Box<dyn Diagnostic + Send + Sync>;
        self.diags.push(diag)
    }

    /// Adds an error-level diagnostic with the given message.
    ///
    /// # Arguments
    /// * `message` - The error message
    #[inline]
    pub fn error(&mut self, message: impl ToString) {
        let diag = MietteDiagnostic::new(message.to_string()).with_severity(Severity::Error);
        self.diagnose(diag);
    }

    /// Adds a warning-level diagnostic with the given message.
    ///
    /// # Arguments
    /// * `message` - The warning message
    #[inline]
    pub fn warn(&mut self, message: impl ToString) {
        let diag = MietteDiagnostic::new(message.to_string()).with_severity(Severity::Warning);
        self.diagnose(diag);
    }

    /// Adds an informational-level diagnostic with the given message.
    ///
    /// # Arguments
    /// * `message` - The informational message
    #[inline]
    pub fn info(&mut self, message: impl ToString) {
        let diag = MietteDiagnostic::new(message.to_string()).with_severity(Severity::Advice);
        self.diagnose(diag);
    }

    /// Adds a source-specific error diagnostic at the given span.
    ///
    /// # Arguments
    /// * `span` - The location in the source code where the error occurred
    /// * `message` - The error message
    #[inline]
    pub fn source_error(&mut self, span: Span, message: impl ToString) {
        let diag = SourceDiagnostic::error(span, message.to_string());
        self.source_diags.push(diag);
    }

    /// Adds a source-specific warning diagnostic at the given span.
    ///
    /// # Arguments
    /// * `span` - The location in the source code where the warning applies
    /// * `message` - The warning message
    #[inline]
    pub fn source_warn(&mut self, span: Span, message: impl ToString) {
        let diag = SourceDiagnostic::warn(span, message.to_string());
        self.source_diags.push(diag);
    }

    /// Adds a source-specific informational diagnostic at the given span.
    ///
    /// # Arguments
    /// * `span` - The location in the source code where the info applies
    /// * `message` - The informational message
    #[inline]
    pub fn source_info(&mut self, span: Span, message: impl ToString) {
        let diag = SourceDiagnostic::info(span, message.to_string());
        self.source_diags.push(diag);
    }

    /// Adds a pre-constructed source diagnostic.
    ///
    /// # Arguments
    /// * `diag` - The source diagnostic to add
    #[inline]
    pub fn diagnose_source(&mut self, diag: impl Into<SourceDiagnostic>) {
        let diag = diag.into();
        self.source_diags.push(diag);
    }

    /// Adds a source error diagnostic for a specific node by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the node to which the error applies
    /// * `message` - The error message
    #[inline]
    pub fn node_error<T>(&mut self, id: NodeId<T>, message: impl ToString) {
        let span = self.spans.get(id).inner_copied();
        self.source_error(span, message);
    }

    /// Adds a source warning diagnostic for a specific node by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the node to which the warning applies
    /// * `message` - The warning message
    #[inline]
    pub fn node_warn<T>(&mut self, id: NodeId<T>, message: impl ToString) {
        let span = self.spans.get(id).inner_copied();
        self.source_warn(span, message);
    }

    /// Adds a source informational diagnostic for a specific node by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the node to which the info applies
    /// * `message` - The informational message
    #[inline]
    pub fn node_info<T>(&mut self, id: NodeId<T>, message: impl ToString) {
        let span = self.spans.get(id).inner_copied();
        self.source_info(span, message);
    }

    /// Consumes the tracer and returns its contained diagnostics.
    ///
    /// This method is useful when you need to extract the collected diagnostics
    /// from the tracer, for example when passing them to another system or
    /// when you're done with the tracing context.
    ///
    /// # Returns
    /// A tuple containing:
    /// * The source-specific diagnostics
    /// * The general diagnostics
    #[inline]
    pub fn into_inner(
        self,
    ) -> (
        Vec<SourceDiagnostic>,
        Vec<Box<dyn Diagnostic + Send + Sync>>,
    ) {
        (self.source_diags, self.diags)
    }
}

impl fmt::Display for Tracer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Trace:")
    }
}

impl std::error::Error for Tracer {}
