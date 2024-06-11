// 参考:GitHubProjects\OtherStudy\cpp_pybind11\type_conversions\bind_stl.cpp

use pyo3::prelude::*;

#[pyclass]
pub struct VecWrapper {
    #[pyo3(get, set)]
    vec: Vec<i32>,
}

#[pymethods]
impl VecWrapper {
    #[new]
    fn new() -> Self {
        VecWrapper { vec: vec![] }
    }

    fn append(&mut self, val: i32) {
        self.vec.push(val);
    }

    fn pop(&mut self) -> PyResult<i32> {
        let pop_v = self.vec.pop();
        pop_v.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyIndexError, _>("pop from empty list"))
    }

    fn get(&self, index: usize) -> PyResult<i32> {
        let get_v = self.vec.get(index);
        let get_v_c = get_v.cloned();

        get_v_c
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyIndexError, _>("Index out of bounds"))
    }
}
