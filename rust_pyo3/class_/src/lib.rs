#![allow(non_snake_case)]
#![allow(warnings)]

use pyo3::prelude::*;

#[pymodule]
fn class_(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    Ok(())
}
