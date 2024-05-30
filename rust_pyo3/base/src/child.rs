use pyo3::prelude::*;

#[pyfunction]
pub fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
pub fn triple(x: usize) -> usize {
    x * 3
}

#[pymodule]
pub fn child(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    Ok(())
}
