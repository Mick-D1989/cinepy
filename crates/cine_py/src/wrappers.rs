use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use cine_core::errors::CineError;
use cine_core::exporters::{EncoderType, FrameData, FrameType};
use cine_core::file::{VideoHeader, VideoOps};

use cine_core::Video;

// // i need to figure out how to wrap this so i don't get hit with the orphan rule
// impl From<CineError> for PyErr {
//     fn from(err: CineError) -> Self {
//         PyValueError::new_err(err.to_string())
//     }
// }

#[pyclass(unsendable)]
pub struct PyVideo {
    inner: Box<dyn VideoOps>,
}

#[pymethods]
impl PyVideo {
    #[new]
    pub fn new(path: &str) -> PyResult<Self> {
        let inner = Video::open(path).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    pub fn get_headers(&self) -> PyResult<(String, u32, u32, u32)> {
        let h: VideoHeader = self.inner.get_headers().unwrap();
        Ok((h.file_name, h.width, h.height, h.frame_count))
    }

    pub fn get_frame_as(&mut self, frame_no: i32, frame_type: String) -> PyResult<Vec<u8>> {
        let ft = match frame_type.as_str() {
            "raw" => FrameType::Raw,
            "png" => FrameType::Png,
            _ => return Err(PyValueError::new_err("Unsupported frame type")),
        };

        match self.inner.get_frame_as(frame_no, ft)? {
            FrameData::U8(v) => Ok(v),
            FrameData::U16(v) => {
                // flatten to bytes for Python
                let bytes: Vec<u8> = v.iter().flat_map(|px| px.to_le_bytes()).collect();
                Ok(bytes)
            }
        }
    }

    pub fn save_frame_as(&self, frame_no: i32, frame_type: String, path: &str) -> PyResult<()> {
        let ft = match frame_type.as_str() {
            "raw" => FrameType::Raw,
            "png" => FrameType::Png,
            _ => return Err(PyValueError::new_err("Unsupported frame type")),
        };
        self.inner.save_frame_as(frame_no, ft, path)?;
        Ok(())
    }
}
