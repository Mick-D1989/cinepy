use crate::{errors, file_types};
use pyo3::PyErr;
use pyo3::prelude::*;
use std::io;
use std::path::Path;

pub struct Video {
    inner: Option<T>,
}

impl Video {
    pub fn open(path: &str) -> PyResult<file_types::CineFile> {
        let ext = Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        match ext {
            "cine" => Ok(file_types::CineFile::new(path)?),
            "mp4" => Ok(todo!()),
            _ => Err(errors::InvalidFileType {
                message: "Invalid File Type. Supported Types are 'cine' and 'mp4'".to_string(),
            })?,
        }
    }
}
