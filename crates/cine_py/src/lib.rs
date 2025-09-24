use pyo3::prelude::*;

pub mod cine_wrappers;
pub mod export_wrappers;
pub mod wrappers;

#[pymodule]
fn cine_py(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<wrappers::CinePy>()?;
    m.add_class::<export_wrappers::PyFrameType>()?;
    m.add_class::<export_wrappers::PySaveType>()?;
    m.add_class::<cine_wrappers::PyBitmapInfoHeader>()?;
    m.add_class::<cine_wrappers::PyCineFileHeader>()?;
    m.add_class::<cine_wrappers::PySetup>()?;

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
