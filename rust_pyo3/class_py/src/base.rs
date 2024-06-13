use pyo3::exceptions::*;
use pyo3::prelude::*;

#[pyclass]
pub struct Number {
    /*
    Properties can be readonly or writeonly by using just #[pyo3(get)] or #[pyo3(set)] respectively.

    To use these annotations, your field type must implement some conversion traits:
        * For get the field type must implement both IntoPy<PyObject> and Clone.
        * For set the field type must implement FromPyObject.
     */
    #[pyo3(get, set)]
    inner: i32,
    // The above would make the num field available for reading and writing as a self.inner Python property.
}

#[pymethods]
impl Number {
    // 构造函数
    #[new]
    fn new(value: i32) -> Self {
        Number { inner: value }
    }
}

#[pyclass]
pub struct Nonzero {
    inner: i32,
}

#[pymethods]
impl Nonzero {
    #[new]
    fn py_new(value: i32) -> PyResult<Self> {
        if value == 0 {
            Err(PyValueError::new_err("cannot be zero"))
        } else {
            Ok(Nonzero { inner: value })
        }
    }

    // For cases which don't satisfy the #[pyo3(get, set)] trait requirements, or need side effects, descriptor methods can be defined in a #[pymethods] impl block.
    #[getter]
    fn get_inner(&self) -> PyResult<i32> {
        Ok(self.inner)
    }

    #[setter]
    fn set_inner(&mut self, value: i32) -> PyResult<()> {
        self.inner = value;
        Ok(())
    }
}
