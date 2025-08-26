use pyo3::prelude::*;
pub mod cine;
pub mod conversions;
pub mod file;
pub mod lut;
pub mod decompress;

// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn cine_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<file::CineFile>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
