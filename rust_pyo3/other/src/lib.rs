#![allow(non_snake_case)]
#![allow(warnings)]

mod py_types;
mod py_type_PyAny;
mod py_type_PyModule;
mod special_type;

use pyo3::prelude::*;

#[pymodule]
fn other(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    Ok(())
}