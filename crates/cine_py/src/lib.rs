use pyo3::prelude::*;

mod wrappers; // put PyVideo in a new file

#[pymodule]
fn cine_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<wrappers::PyVideo>()?;
    Ok(())
}
