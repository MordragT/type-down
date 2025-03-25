use std::{fmt, fs, io, path::Path, sync::Arc};

use miette::SourceCode;
use ropey::Rope;

use crate::Span;

/// Represents a source file with its path, name, and content.
///
/// This struct is used to track source code information for error reporting
/// and other language processing tasks. It implements `SourceCode` to integrate
/// with the `miette` error reporting library.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Source {
    /// The file path, wrapped in an Arc for efficient cloning
    path: Arc<Path>,
    /// The file name, wrapped in an Arc for efficient cloning
    name: Arc<str>,
    /// The source code content, wrapped in an Arc for efficient cloning
    source: Arc<str>,
    /// The source code content represented as a rope data structure
    /// for efficient text manipulation operations
    rope: Rope,
}

impl Source {
    /// Creates a new `Source` from path, name, and source content.
    ///
    /// # Arguments
    /// * `path` - The path to the source file
    /// * `name` - The name of the source file
    /// * `source` - The content of the source file
    ///
    /// # Returns
    /// A new `Source` instance
    pub fn new(path: impl AsRef<Path>, name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        let source = source.as_ref();

        Self {
            path: Arc::from(path.as_ref()),
            name: Arc::from(name.as_ref()),
            source: Arc::from(source),
            rope: Rope::from_str(source),
        }
    }

    /// Creates a new `Source` by reading the contents from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Returns
    /// A `Result` containing either a new `Source` or an IO error
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let name = path.file_name().unwrap().to_string_lossy();
        let source = fs::read_to_string(&path)?;

        Ok(Self::new(&path, name, source))
    }

    /// Returns the name of the source.
    ///
    /// # Returns
    /// The source name as a string slice
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the source content as a string slice.
    ///
    /// # Returns
    /// The source content
    pub fn as_str(&self) -> &str {
        &self.source
    }

    /// Returns the source content as a Rope.
    ///
    /// # Returns
    /// The source content as a Rope
    pub fn as_rope(&self) -> Rope {
        self.rope.clone()
    }

    /// Returns the path of the source.
    ///
    /// # Returns
    /// An Arc-wrapped Path to the source file
    pub fn path(&self) -> Arc<Path> {
        self.path.clone()
    }

    /// Returns the parent directory of the source file.
    ///
    /// # Returns
    /// The parent directory path
    ///
    /// # Panics
    /// Panics if the source has no parent directory
    pub fn work_path(&self) -> &Path {
        self.path.parent().unwrap()
    }

    /// Returns the length of the source content in bytes.
    ///
    /// # Returns
    /// The length of the source content
    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// Creates a `Span` representing the end of the source content.
    ///
    /// # Returns
    /// A `Span` positioned at the end of the source
    pub fn end_of_input(&self) -> Span {
        Span {
            start: self.len(),
            end: self.len(),
            context: (),
        }
    }
}

/// Implementation of `SourceCode` trait for integration with miette error reporting.
impl SourceCode for Source {
    /// Reads a span of the source code with context lines.
    ///
    /// # Arguments
    /// * `span` - The span to read
    /// * `context_lines_before` - Number of context lines to include before the span
    /// * `context_lines_after` - Number of context lines to include after the span
    ///
    /// # Returns
    /// A `Result` containing either the span contents or a miette error
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let inner_contents =
            self.source
                .read_span(span, context_lines_before, context_lines_after)?;

        let contents = miette::MietteSpanContents::new_named(
            self.name.to_string(),
            inner_contents.data(),
            *inner_contents.span(),
            inner_contents.line(),
            inner_contents.column(),
            inner_contents.line_count(),
        );

        Ok(Box::new(contents))
    }
}

/// Implementation of `Display` to allow printing the source content.
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.source)
    }
}
