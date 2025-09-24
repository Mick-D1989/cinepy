use pyo3::prelude::*;

use cine_core::exporters::FrameType;
use cine_core::exporters::SaveType;

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
