pub mod cine;
pub mod conversions;
pub mod decompress;
pub mod errors;
pub mod exporters;
pub mod file;

use std::io::Result;
use std::path::Path;

use crate::errors::FileTypeError;

pub struct Video;

impl Video {
    pub fn open(path: &str) -> errors::CineResult<Box<dyn file::VideoOps>> {
        let ext = Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        match ext {
            "cine" => {
                let x = file::CineFile::open(path).unwrap();
                Ok(Box::new(x))
            }
            "mp4" => {
                let x = file::Mp4File::open(path).unwrap();
                Ok(Box::new(x))
            }
            _ => Err(errors::CineError::Unsupported(FileTypeError {
                file_type: ext.to_string(),
            })),
        }
    }
}
