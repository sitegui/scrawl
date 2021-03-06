//! # Scrawl
//! A library for opening a file for editing in a text editor and capturing the result as a String
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

/* Standard Library */
use std::path::Path;

/* Internal Modules */
pub mod error;
use error::ScrawlError;

pub mod editor;

/* Convenience functions */
/// New opens an empty text buffer in an editor and returns a Result<String> with the contents.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # fn main() -> Result<(), ScrawlError> {
///     let output = scrawl::new()?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn new() -> Result<String, ScrawlError> {
    editor::new().open()
}

/// With opens a text buffer with the contents of the provided String in an editor. Returns a Result<String> with the edited contents.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # fn main() -> Result<(), ScrawlError> {
///     let output = scrawl::with("Hello World!")?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn with(content: &str) -> Result<String, ScrawlError> {
    editor::new().contents(content).open()
}

/// Open opens a text buffer in an editor with the contents of the file specified. This does **not** edit the contents of the file. Returns a Result<String> with the contents of the buffer.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # use std::path::Path;
///
/// # fn main() -> Result<(), ScrawlError> {
///     let output = scrawl::open("hello.txt")?;
///     println!("{}", output);
///
///     let path = Path::new("website.html");
///     let output = scrawl::open(path)?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn open<P: AsRef<Path>>(path: P) -> Result<String, ScrawlError> {
    editor::new().file(path.as_ref()).open()
}

/// Edit opens a text buffer in an editor with the contents of the file specified. This **does** edit the contents of the file. Returns a Result<String> with the contents of the buffer.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # use std::path::Path;
///
/// # fn main() -> Result<(), ScrawlError> {
///     /* Directly edits the file, no output is returned */
///     scrawl::edit("hello.txt")
/// # }
/// ```
pub fn edit<P: AsRef<Path>>(path: P) -> Result<(), ScrawlError> {
    editor::new().file(path.as_ref()).edit().open()
}
