use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

#[pyfunction]
#[pyo3(signature = (num0, num1=-1))] // 默认参数num1=-1
pub fn default_para(num0: i32, num1: i32) -> i32 {
    num0 + num1
}

#[pyfunction]
#[pyo3(signature = (num0, *py_args))]
pub fn args_para(num0: i32, py_args: &Bound<'_, PyTuple>) {

}

#[pyfunction]
#[pyo3(signature = (num0, *py_kwargs))]
pub fn kwargs(num0: i32, py_kwargs: Option<&Bound<'_, PyDict>>) {

}
