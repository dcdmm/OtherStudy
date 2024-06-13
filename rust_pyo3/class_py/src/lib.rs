#![allow(non_snake_case)]
#![allow(warnings)]

mod base;

use pyo3::prelude::*;

#[pymodule]
fn class_py(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<base::Number>()?;
    m.add_class::<base::Nonzero>()?;
    Ok(())
}
