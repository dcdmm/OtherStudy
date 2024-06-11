use pyo3::exceptions::{PyIndexError, PyTypeError};
use pyo3::prelude::*;

/*
Type Alias pyo3::PyResult

`pub type PyResult<T> = Result<T, PyErr>;`

sents the result of a Python call.
*/

#[pyfunction]
pub fn throws_test0() -> PyResult<()> {
    // Err(PyErr::new::<PyTypeError, _>("Error message"));
    Err(PyTypeError::new_err("Error message")) // 与上等价
}

#[pyfunction]
pub fn throws_test1(mut x: Vec<i32>) -> PyResult<i32> {
    let get_v = x.get(1);
    let get_v_c = get_v.cloned();

    get_v_c.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyIndexError, _>("Index out of bounds"))
}
