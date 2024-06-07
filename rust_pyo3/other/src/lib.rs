#![allow(non_snake_case)]
#![allow(warnings)]

mod py_types;
mod py_type_PyAny;
mod py_type_PyModule;
mod special_type_Python;
mod special_type_Py;

use pyo3::prelude::*;

#[pymodule]
fn other(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(special_type_Python::Python_test1, m)?)?;
    Ok(())
}