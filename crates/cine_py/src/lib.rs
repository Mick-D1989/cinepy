use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// Define a base Python exception for your crate
create_exception!(cine_py, PyCineError, PyException);

pub mod wrappers;

#[pymodule]
fn cine_py(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<wrappers::CinePy>()?;
    m.add_class::<wrappers::PyFrameType>()?;

    m.add("PyCineError", py.get_type::<wrappers::PyCineError>())?;
    m.add(
        "PyConversionError",
        py.get_type::<wrappers::PyConversionError>(),
    )?;
    m.add(
        "PyUnsupportedError",
        py.get_type::<wrappers::PyUnsupportedError>(),
    )?;
    Ok(())
}
