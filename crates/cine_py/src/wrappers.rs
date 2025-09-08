use cine_core::exporters::SaveType;
use pyo3::Python;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyIOError};
use pyo3::prelude::*;

use numpy::PyArray1;

use cine_core::Video;
use cine_core::errors::CineError;
use cine_core::exporters::{FrameData, FrameType};
use cine_core::file::{VideoHeader, VideoOps};

// Define a base Python exception for your crate
create_exception!(cinepy, PyCineError, PyException);
create_exception!(cinepy, PyConversionError, PyCineError);
create_exception!(cinepy, PyUnsupportedError, PyCineError);
create_exception!(cinepy, PyEncodingError, PyCineError);

pub struct PyCineErr(pub CineError);

impl From<PyCineErr> for PyErr {
    fn from(err: PyCineErr) -> PyErr {
        match err.0 {
            CineError::Conversion(e) => PyConversionError::new_err(e.to_string()),
            CineError::Unsupported(e) => PyUnsupportedError::new_err(e.to_string()),
            CineError::IoError(e) => PyIOError::new_err(e.to_string()),
            CineError::Encoding(e) => PyEncodingError::new_err(e.to_string()),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum PyFrameType {
    Base64,
    Bytes,
    Png,
    Raw,
}

impl From<PyFrameType> for FrameType {
    fn from(val: PyFrameType) -> Self {
        match val {
            PyFrameType::Base64 => FrameType::Base64,
            PyFrameType::Bytes => FrameType::Bytes,
            PyFrameType::Png => FrameType::Png,
            PyFrameType::Raw => FrameType::Raw,
        }
    }
}

impl From<FrameType> for PyFrameType {
    fn from(val: FrameType) -> Self {
        match val {
            FrameType::Base64 => PyFrameType::Base64,
            FrameType::Bytes => PyFrameType::Bytes,
            FrameType::Png => PyFrameType::Png,
            FrameType::Raw => PyFrameType::Raw,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum PySaveType {
    Jpeg,
    Mp4,
    Png,
}

impl From<PySaveType> for SaveType {
    fn from(val: PySaveType) -> Self {
        match val {
            PySaveType::Jpeg => SaveType::Jpeg,
            PySaveType::Mp4 => SaveType::Mp4,
            PySaveType::Png => SaveType::Png,
        }
    }
}

impl From<SaveType> for PySaveType {
    fn from(val: SaveType) -> Self {
        match val {
            SaveType::Jpeg => PySaveType::Jpeg,
            SaveType::Mp4 => PySaveType::Mp4,
            SaveType::Png => PySaveType::Png,
        }
    }
}

#[pyclass(unsendable)]
pub struct CinePy {
    inner: Box<dyn VideoOps>,
}

#[pymethods]
impl CinePy {
    #[new]
    pub fn new(path: &str) -> PyResult<Self> {
        let inner = Video::open(path).map_err(PyCineErr)?;
        Ok(Self { inner })
    }

    pub fn get_headers(&self) -> PyResult<(String, u32, u32, u32)> {
        let h: VideoHeader = self.inner.get_headers().map_err(PyCineErr)?;
        Ok((h.file_name, h.width, h.height, h.frame_count))
    }

    pub fn get_frame_as(&mut self, frame_no: i32, frame_type: PyFrameType) -> PyResult<PyObject> {
        let ft = frame_type.into();

        let frame = self.inner.get_frame_as(frame_no, ft).map_err(PyCineErr)?;
        // Returns all types as a numpy array in python.
        Python::with_gil(|py| match frame {
            FrameData::Base64(v) => {
                let arr = PyArray1::from_vec(py, v.into_bytes());
                Ok(arr.into())
            }
            FrameData::Bytes(v) => {
                let arr = PyArray1::from_vec(py, v);
                Ok(arr.into())
            }
            FrameData::Png(v) => {
                let arr = PyArray1::from_vec(py, v);
                Ok(arr.into())
            }
            FrameData::Raw(v) => {
                let arr = PyArray1::from_vec(py, v);
                Ok(arr.into())
            }
        })
    }

    pub fn save_frame_as(
        &mut self,
        frame_no: i32,
        save_type: PySaveType,
        path: &str,
    ) -> PyResult<()> {
        let st = save_type.into();
        self.inner
            .save_frame_as(frame_no, st, path)
            .map_err(PyCineErr)?;
        Ok(())
    }
}
