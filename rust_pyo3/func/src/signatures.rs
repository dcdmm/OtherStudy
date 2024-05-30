use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (num=-1))]
pub fn new(num: i32) -> i32 {
    num
}
