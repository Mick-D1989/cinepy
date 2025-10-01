pub mod cine;
pub mod conversions;
pub mod decompress;
pub mod errors;
pub mod exporters;
pub mod file;
pub mod utils;
use crate::errors::FileTypeError;
use std::path::Path;

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

#[cfg(test)]
mod tests {

    #[test]
    fn test_open() {
        let path = "../../files/uncompressed_gray.cine";
        let mut video_file =
            crate::Video::open(path).expect("Failed to open video file for benchmarking");
    }

    #[test]
    fn test_get_png() {
        let path = "../../files/uncompressed_gray.cine";
        let mut video_file =
            crate::Video::open(path).expect("Failed to open video file for benchmarking");
        let png = video_file.get_frame_as(0, crate::exporters::FrameType::Png);
    }
}
