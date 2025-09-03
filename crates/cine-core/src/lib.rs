pub mod cine;
pub mod conversions;
pub mod decompress;
pub mod errors;
pub mod exporters;
pub mod file;

use std::io::Result;
use std::path::Path;

pub struct Video;

impl Video {
    pub fn open(path: &str) -> Result<Box<dyn file::VideoOps>> {
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
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported file type",
            )),
        }
    }
}
