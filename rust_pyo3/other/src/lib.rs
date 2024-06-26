#![allow(non_snake_case)]
#![allow(warnings)]

mod py_type_PyAny;
mod py_type_PyModule;
mod py_types;
mod special_type_Bound;
mod special_type_Py;
mod special_type_Python;
mod special_type_PyRef;
mod special_type_PyErr;

use pyo3::prelude::*;

#[pymodule]
fn other(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(special_type_Python::Python_test1, m)?)?;
    m.add_function(wrap_pyfunction!(special_type_PyErr::throws_test0, m)?)?;
    m.add_function(wrap_pyfunction!(special_type_PyErr::throws_test1, m)?)?;
    Ok(())
}
