mod signatures;

use pyo3::prelude::*;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
#[pyo3(name = "no_args")] // 修改函数在python中的名称(即调用从func.no_args_py()修改为func.no_args())
fn no_args_py() -> usize {
    42
}

#[pymodule]
fn func(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(no_args_py, m)?)?;
    m.add_function(wrap_pyfunction!(signatures::new, m)?)?;
    Ok(())
}
