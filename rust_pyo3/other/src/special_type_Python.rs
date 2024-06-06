/*
marker token that represents holding the GIL.

It serves three main purposes:
    * It provides a global API for the Python interpreter, such as Python::eval_bound.
    * It can be passed to functions that require a proof of holding the GIL, such as Py::clone_ref.
    * Its lifetime represents the scope of holding the GIL which can be used to create Rust references that are bound to it, such as &PyAny.

The following are the recommended ways to obtain a Python token, in order of preference:
    * In a function or method annotated with #[pyfunction] or #[pymethods] you can declare it as a parameter, and PyO3 will pass in the token when Python code calls it.
    * If you already have something with a lifetime bound to the GIL, such as &PyAny, you can use its .py() method to get a token.
    * When you need to acquire the GIL yourself, such as when calling Python code from Rust, you should call Python::with_gil to do that and pass your code as a closure to it.
*/

use pyo3::prelude::*;
use pyo3::types::*;

#[test]
fn Python_test0() {
    /*
    pub fn with_gil<F, R>(f: F) -> R
    where
        F: for<'py> FnOnce(Python<'py>) -> R,

    Acquires the global interpreter lock, allowing access to the Python interpreter. The provided closure F will be executed with the acquired Python marker token.

    If implementing #[pymethods] or #[pyfunction], declare py: Python as an argument. PyO3 will pass in the token to grant access to the GIL context in which the function is running, avoiding the need to call with_gil.
     */

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let s = PyString::new_bound(py, "hello world");
    })
}

#[pyfunction]
pub fn Python_test1(py: Python<'_>, number: f64) -> PyResult<(f64)> {
    let math = PyModule::import(py, "math")?; // 导入python math模块
    let sqrt_func = math.getattr("sqrt")?; //获取math模块中的sqrt函数
    let result = sqrt_func.call1((number,))?; // 调用sqrt函数
    let result_f64: f64 = result.extract()?;
    Ok(result_f64)
}
