use crate::file::VideoHeader;
use image::ImageError;
use std::error::Error;
use std::fmt;
use std::result::Result;

// Define custom Result type
pub type CineResult<T> = Result<T, CineError>;

#[derive(Debug)]
pub enum CineError {
    Conversion(ConversionError),
    Unsupported(FileTypeError),
    Header(HeaderAccessError),
    IoError(std::io::Error),
    Encoding(image::ImageError),
}

#[derive(Debug)]
pub struct ConversionError {
    pub fail_type: String,
    pub source: Box<dyn Error + Send + Sync>,
}

#[derive(Debug)]
pub struct FileTypeError {
    pub file_type: String,
}

#[derive(Debug)]
pub struct HeaderAccessError {
    pub bad_header: VideoHeader,
}

// --- Implementations for ConversionError ---
impl ConversionError {
    // The `new` function now stores the source error
    pub fn new(fail_type: impl ToString, err: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        ConversionError {
            fail_type: fail_type.to_string(),
            source: err.into(),
        }
    }

    pub fn from_string(fail_type: impl ToString, message: impl ToString) -> Self {
        ConversionError {
            fail_type: fail_type.to_string(),
            source: message.to_string().into(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to convert file type: {}", self.fail_type)
    }
}

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// --- Implementations for FileTypeError ---

impl fmt::Display for FileTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupported file type: {}", self.file_type)
    }
}

impl fmt::Display for HeaderAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Attempting to access header which doesn't exist: {}",
            self.bad_header
        )
    }
}

impl fmt::Display for VideoHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VideoHeader::BitmapInfoHeader => write!(f, "BitmapInfoHeader"),
            VideoHeader::CineFileHeader => write!(f, "CineFileHeader"),
            VideoHeader::Setup => write!(f, "Setup"),
        }
    }
}

// Implement the Error trait. This error is a root cause, so `source()` returns None.
impl Error for FileTypeError {}
impl Error for HeaderAccessError {}

// --- Implementations for the main CineError enum ---
// This allows you to use the `?` operator on functions to propegate the error up the stack
impl From<std::io::Error> for CineError {
    fn from(err: std::io::Error) -> CineError {
        CineError::IoError(err)
    }
}

impl From<ImageError> for CineError {
    fn from(err: ImageError) -> CineError {
        CineError::Encoding(err)
    }
}

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

impl From<HeaderAccessError> for CineError {
    fn from(err: HeaderAccessError) -> CineError {
        CineError::Header(err)
    }
}

impl fmt::Display for CineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CineError::Conversion(err) => err.fmt(f),
            CineError::IoError(err) => err.fmt(f),
            CineError::Unsupported(err) => err.fmt(f),
            CineError::Encoding(err) => err.fmt(f),
            CineError::Header(err) => err.fmt(f),
        }
    }
}

impl Error for CineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CineError::Conversion(err) => Some(err),
            CineError::IoError(err) => Some(err),
            CineError::Unsupported(err) => Some(err),
            CineError::Encoding(err) => Some(err),
            CineError::Header(err) => Some(err),
        }
    }
}
