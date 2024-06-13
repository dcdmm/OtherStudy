use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};

#[pyclass]
pub struct MyClass {
    #[pyo3(get, set)]
    num: i32,
}

// Similar to #[pyfunction], the #[pyo3(signature = (...))] attribute can be used to specify the way that #[pymethods] accept arguments.
#[pymethods]
impl MyClass {
    #[new]
    #[pyo3(signature = (num=-1))]
    fn new(num: i32) -> Self {
        MyClass { num }
    }

    #[pyo3(signature = (num=10, name="Hello", *py_args,  **py_kwargs))]
    fn method(
        &mut self,
        num: i32,
        name: &str,
        py_args: &Bound<'_, PyTuple>,
        py_kwargs: Option<&Bound<'_, PyDict>>,
    ) -> String {
        let num_before = self.num;
        self.num = num;
        format!(
            "num={} (was previously={}), py_args={:?}, name={}, py_kwargs={:?} ",
            num, num_before, py_args, name, py_kwargs,
        )
    }

    #[classmethod]
    // The #[pyo3(text_signature = "...") option for #[pyfunction] also works for #[pymethods].
    #[pyo3(text_signature = "(cls, e_int, f_int)")]
    fn my_class_method(cls: &Bound<'_, PyType>, e: i32, f: i32) -> i32 {
        e + f
    }
    #[staticmethod]
    #[pyo3(text_signature = "(e_int, f_int)")]
    fn my_static_method(e: i32, f: i32) -> i32 {
        e + f
    }
}
