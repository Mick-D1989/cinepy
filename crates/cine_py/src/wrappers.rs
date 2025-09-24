use pyo3::Python;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyIOError};
use pyo3::prelude::*;

use cine_core::Video;
use cine_core::errors::CineError;
use cine_core::exporters::FrameData;
use cine_core::file::{ReturnableHeaders, VideoHeader, VideoOps};
use numpy::PyArray1;

use crate::cine_wrappers;
use crate::export_wrappers::{PyFrameType, PySaveType};

// Define a base Python exception for your crate
create_exception!(cinepy, PyCineError, PyException);
create_exception!(cinepy, PyConversionError, PyCineError);
create_exception!(cinepy, PyUnsupportedError, PyCineError);
create_exception!(cinepy, PyEncodingError, PyCineError);
create_exception!(cinepy, PyHeaderError, PyCineError);

pub struct PyCineErr(pub CineError);

impl From<PyCineErr> for PyErr {
    fn from(err: PyCineErr) -> PyErr {
        match err.0 {
            CineError::Conversion(e) => PyConversionError::new_err(e.to_string()),
            CineError::Unsupported(e) => PyUnsupportedError::new_err(e.to_string()),
            CineError::IoError(e) => PyIOError::new_err(e.to_string()),
            CineError::Encoding(e) => PyEncodingError::new_err(e.to_string()),
            CineError::Header(e) => PyHeaderError::new_err(e.to_string()),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum PyVideoHeader {
    BitmapInfoHeader,
    CineFileHeader,
    Setup,
}

impl From<PyVideoHeader> for VideoHeader {
    fn from(val: PyVideoHeader) -> Self {
        match val {
            PyVideoHeader::BitmapInfoHeader => VideoHeader::BitmapInfoHeader,
            PyVideoHeader::CineFileHeader => VideoHeader::CineFileHeader,
            PyVideoHeader::Setup => VideoHeader::Setup,
        }
    }
}

impl From<VideoHeader> for PyVideoHeader {
    fn from(val: VideoHeader) -> Self {
        match val {
            VideoHeader::BitmapInfoHeader => PyVideoHeader::BitmapInfoHeader,
            VideoHeader::CineFileHeader => PyVideoHeader::CineFileHeader,
            VideoHeader::Setup => PyVideoHeader::Setup,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum PyReturnableHeaders {
    CineFileHeader(cine_wrappers::PyCineFileHeader),
    BitmapInfoHeader(cine_wrappers::PyBitmapInfoHeader),
    Setup(cine_wrappers::PySetup),
}

impl From<ReturnableHeaders> for PyReturnableHeaders {
    fn from(val: ReturnableHeaders) -> Self {
        match val {
            ReturnableHeaders::CineFileHeader(header) => {
                PyReturnableHeaders::CineFileHeader(header.into())
            }
            ReturnableHeaders::BitmapInfoHeader(header) => {
                PyReturnableHeaders::BitmapInfoHeader(header.into())
            }
            ReturnableHeaders::Setup(header) => PyReturnableHeaders::Setup(header.into()),
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

    pub fn get_headers(&self, header: PyVideoHeader) -> PyResult<PyReturnableHeaders> {
        let return_header = self.inner.get_headers(header.into()).map_err(PyCineErr)?;
        Ok(return_header.into())
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
