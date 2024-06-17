use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass]
/*
Restrictions(参考见: https://pyo3.rs/v0.21.2/class)
    * No lifetime parameters
    * No generic parameters
    * Must be Send
*/
pub struct Number {
    #[pyo3(get, set)]
    inner: i32,
    /*
    The above would make the inner field available for reading and writing as a self.inner Python property.

    Properties can be readonly or writeonly by using just #[pyo3(get)] or #[pyo3(set)] respectively.

    To use these annotations, your field type must implement some conversion traits:
        * For get the field type must implement both IntoPy<PyObject> and Clone.
        * For set the field type must implement FromPyObject.
     */
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

    // For cases which don't satisfy the #[pyo3(get, set)] trait requirements, or need side effects, descriptor methods can be defined in a #[pymethods] impl block. This is done using the #[getter] and #[setter] attributes, like in the example below:
    #[getter]
    // 函数名为: `get_` + 字段名(本例中为`inner`)
    fn get_inner(&self) -> PyResult<i32> {
        Ok(self.inner)
    }

    #[setter]
    // 函数名为: `set_` + 字段名(本例中为`inner`)
    fn set_inner(&mut self, value: i32) -> PyResult<()> {
        self.inner = value;
        Ok(())
    }

    // 实例方法
    fn inner_add(&self, x: i32) -> PyResult<i32> {
        Ok(self.inner + x)
    }

    // 类属性
    #[classattr]
    fn my_attribute() -> String {
        "hello".to_string()
    }

    // If the class attribute is defined with const code only, one can also annotate associated constants:
    #[classattr]
    const MY_CONST_ATTRIBUTE: &'static str = "foobar";

    // 类方法(The first parameter implicitly has type &Bound<'_, PyType>.)
    #[classmethod]
    fn cls_method(cls: &Bound<'_, PyType>) -> PyResult<i32> {
        Ok(10)
    }

    // 静态方法
    #[staticmethod]
    fn static_method(param1: i32, param2: &str) -> PyResult<i32> {
        Ok(100)
    }
}
