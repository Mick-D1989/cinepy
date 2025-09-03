use std::error::Error;
use std::fmt;
use std::result::Result; // Use the full path to avoid ambiguity

// NOTE: Removed `Clone`, `Hash`, `PartialEq` because `std::io::Error` doesn't support them.
#[derive(Debug)]
pub enum CineError {
    Conversion(ConversionError),
    Unsupported(FileTypeError),
    IoError(std::io::Error),
}

// Struct to hold conversion-specific errors
#[derive(Debug)]
pub struct ConversionError {
    pub file_type: String,
    // Add a source field to store the underlying error
    pub source: Box<dyn Error + Send + Sync>,
}

// Struct to hold file type-specific errors
#[derive(Debug)]
pub struct FileTypeError {
    pub file_type: String,
}

// --- Implementations for ConversionError ---

impl ConversionError {
    // The `new` function now stores the source error
    pub fn new(file_type: impl ToString, err: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        ConversionError {
            file_type: file_type.to_string(),
            source: err.into(),
        }
    }
}

// User-friendly display message
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to convert file type: {}", self.file_type)
    }
}

// Implement the Error trait to allow for error chaining
impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// --- Implementations for FileTypeError ---

// User-friendly display message
impl fmt::Display for FileTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupported file type: {}", self.file_type)
    }
}

// Implement the Error trait. This error is a root cause, so `source()` returns None.
impl Error for FileTypeError {}

// --- Implementations for the main CineError enum ---

// This allows you to use the `?` operator on functions returning `std::io::Result`.
impl From<std::io::Error> for CineError {
    fn from(err: std::io::Error) -> CineError {
        CineError::IoError(err)
    }
}

// Now we can also automatically convert our custom error structs into the enum
impl From<ConversionError> for CineError {
    fn from(err: ConversionError) -> CineError {
        CineError::Conversion(err)
    }
}

impl From<FileTypeError> for CineError {
    fn from(err: FileTypeError) -> CineError {
        CineError::Unsupported(err)
    }
}

// Define our custom Result type
pub type CineResult<T> = Result<T, CineError>;

// The Display implementation now works because all inner types implement Display.
impl fmt::Display for CineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CineError::Conversion(err) => err.fmt(f),
            CineError::IoError(err) => err.fmt(f),
            CineError::Unsupported(err) => err.fmt(f),
        }
    }
}

// The Error implementation now works because all inner types implement Error.
impl Error for CineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CineError::Conversion(err) => Some(err), // err.source() is called implicitly
            CineError::IoError(err) => Some(err),
            CineError::Unsupported(err) => Some(err),
        }
    }
}
