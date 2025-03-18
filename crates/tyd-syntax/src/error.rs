use std::fmt;

use chumsky::error::Rich;
use miette::{Diagnostic, LabeledSpan, Severity};

use crate::{Span, Spanned};

pub type SourceResult<T> = Result<T, Vec<SourceDiagnostic>>;

#[derive(Debug, Clone)]
pub struct SourceDiagnostic {
    pub message: String,
    pub help: Option<String>,
    pub severity: Severity,
    pub span: Span,
    pub trace: Vec<Spanned<String>>,
}

impl SourceDiagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            help: None,
            severity: Severity::Error,
            span,
            trace: Vec::new(),
        }
    }

    pub fn warn(span: Span, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            help: None,
            severity: Severity::Warning,
            span,
            trace: Vec::new(),
        }
    }

    pub fn info(span: Span, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            help: None,
            severity: Severity::Advice,
            span,
            trace: Vec::new(),
        }
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn set_help(&mut self, help: impl Into<String>) -> &mut Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_trace(mut self, trace: impl IntoIterator<Item = Spanned<String>>) -> Self {
        self.trace = trace.into_iter().collect();
        self
    }

    pub fn set_trace(&mut self, trace: impl IntoIterator<Item = Spanned<String>>) -> &mut Self {
        self.trace = trace.into_iter().collect();
        self
    }

    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }

    pub fn is_warning(&self) -> bool {
        self.severity == Severity::Warning
    }

    pub fn is_info(&self) -> bool {
        self.severity == Severity::Advice
    }
}

impl<'src> From<Rich<'src, char>> for SourceDiagnostic {
    fn from(e: Rich<'src, char>) -> Self {
        let message = e.to_string();
        let span = *e.span();
        let trace = e
            .contexts()
            .map(|(label, span)| (label.to_string(), *span))
            .collect();

        Self {
            message,
            help: None,
            severity: Severity::Error,
            span,
            trace,
        }
    }
}

impl fmt::Display for SourceDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for SourceDiagnostic {}

impl Diagnostic for SourceDiagnostic {
    fn severity(&self) -> Option<Severity> {
        Some(self.severity)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.help
            .as_ref()
            .map(|advice| Box::new(advice) as Box<dyn fmt::Display>)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let primary =
            LabeledSpan::new_primary_with_span(Some(self.message.clone()), self.span.into_range());

        let trace = self.trace.iter().map(|(label, span)| {
            LabeledSpan::new_with_span(Some(label.clone()), span.into_range())
        });

        let iter = Box::new(std::iter::once(primary).chain(trace))
            as Box<dyn Iterator<Item = LabeledSpan>>;

        Some(iter)
    }
}
