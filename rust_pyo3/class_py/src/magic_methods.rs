use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass]
pub struct SimpleIterator {
    count: i32,
    limit: i32,
}

#[pymethods]
impl SimpleIterator {
    #[new]
    fn new(limit: i32) -> Self {
        SimpleIterator { count: 0, limit }
    }

    /*
    When PyO3 handles a magic method, a couple of changes apply compared to other #[pymethods]:
        * The Rust function signature is restricted to match the magic method.
        * The #[pyo3(signature = (...)] and #[pyo3(text_signature = "...")] attributes are not allowed.
     */
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<i32> {
        if slf.count < slf.limit {
            let val = slf.count;
            slf.count += 1;
            Some(val)
        } else {
            None
        }
    }
}
