#![allow(non_snake_case)]
#![allow(warnings)]

mod signature;
mod text_signature;

use pyo3::prelude::*;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
#[pyo3(name = "no_args")] // 修改python调用名称从no_args_py(默认)为no_args
fn no_args_py() -> usize {
    42
}

#[pymodule]
fn func(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(no_args_py, m)?)?;

    m.add_class::<text_signature::MyStruct>()?;

    m.add_function(wrap_pyfunction!(signature::default_para, m)?)?;
    m.add_function(wrap_pyfunction!(signature::args_para, m)?)?;
    m.add_function(wrap_pyfunction!(signature::kwargs_prara, m)?)?;

    Ok(())
}
