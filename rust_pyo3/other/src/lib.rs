#![allow(non_snake_case)]
#![allow(warnings)]

mod py_type;

use pyo3::prelude::*;

#[pymodule]
fn other(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    Ok(())
}
