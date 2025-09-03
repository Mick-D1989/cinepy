use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(Debug)]
pub struct InvalidFileType {
    pub message: String,
}

impl From<InvalidFileType> for PyErr {
    fn from(err: InvalidFileType) -> Self {
        PyValueError::new_err(format!("{}", err.message))
    }
}
